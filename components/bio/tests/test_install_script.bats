setup() {
  if [ -n "$CI" ]; then
    rm -f /bin/bio
    rm -rf /hab/pkgs/biome/bio
  else
    echo "Not running in CI, skipping cleanup"
  fi
}

darwin() {
  [ "$(uname)" == "Darwin" ]
}

linux() {
  [ "$(uname)" == "Linux" ]
}

installed_version() {
  bio --version | cut -d'/' -f1
}

installed_target() {
  version_release="$(bio --version | cut -d' ' -f2)"
  version="$(cut -d'/' -f1 <<< "$version_release")"
  release="$(cut -d'/' -f2 <<< "$version_release")"
  cat /hab/pkgs/biome/bio/"$version"/"$release"/TARGET
}

@test "Install latest for x86_86-linux" {
  linux || skip "Did not detect a Linux system"
  run components/bio/install.sh

  [ "$status" -eq 0 ]
  [ "$(installed_target)" == "x86_64-linux" ]
}

@test "Install specific version for x86_64-linux" {
  linux || skip "Did not detect a Linux system"
  run components/bio/install.sh -v 0.90.6

  [ "$status" -eq 0 ]
  [ "$(installed_version)" == "bio 0.90.6" ]
  [ "$(installed_target)" == "x86_64-linux" ]
}

@test "Install from bintray for x86_84-linux" {
  linux || skip "Did not detect a Linux system"
  run components/bio/install.sh -v 0.79.1 

  [ "$status" -eq 0 ]
  [ "$(installed_version)" == "bio 0.79.1" ]
  [ "$(installed_target)" == "x86_64-linux" ]
}

@test "Install latest for x86_64-linux-kernel2" {
  linux || skip "Did not detect a Linux system"
  run components/bio/install.sh -t "x86_64-linux-kernel2"

  [ "$status" -eq 0 ]
  [ "$(installed_target)" == "x86_64-linux-kernel2" ]
}

@test "Install specific version for x86_64-linux-kernel2" {
  linux || skip "Did not detect a Linux system"
  run components/bio/install.sh -v 0.90.6 -t "x86_64-linux-kernel2"

  [ "$status" -eq 0 ]
  [ "$(installed_version)" == "bio 0.90.6" ]
  [ "$(installed_target)" == "x86_64-linux-kernel2" ]
}

@test "Install from bintray for x86_84-linux-kernel2" {
  linux || skip "Did not detect a Linux system"
  run components/bio/install.sh -v 0.79.1 -t "x86_64-linux-kernel2"

  [ "$status" -eq 0 ]
  [ "$(installed_version)" == "bio 0.79.1" ]
  [ "$(installed_target)" == "x86_64-linux-kernel2" ]
}

@test "Install latest for x86_86-darwin" {
  darwin || skip "Did not detect a Darwin system"
  run components/bio/install.sh

  [ "$status" -eq 0 ]
}

@test "Install specific version for x86_64-darwin" {
  darwin || skip "Did not detect a Darwin system"
  run components/bio/install.sh -v 0.90.6
  
  [ "$status" -eq 0 ]
  [ "$(installed_version)" == "bio 0.90.6" ]
}

@test "Install from bintray for x86_84-darwin" {
  darwin || skip "Did not detect a Darwin system"
  run components/bio/install.sh -v 0.79.1

  [ "$status" -eq 0 ]
  [ "$(installed_version)" == "bio 0.79.1" ]
}


