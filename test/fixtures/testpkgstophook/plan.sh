pkg_name="testpkgstophook"
pkg_origin="biome-testing"
pkg_version="0.1.0"
pkg_maintainer="The Biome Maintainers <humans@biome.sh>"
pkg_license=("Apache-2.0")
pkg_deps=()
pkg_build_deps=()
do_build() {
  return 0
}

# shellcheck disable=SC2154
do_install() {
  mkdir -p "$pkg_prefix/hooks"
  cat <<EOT >> "$pkg_prefix/hooks/run"
#!/bin/sh

_term() {
  echo "run hook is terminating" >> /tmp/testpkgstophook.out
}

trap _term TERM

while true; do
  sleep 1
done
EOT

  cat <<EOT >> "$pkg_prefix/hooks/post-stop"
#!/bin/sh
set -e

echo "post-stop hook has fired" >> /tmp/testpkgstophook.out
EOT
  chmod 755 "$pkg_prefix/hooks/run"
}
