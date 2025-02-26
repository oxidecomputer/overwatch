# Overwatch Zone

This binary can be used to produce an Omicron-branded Zone image, which contains
the overwatch binary in a specially formatted tarball.

A manifest describing this Zone image exists in
[package-manifest.toml](../package-manifest.toml), and the resulting image is
created as `out/overwatch.tar.gz`.

To create the Zone image:

```rust
$ cargo build --release
$ cargo run -p overwatch-package
```
