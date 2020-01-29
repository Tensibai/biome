# This tests that removing a follower from a functioning leader topology
# service group will continue to perform a succesful roaming update
# We will load services on three nodes and then stop the supervisor on one
# of the follower nodes. Next we perform an update and expect the remaining
# two nodes to update. Prior to https://github.com/habitat-sh/habitat/pull/7167
# a rolling update after a member death would cause the leader to wait for dead
# members to update themselves which of course will never happen. So we
# perform another update which should succeed if the leader is ignoring dead
# members as it should.

$testChannel = "rolling-$([DateTime]::Now.Ticks)"

Describe "Rolling Update after a follower is removed and quorum is not lost" {
    $release1="biome-testing/nginx/1.17.4/20191115184838"
    $release2="biome-testing/nginx/1.17.4/20191115185517"
    $release3="biome-testing/nginx/1.17.4/20191115185900"
    bio pkg promote $release1 $testChannel
    Load-SupervisorService "biome-testing/nginx" -Remote "alpha.biome.dev" -Topology leader -Strategy rolling -Channel $testChannel
    Load-SupervisorService "biome-testing/nginx" -Remote "beta.biome.dev" -Topology leader -Strategy rolling -Channel $testChannel
    Load-SupervisorService "biome-testing/nginx" -Remote "gamma.biome.dev" -Topology leader -Strategy rolling -Channel $testChannel

    @("alpha", "beta", "gamma") | ForEach-Object {
        It "loads initial release on $_" {
            Wait-Release -Ident $release1 -Remote $_
        }
    }

    Context "Remove first follower" {
        $leader = Get-Leader "bastion" "nginx.default"
        $follower=$null
        @("alpha", "beta", "gamma") | ForEach-Object {
            if($_ -ne $leader.Name -and !$follower) {
                $follower = $_
            }
        }
        Stop-ComposeSupervisor $follower
        bio pkg promote $release2 $testChannel

        # we expect everyone to be updated now but prior to
        # https://github.com/habitat-sh/habitat/pull/7167 the leader will
        # indefinitely wait for the dead followers to update
        @("alpha", "beta", "gamma") | Where-Object { $_ -ne $follower } | ForEach-Object {
            It "updates to $release2 on $_" {
                Wait-Release -Ident $release2 -Remote $_
            }
        }

        Context "update again" {
            # if the leader is not stuck waiting for dead members for the previous update,
            # this update should succeed
            bio pkg promote $release3 $testChannel
            @("alpha", "beta", "gamma") | Where-Object { $_ -ne $follower } | ForEach-Object {
                It "updates to $release3 on $_" {
                    Wait-Release -Ident $release3 -Remote $_
                }
            }
        }
    }

    AfterAll {
        bio bldr channel destroy $testChannel --origin biome-testing
    }
}
