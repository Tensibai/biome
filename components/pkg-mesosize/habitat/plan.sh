# shellcheck disable=2154
pkg_name=bio-pkg-mesosize
pkg_origin=biome
pkg_maintainer="The Biome Maintainers <humans@biome.sh>"
pkg_license=('Apache-2.0')
pkg_deps=(core/coreutils
          core/findutils
          core/gawk
          core/grep
          core/bash
          core/tar
          core/gzip
          biome/bio)
pkg_build_deps=()
pkg_bin_dirs=(bin)

program=$pkg_name

pkg_version() {
  if [[ -n "${DO_FAKE_RELEASE:-}" ]]; then
    cat "$SRC_PATH/../../VERSION_FAKE"
  else
    cat "$SRC_PATH/../../VERSION"
  fi
}
 
do_before() {
  do_default_before
  update_pkg_version
}

do_build() {
  cp -v "$SRC_PATH"/bin/${program}.sh ${program}

  # Use the bash from our dependency list as the shebang. Also, embed the
  # release version of the program.
  sed \
    -e "s,#!/bin/bash$,#!$(pkg_path_for bash)/bin/bash," \
    -e "s,@author@,$pkg_maintainer,g" \
    -e "s,@version@,$pkg_version/$pkg_release,g" \
    -i $program
}

do_install() {
  install -v -D $program "$pkg_prefix"/bin/$program
}
