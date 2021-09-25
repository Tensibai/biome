$ErrorActionPreference="stop"

Write-Host "--- Generating a signing key"
bio origin key generate "$env:HAB_ORIGIN"

Write-Host "--- Testing fresh install of Biome can communicate with builder"
Context "Biome SSL Cache" {
    BeforeEach {
        Remove-Item c:\hab\cache\ssl -Recurse -Force
    }

    Describe "Fresh Install" {
        # Note this isn't a pure fresh install, as we had to bootstrap this machine with
        # The previous release in order to test the current build.  However, the BeforeEach
        # block above ensures we don't have any cached certs that this may be pulling in.
        It "Can install packages" {
            bio pkg install core/7zip --channel stable
            $LASTEXITCODE | Should -Be 0
        }
    }

    Describe "Custom Certificates" {
        It "Can install packages when an invalid certificate is present" {
            New-Item -Type Directory -Path c:\hab\cache\ssl
            New-Item -Type File -Path c:\hab\cache\ssl\invalid-certifcate.pem
            Add-Content -Path c:\hab\cache\ssl\invalid-certificate.pem "I AM NOT A CERTIFICATE"

            bio pkg install core/nginx --channel stable
            $LASTEXITCODE | Should -Be 0
        }

        It "Loads custom certificates" {
            New-Item -Type Directory -Path c:\hab\cache\ssl
            bio pkg install core/openssl --channel stable
            bio pkg exec core/openssl openssl req -newkey rsa:2048 -batch -nodes -keyout key.pem -x509 -days 365 -out c:\hab\cache\ssl\custom-certificate.pem

            $env:RUST_LOG="debug"
            Start-Process bio -ArgumentList "pkg search core/lessmsi" -RedirectStandardError err.log -Wait
            $env:RUST_LOG="info"
            Get-Content -Raw -Path err.log | Should -Match "Processing cert file: C:\\hab/cache/ssl\\custom-certificate.pem"
        }
    }

    Describe "Studio" {
        It "Makes custom certificates available in the studio" {
            $customCertFile = "c:\hab\cache\ssl\custom-certificate.pem"
            # We can use unix style pathing here to get the real root of the studio
            # That way we don't have to worry about the studio name if our
            # CWD ever changes from 'workdir'
            $studioCertFile = "/hab/cache/ssl/custom-certificate.pem"
            New-Item -Type Directory -Path c:\hab\cache\ssl
            bio pkg install core/openssl --channel stable
            bio pkg exec core/openssl openssl req -newkey rsa:2048 -batch -nodes -keyout key.pem -x509 -days 365 -out $customCertFile

            $result = bio studio run "(Test-Path $studioCertFile).ToString()"
            Write-Host $result
            $result[-1] | Should -Be "True"
        }
    }
}
