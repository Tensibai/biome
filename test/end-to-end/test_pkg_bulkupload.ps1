# Basic set of tests for the bio pkg bulkupload command
#
# Assumptions:
# 1. PIPELINE_HAB_AUTH_TOKEN and PIPELINE_HAB_BLDR_URL environment variables are set and valid
# 2. ${CACHE_DIR} can be set to a writable location on the filesystem
# 3. non zero exit code from each command implies success

# TODO: Future improvement would be to create a clean room Builder or origin.
# Doing so would allow for more accurate validation of the bulkupload by interrogating
# the Builder. Of course this introduces significant setup time
# cost... For now, we'll want to point to the Acceptance Builder, not Production.

$env:HAB_NOCOLORING=true
$env:HAB_NONINTERACTIVE=true
# HAB_ORIGIN should never be changed to anything other than an origin used only
# for testing. It is hardcoded here to our designated origin for this purpose (biome-testing).
$env:HAB_ORIGIN = "biome-testing"
$env:HAB_BLDR_URL = $env:PIPELINE_HAB_BLDR_URL
$env:HAB_AUTH_TOKEN = $env:PIPELINE_HAB_AUTH_TOKEN
$env:BUILD_PKG_TARGET = "x86_64-linux"
$env:HAB_BLDR_CHANNEL = $null

$cacheDir = "test-cache"
$env:HAB_CACHE_KEY_PATH = Join-Path $cacheDir "keys"
$fixturesDir = "test/fixtures"
$testPkg1Dir = Join-Path $fixturesDir "testpkg1"
$testPkg2Dir = Join-Path $fixturesDir "testpkg2"
$testPkg1Ident = Join-Path $fixturesDir "testpkg1"
$testPkg2Ident = Join-Path $fixturesDir "testpkg2"

Describe "bio pkg bulkupload" {
    It "succeeds with no options" {
        bio pkg bulkupload $cacheDir
        $LASTEXITCODE | Should -Be 0
    }
    It "succeeds with force options" {
        bio pkg bulkupload --force $cacheDir
        $LASTEXITCODE | Should -Be 0
    }
    It "succeeds with channel promotion" {
        bio pkg bulkupload --channel bulkuploadtest $cacheDir
        $LASTEXITCODE | Should -Be 0
    }
    It "succeeds with autobuild" {
        bio pkg bulkupload --auto-build $cacheDir
        $LASTEXITCODE | Should -Be 0
    }
    It "fails without directory argument" {
        bio pkg bulkupload
        $LASTEXITCODE | Should -Not -Be 0
    }
    It "fails when directory does not exist" {
        bio pkg bulkupload doesnotexist
        $LASTEXITCODE | Should -Not -Be 0
    }
    It "fails when given a bad url" {
        bio pkg bulkupload --url asdf $cacheDir
        $LASTEXITCODE | Should -Not -Be 0
    }
    It "fails when given a bad auth token" {
        bio pkg bulkupload --auth asdfjkl $cacheDir
        $LASTEXITCODE | Should -Not -Be 0
    }
    It "fails when missing channel name" {
        bio pkg bulkupload --channel $cacheDir
        $LASTEXITCODE | Should -Not -Be 0
    }

    BeforeAll {
        # origin create will exit 0 if the origin already exists
        bio origin create --url $env:HAB_BLDR_URL $env:HAB_ORIGIN
        if(Test-Path $cacheDir) { Remove-Item $cacheDir -Recurse -Force }
        New-Item (Join-Path $cacheDir "artifacts") -ItemType directory
        New-Item (Join-Path $cacheDir "keys") -ItemType directory
        # We always attempt to re-use the same package versions so we are not cluttering up Builder needlessly.
        # The packages may not exist yet in Builder, therefore we allow for failure on the download.
        bio pkg download --url $env:HAB_BLDR_URL --download-directory $cacheDir --channel unstable $testPkg1Ident $testPkg2Ident
        if((Get-ChildItem (Join-Path $cacheDir "artifacts")).Count -eq 0) {
            bio origin key download --secret --url $env:HAB_BLDR_URL --cache-key-path (Join-Path $cacheDir "keys") $env:HAB_ORIGIN
            bio origin key download --url $env:HAB_BLDR_URL --cache-key-path (Join-Path $cacheDir "keys") $env:HAB_ORIGIN
            @($testPkg1Dir, $testPkg2Dir) | ForEach-Object {
                # Build the packages and sign them with HAB_ORIGIN key from HAB_CACHE_KEY_PATH set above.
                bio pkg build $_
                Get-Content "results/last_build.env" | ForEach-Object { Add-Content "results/last_build.ps1" -Value "`$$($_.Replace("=", '="'))`"" }
                . results/last_build.ps1
                Copy-Item (Join-Path "results" $pkg_artifact) (Join-Path $cacheDir "artifacts")
            }
        }
    }
}
