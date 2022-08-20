# This tests that removing the leader from a functioning leader topology
# service group that has enough nodes to maintain quorum after the leader is
# lost, it will continue to perform a succesful rolling update after a new
# leader is elected.
#
# We will load services on three nodes and then stop the supervisor on
# the leader node prompting a new election where one of the two follower nodes
# becomes a leader. Next we perform an update and expect both nodes to update.
# Prior to https://github.com/habitat-sh/habitat/pull/7167, the update after a
# new leader is elected would never occur because the new leader would continue
# to behave like a follower and wait for instructions to update.

$testChannel = "rolling-$([DateTime]::Now.Ticks)"

Describe "Rolling Update demotes a package in the middle of an update" {
    $release1="biome-testing/nginx/1.17.4/20191115184838"
    $release2="biome-testing/nginx/1.17.4/20191115185517"
    $release3="biome-testing/nginx/1.17.4/20191115185900"
    bio pkg promote $release1 $testChannel
    Load-SupervisorService "biome-testing/nginx" -Remote "alpha.biome.dev" -Topology leader -Strategy rolling -Channel $testChannel -UpdateCondition track-channel
    Load-SupervisorService "biome-testing/nginx" -Remote "beta.biome.dev" -Topology leader -Strategy rolling -Channel $testChannel -UpdateCondition track-channel
    Load-SupervisorService "biome-testing/nginx" -Remote "gamma.biome.dev" -Topology leader -Strategy rolling -Channel $testChannel -UpdateCondition track-channel

    @("alpha", "beta", "gamma") | ForEach-Object {
        It "loads initial release on $_" {
            Wait-Release -Ident $release1 -Remote $_
        }
    }

    Context "Promote Package" {
        $leader = Get-Leader "bastion" "nginx.default"
        bio pkg promote $release2 $testChannel

        It "updates $($leader.Name) to $release2" {
            Wait-Release -Ident $release2 -Remote $leader.Name
        }
    }

    Context "Demote Package" {
        bio pkg demote $release2 $testChannel

        @("alpha", "beta", "gamma") | ForEach-Object {
            It "updates to $release1 on $_" {
                Wait-Release -Ident $release1 -Remote $_
            }
        }
    }

    Context "Promote Package after demote" {
        bio pkg promote $release3 $testChannel

        @("alpha", "beta", "gamma") | ForEach-Object {
            It "updates to $release3 on $_" {
                Wait-Release -Ident $release3 -Remote $_ -Timeout 30
            }
        }
    }

    AfterAll {
        bio bldr channel destroy $testChannel --origin biome-testing
    }
}
