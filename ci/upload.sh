#!/bin/bash -e

PACKAGES=(
    components/bio
    components/launcher
    components/sup

    components/plan-build
    components/backline
    components/studio

    components/pkg-export-container
    components/pkg-cfize
    components/pkg-export-tar
    components/pkg-mesosize
)

for pkg in "${PACKAGES[@]}"; do
    source "results/$(basename "$pkg").env"

    # shellcheck disable=SC2154
    bio pkg upload results/"$pkg_artifact"
done
