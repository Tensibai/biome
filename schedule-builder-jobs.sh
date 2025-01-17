#!/bin/sh

# This script schedules build for whole biome distro
# It is interactive

# Linux
bio bldr job start -g biome/bio
bio bldr job start    biome/bio-launcher
bio bldr job start    biome/bio-sup
bio bldr job start    biome/bio-pkg-export-docker
bio bldr job start    biome/bio-pkg-export-tar
bio bldr job start    biome/bio-pkg-export-helm
bio bldr job start    biome/bio-pkg-export-kubernetes

# Windows
bio bldr job start -g biome/bio x86_64-windows
bio bldr job start    biome/bio-launcher x86_64-windows
bio bldr job start    biome/bio-sup x86_64-windows
bio bldr job start    biome/bio-pkg-export-docker x86_64-windows
bio bldr job start    biome/bio-pkg-export-tar x86_64-windows
bio bldr job start    biome/bio-pkg-export-helm x86_64-windows
bio bldr job start    biome/bio-pkg-export-kubernetes x86_64-windows
