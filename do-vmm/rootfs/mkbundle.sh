#!/usr/bin/bash

mkdir -p ctr-bundle/rootfs

# Download and untar an alpine image
curl -O https://dl-cdn.alpinelinux.org/alpine/v3.14/releases/x86_64/alpine-minirootfs-3.14.2-x86_64.tar.gz
tar xf alpine-minirootfs-3.14.2-x86_64.tar.gz -C ctr-bundle/rootfs

# Create an ubuntu based root filesystem
#podman export $(podman create ubuntu) | tar -C rootfs -xvf -

pushd ctr-bundle

# Generate a runtime spec
runc spec --rootless

popd



