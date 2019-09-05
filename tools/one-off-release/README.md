# Creating a One-Off Release

**WARNING** - One-off builds are generally a bad idea and should be used with extreme caution and only as a last resort.

## Run the Release Pipeline

1. Open [https://buildkite.com/chef/biome-sh-biome-master-release-biome](https://buildkite.com/chef/biome-sh-biome-master-release-biome) and click on "New Build"
2. Fill in the requested information. You can input a description of the build, select the commit and branch, and set environment variables.
3. Click "Create Build". This will start the build and take you to a web page showing the build status. Build artifacts are currently only uploaded to builder acceptance.

## Upload Build Artifacts to Live Builder

1. Wait for the build to finish successfully.
2. Copy the output of the release pipeline. The output should be below the pipeline status next to a pencil icon. Each line should have the form `* package (target)`. For example: 

``` text
* biome/bio/0.84.33/20190819184101 (x86_64-linux-kernel2)
* biome/bio/0.84.33/20190819184052 (x86_64-linux)
* biome/bio/0.84.33/20190819184126 (x86_64-windows)
* biome/bio/0.84.33/20190819184159 (x86_64-darwin)
* biome/bio-plan-build/0.84.33/20190819190131 (x86_64-linux-kernel2)
* biome/bio-plan-build-ps1/0.84.33/20190819190159 (x86_64-windows)
* biome/bio-plan-build/0.84.33/20190819190126 (x86_64-linux)
* biome/bio-backline/0.84.33/20190819190300 (x86_64-linux)
* biome/bio-backline/0.84.33/20190819190308 (x86_64-linux-kernel2)
* biome/bio-studio/0.84.33/20190819190415 (x86_64-linux-kernel2)
* biome/bio-studio/0.84.33/20190819190412 (x86_64-linux)
* biome/bio-studio/0.84.33/20190819190434 (x86_64-windows)
* biome/bio-launcher/11942/20190819190720 (x86_64-linux-kernel2)
* biome/bio-launcher/11942/20190819190715 (x86_64-linux)
* biome/bio-launcher/11942/20190819190756 (x86_64-windows)
* biome/bio-sup/0.84.33/20190819192034 (x86_64-linux-kernel2)
* biome/bio-sup/0.84.33/20190819192058 (x86_64-windows)
* biome/bio-sup/0.84.33/20190819192019 (x86_64-linux)
* biome/bio-pkg-mesosize/0.84.33/20190819193654 (x86_64-linux)
* biome/bio-pkg-aci/0.84.33/20190819193651 (x86_64-linux)
* biome/bio-pkg-export-tar/0.84.33/20190819193657 (x86_64-linux-kernel2)
* biome/bio-pkg-export-tar/0.84.33/20190819193654 (x86_64-linux)
* biome/bio-pkg-export-docker/0.84.33/20190819193646 (x86_64-linux)
* biome/bio-pkg-export-kubernetes/0.84.33/20190819193657 (x86_64-linux)
* biome/bio-pkg-export-helm/0.84.33/20190819193651 (x86_64-linux)
* biome/bio-pkg-export-tar/0.84.33/20190819193730 (x86_64-windows)
* biome/bio-pkg-export-docker/0.84.33/20190819193730 (x86_64-windows)
* biome/bio-pkg-cfize/0.84.33/20190819195144 (x86_64-linux)
```
3. From this directory run the following command to see the help of the `one-off-release` script. Note, the first time you run this command it will have to download the dependencies and build the tool which can take some time. Subsequent runs will be much faster.
    > cargo run --release -- -h
4. Run the following command to upload the build artifacts to builder live. You must specify your authorization token and can optionally supply a channel to tag the artifacts with.
    > cargo run --release -- -z "live-builder-auth-token" -c "channel-tag"
5. Paste the output of the release pipeline into stdin. Enter a newline to stdin (hit enter twice).

The script does the following:
1. Parse the package identifier and target out of each line
2. Downloads the packages from builder acceptance
3. Stores the package hart files in a temporary directory (by default `one-off-release-tmp-dir`)
4. Uses the `bio` command to upload the hart files to builder live
