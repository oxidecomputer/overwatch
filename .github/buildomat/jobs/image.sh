#!/bin/bash
#:
#: name = "image"
#: variety = "basic"
#: target = "helios-latest"
#: rust_toolchain = "stable"
#: output_rules = [
#:   "/out/*",
#: ]
#:
#: [[publish]]
#: series = "image"
#: name = "overwatch.tar"
#: from_output = "/out/overwatch.tar"
#:
#: [[publish]]
#: series = "image"
#: name = "overwatch.sha256.txt"
#: from_output = "/out/overwatch.sha256.txt"
#:
#: [[publish]]
#: series = "image"
#: name = "omicron-zone-overwatch.tar.gz"
#: from_output = "/out/omicron-zone-overwatch.tar.gz"
#:
#: [[publish]]
#: series = "image"
#: name = "omicron-zone-overwatch.sha256.txt"
#: from_output = "/out/omicron-zone-overwatch.sha256.txt"

set -o errexit
set -o pipefail
set -o xtrace

cargo --version
rustc --version

banner build
ptime -m cargo build --release

banner image
ptime -m cargo run --release -p overwatch-package

banner package
pfexec mkdir -p /out
pfexec chown "$UID" /out

banner contents
tar tvfz out/omicron-zone-overwatch.tar.gz

banner copy
mv out/omicron-zone-overwatch.tar.gz /out/omicron-zone-overwatch.tar.gz
pushd target/release
tar cf /out/overwatch.tar overwatch
popd

banner checksum
cd /out
digest -a sha256 overwatch.tar > overwatch.sha256.txt
digest -a sha256 omicron-zone-overwatch.tar.gz > omicron-zone-overwatch.sha256.txt
