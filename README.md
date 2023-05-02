# Overwatch

Connecting high-level networking requirements to low-level system configuration
and behaviors.

See [RFD 389](https://rfd.shared.oxide.computer/rfd/0389) for motivations. This
is an early work-in-progress prototype.

## Current Capabilities
- P4 programmable snoop with support for all packet types the Oxide platform
  switches support.
- Interpret hex-encoded packet dumps from other tools such as `snoop`, `dtrace` and
  `mdb`.
