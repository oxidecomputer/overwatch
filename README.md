# Overwatch

This is a packet tracing tool much like snoop or tcpdump. Its superpower is
that it's P4 programmable.

## OS Support
- Illumos

## Core Capabilities
- Snoop raw packets
- Filter packets based on
    - IP addresses
    - IP protocol
    - L4 ports
    - Application layer protocol
    - VLAN VID
    - IP version
    - ARP
    - All the above for Geneve encapsulated packets.
- Show packet contents in nicely formatted hex.
- Render packet traces from raw data files in hex format.

## Contributing

Pull requests welcome. Please make sure CI scripts in the `.github` run OK
before requesting review.
