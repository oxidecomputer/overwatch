// Copyright 2023 Oxide Computer Company

struct ingress_metadata_t {
    bit<16> port;
    bool dst;
    bool src;
}

struct egress_metadata_t {
    bit<16> port;
    bool broadcast;
    bool drop;
}

extern Checksum {
    bit<16> run<T>(in T data);
}