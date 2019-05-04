pkg_name=possums
pkg_origin=happyhumans
pkg_version=8.1.4
pkg_maintainer="The Biome Maintainers <humans@biome.sh>"
pkg_license=('apachev2')
pkg_source=nosuchfile.tar.gz
pkg_deps=()
pkg_build_deps=()

do_build() {
  cp -v "$PLAN_CONTEXT/signme.dat" signme.dat
}

# shellcheck disable=SC2154
do_install() {
  install -v -D signme.dat "$pkg_prefix/share/signme.dat"
}

# Turn the remaining default phases into no-ops

do_download() {
  return 0
}

do_verify() {
  return 0
}

do_unpack() {
  return 0
}

do_prepare() {
  return 0
}
