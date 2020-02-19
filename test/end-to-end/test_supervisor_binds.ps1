Describe "Supervisor binds" {
    BeforeAll {
        bio origin key generate $env:HAB_ORIGIN

        Invoke-BuildAndInstall testpkgbindproducer
        Invoke-BuildAndInstall testpkgbindconsumer

        Start-Supervisor -Timeout 45 | Out-Null
    }

    It "consumer bind to producer export" {
        Load-SupervisorService -PackageName $env:HAB_ORIGIN/testpkgbindproducer -Timeout 20
        Load-SupervisorService -PackageName $env:HAB_ORIGIN/testpkgbindconsumer -Timeout 20 -Bind alias:testpkgbindproducer.default

        # The consumer's myconfig.conf is a template that holds the value
        # of the producers exported property which should be "default1"
        Get-Content "/hab/svc/testpkgbindconsumer/config/myconfig.conf" | Should -Be "default1"
    }
}
