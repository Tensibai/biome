# Regression test to address https://github.com/habitat-sh/habitat/issues/4673
# Fixed in https://github.com/habitat-sh/habitat/pull/5365

Function Get-SocketFile {
    Get-ChildItem /tmp/rust-ipc-socket.* | ForEach-Object { $_.Name }
}

Describe "Supervisor shutdown" {
    $socket_files_before = New-TemporaryFile
    Get-SocketFile | Out-File $socket_files_before

    $launcher_proc = Start-Supervisor -Timeout 60
    bio sup term
    $launcher_proc.WaitForExit()

    $socket_files_after = New-TemporaryFile
    Get-SocketFile | Out-File $socket_files_after

    It "cleans up socket files" {
        Compare-Object $socket_files_before $socket_files_after -ExcludeDifferent | Should -Be $null
    }
}
