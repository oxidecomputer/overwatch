#!/bin/bash
#:
#: name = "build"
#: variety = "basic"
#: target = "helios-latest"
#: rust_toolchain = "stable"
#: output_rules = [
#:   "/out/*",
#:   "/work/debug/*",
#:   "/work/release/*",
#: ]
#:
#: [[publish]]
#: series = "build"
#: name = "overwatch"
#: from_output = "/work/release/overwatch"
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

set -o errexit
set -o pipefail
set -o xtrace

cargo --version
rustc --version

banner "check"
cargo fmt -- --check
cargo clippy --all-targets -- --deny warnings
cargo check

banner "build"
ptime -m cargo build
ptime -m cargo build --release

for x in debug release
do
    mkdir -p /work/$x
    cp target/$x/overwatch /work/$x/
done

banner package
pfexec mkdir -p /out
pfexec chown "$UID" /out
cd target/release
tar cvf /out/overwatch.tar overwatch

banner checksum
cd /out
digest -a sha256 overwatch.tar > overwatch.sha256.txt
