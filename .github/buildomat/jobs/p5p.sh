#!/bin/bash
#:
#: name = "overwatch-p5p"
#: variety = "basic"
#: target = "helios-2.0"
#: rust_toolchain = "stable"
#: output_rules = [
#:   "=/out/overwatch.p5p",
#:   "=/out/overwatch.p5p.sha256",
#: ]
#:
#: [[publish]]
#: series = "repo"
#: name = "overwatch.p5p"
#: from_output = "/out/overwatch.p5p"
#:
#: [[publish]]
#: series = "repo"
#: name = "overwatch.p5p.sha256"
#: from_output = "/out/overwatch.p5p.sha256"
#:

set -o errexit
set -o pipefail
set -o xtrace

cargo --version
rustc --version

banner build
ptime -m cargo build --release

banner package
cargo xtask package

banner copy
pfexec mkdir -p /out
pfexec chown "$UID" /out
PKG_NAME="/out/overwatch.p5p"
mv pkg/packages/repo/*.p5p "$PKG_NAME"
sha256sum "$PKG_NAME" > "$PKG_NAME.sha256"
