# SPDX-License-Identifier: MPL-2.0
# ClusterFuzzLite build image for betlang.
#
# ClusterFuzzLite invokes `docker build -f .clusterfuzzlite/Dockerfile`,
# so this file must be named `Dockerfile`. It mirrors the OCI-neutral
# Containerfile in this directory (kept for podman-first local builds per
# the authority stack); keep the two in sync.
FROM gcr.io/oss-fuzz-base/base-builder-rust
RUN apt-get update && apt-get install -y make autoconf automake libtool
COPY . $SRC/betlang
WORKDIR $SRC/betlang
COPY .clusterfuzzlite/build.sh $SRC/
