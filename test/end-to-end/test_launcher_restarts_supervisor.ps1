# A simple test that the launcher restarts a supervisor when it exits abruptly
# The one optional argument set the exit code of the supervisor (default: 1).
# By default this runs against the installed biome binaries. To override and
# test locally-built code, set overrides in the environment of the script.
# See https://github.com/habitat-sh/habitat/blob/master/BUILDING.md#testing-changes

Describe "Exited Supervisor" {
    $exit_file = New-TemporaryFile
    $env:HAB_FEAT_TEST_EXIT=$exit_file
    $launcher_proc = Start-Supervisor -Timeout 45 -LogFile "sup.log"
    $supervisor_proc = Get-Process bio-sup
    Set-Content $exit_file -Value 1
    while(!$supervisor_proc.HasExited) {
        Start-Sleep 1
    }
    while(!(Get-Process bio-sup -ErrorAction SilentlyContinue)) {
        Start-Sleep 1
    }
    $new_supervisor_proc = Get-Process bio-sup
    $new_launcher_proc = Get-Process bio-launch

    It "will not exit the launcher" {
        $launcher_proc.HasExited | Should -Be $false
    }
    It "will spawn a new supervisor" {
        $new_supervisor_proc.Id | Should -Not -Be $supervisor_proc.Id
    }
    It "will not spawn a new launcher" {
        $new_launcher_proc.Id | Should -Be $launcher_proc.Id
    }
}
