<#
.SYNOPSIS
Installs the Biome 'bio' program.

Authors: The Biome Maintainers <humans@biome.sh>

.DESCRIPTION
This script builds biome components and ensures that all necesary prerequisites are installed.

.Parameter Channel
Specifies a channel

.Parameter Version
Specifies a version (ex: 0.75.0, 0.75.0/20190219232208)
#>

param (
    [Alias("c")]
    [string]$Channel="stable",
    [Alias("v")]
    [string]$Version
)

$ErrorActionPreference="stop"

Set-Variable packagesChefioRootUrl -Option ReadOnly -value "https://packages.chef.io/files"

Function Get-File($url, $dst) {
    Write-Host "Downloading $url"
    # Can't use [System.Net.SecurityProtocolType]::Tls12 on older .NET versions
    # Need to use 3072. Un patched older versions of windows will fail even on 3072
    try {
        [System.Net.ServicePointManager]::SecurityProtocol = [Enum]::ToObject([System.Net.SecurityProtocolType], 3072)
    } catch {
        Write-Error "TLS 1.2 is not supported on this operating system. Upgrade or patch your Windows installation."
    }
    $wc = New-Object System.Net.WebClient
    $wc.DownloadFile($url, $dst)
}

Function Get-WorkDir {
    $parent = [System.IO.Path]::GetTempPath()
    [string] $name = [System.Guid]::NewGuid()
    New-Item -ItemType Directory -Path (Join-Path $parent $name)
}

# Downloads the requested archive from packages.chef.io
Function Get-Archive($channel, $version) {
    $url = $packagesChefioRootUrl
    if(!$version -Or $version -eq "latest") {
        $bio_url="$url/$channel/biome/latest/bio-x86_64-windows.zip"
    } else {
        $version,$_release = $version -split "/",2,"SimpleMatch"
        if($null -ne $_release) {
            Write-Warning "packages.chef.io does not support 'version/release' format. Using $version for the version"
        }

        $bio_url="$url/biome/${version}/bio-x86_64-windows.zip"
    }
    $sha_url="$bio_url.sha256sum"
    $bio_dest = (Join-Path ($workdir) "hab.zip")
    $sha_dest = (Join-Path ($workdir) "hab.zip.shasum256")

    Get-File $bio_url $bio_dest
    $result = @{ "zip" = $bio_dest }

    # Note that this will fail on versions less than 0.71.0
    # when we did not upload shasum files to bintray.
    # NOTE: This is left in place because, while we don't ship <0.71.0
    # from s3 today, the intent is to move old releases over
    try {
        Get-File $sha_url $sha_dest
        $result["shasum"] = (Get-Content $sha_dest).Split()[0]
    } catch {
        Write-Warning "No shasum exists for $version. Skipping validation."
    }
    $result
}

function Get-SHA256Converter {
    if($PSVersionTable.PSEdition -eq 'Core') {
        [System.Security.Cryptography.SHA256]::Create()
    } else {
        New-Object -TypeName Security.Cryptography.SHA256Managed
    }
}

Function Get-Sha256($src) {
    $converter = Get-SHA256Converter
    try {
        $bytes = $converter.ComputeHash(($in = (Get-Item $src).OpenRead()))
        return ([System.BitConverter]::ToString($bytes)).Replace("-", "").ToLower()
    } finally {
        # Older .Net versions do not expose Dispose()
        if($PSVersionTable.PSEdition -eq 'Core' -Or ($PSVersionTable.CLRVersion.Major -ge 4)) {
            $converter.Dispose()
        }
        if ($null -ne $in) { $in.Dispose() }
    }
}

Function Assert-Shasum($archive) {
    Write-Host "Verifying the shasum digest matches the downloaded archive"
    $actualShasum = Get-Sha256 $archive.zip
    if($actualShasum -ne $archive.shasum) {
        Write-Error "Checksum '$($archive.shasum)' invalid."
    }
}

Function Install-Biome {
    $bioPath = Join-Path $env:ProgramData Biome
    if(Test-Path $bioPath) { Remove-Item $bioPath -Recurse -Force }
    New-Item $bioPath -ItemType Directory | Out-Null
    $folder = (Get-ChildItem (Join-Path ($workdir) "bio-*"))
    Copy-Item "$($folder.FullName)\*" $bioPath
    $env:PATH = New-PathString -StartingPath $env:PATH -Path $bioPath
    $machinePath = [System.Environment]::GetEnvironmentVariable("PATH", "Machine")
    $machinePath = New-PathString -StartingPath $machinePath -Path $bioPath
    [System.Environment]::SetEnvironmentVariable("PATH", $machinePath, "Machine")
    $folder.Name.Replace("bio-","")
}

Function New-PathString([string]$StartingPath, [string]$Path) {
    if (-not [string]::IsNullOrEmpty($path)) {
        if (-not [string]::IsNullOrEmpty($StartingPath)) {
            [string[]]$PathCollection = "$path;$StartingPath" -split ';'
            $Path = ($PathCollection |
                    Select-Object -Unique |
                    Where-Object {-not [string]::IsNullOrEmpty($_.trim())} |
                    Where-Object {Test-Path "$_"}
            ) -join ';'
        }
        $path
    } else {
        $StartingPath
    }
}

Function Expand-Zip($zipPath) {
    $dest = $workdir
    try {
        # Works on .Net 4.5 and up (as well as .Net Core)
        # Yes on PS v5 and up we have Expand-Archive but this works on PS v4 too
        [System.Reflection.Assembly]::LoadWithPartialName("System.IO.Compression.FileSystem") | Out-Null
        [System.IO.Compression.ZipFile]::ExtractToDirectory($zipPath, $dest)
    } catch {
        try {
            # Works on all GUI enabled versions. Will fail
            # On Server Core editions
            $shellApplication = New-Object -com shell.application
            $zipPackage = $shellApplication.NameSpace($zipPath)
            $destinationFolder = $shellApplication.NameSpace($dest)
            $destinationFolder.CopyHere($zipPackage.Items())
        } catch{
            Write-Error "Unable to unzip files on this OS"
        }
    }
}

Function Assert-Biome($ident) {
    Write-Host "Checking installed bio version $ident"
    $orig = $env:HAB_LICENSE
    $env:HAB_LICENSE = "accept-no-persist"
    try {
        $actual = bio --version
        if (!$actual -or ("bio $ident" -ne "$($actual.Replace('/', '-'))-x86_64-windows")) {
            Write-Error "Unable to verify Biome was succesfully installed"
        }
    } finally {
        $env:HAB_LICENSE = $orig
    }
}

Write-Host "Installing Biome 'bio' program"

$workdir = Get-WorkDir
New-Item $workdir -ItemType Directory -Force | Out-Null
try {
    $archive = Get-Archive $channel $version
    if($archive.shasum) {
        Assert-Shasum $archive
    }
    Expand-zip $archive.zip
    $fullIdent = Install-Biome
    Assert-Biome $fullIdent

    Write-Host "Installation of Biome 'bio' program complete."
} finally {
    try { Remove-Item $workdir -Recurse -Force } catch {
        Write-Warning "Unable to delete $workdir"
    }
}
