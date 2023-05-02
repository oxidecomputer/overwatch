// Copyright 2023 Oxide Computer Company

#include <core.p4>
#include <watchnpu.p4>
#include <headers.p4>
#include <parse.p4>

SoftNPU(
    parse(),
    ingress(),
    egress()
) main;

control eth(
    inout headers_t hdr,
    inout ingress_metadata_t ingress,
    inout egress_metadata_t egress,
) {
    action keep() { egress.port = 16w1; }
    action drop() { egress.drop = true; }

    table ethertype {
        key = { hdr.ethernet.ether_type: ternary; }
        actions = { keep; drop; }
        default_action = NoAction;
    }

    apply {
        ethertype.apply();
    }
}

control ipv4(
    inout headers_t hdr,
    inout ingress_metadata_t ingress,
    inout egress_metadata_t egress,
) {
    action keep() { egress.port = 16w1; }
    action drop() { egress.drop = true; }

    table src {
        key = { hdr.ipv4.src: ternary; }
        actions = { keep; drop; }
        default_action = NoAction;
    }

    table dst {
        key = { hdr.ipv4.dst: ternary; }
        actions = { keep; drop; }
        default_action = NoAction;
    }

    apply {
        src.apply();
        dst.apply();
    }
}

control ipv6(
    inout headers_t hdr,
    inout ingress_metadata_t ingress,
    inout egress_metadata_t egress,
) {
    action keep() { egress.port = 16w1; }
    action drop() { egress.drop = true; }

    table src {
        key = { hdr.ipv6.src: ternary; }
        actions = { keep; drop; }
        default_action = NoAction;
    }

    table dst {
        key = { hdr.ipv6.dst: ternary; }
        actions = { keep; drop; }
        default_action = NoAction;
    }

    apply {
        src.apply();
        dst.apply();
    }
}

control ingress(
    inout headers_t hdr,
    inout ingress_metadata_t ingress,
    inout egress_metadata_t egress,
) {
    eth() eth;
    ipv4() ipv4;
    ipv6() ipv6;

    apply {
        egress.port = 16w1;

        if (hdr.ethernet.isValid()) {
            eth.apply(hdr, ingress, egress);
        }
        if (hdr.ipv4.isValid()) {
            ipv4.apply(hdr, ingress, egress);
        }
        if (hdr.ipv6.isValid()) {
            ipv6.apply(hdr, ingress, egress);
        }
    }
}

control egress(
    inout headers_t hdr,
    inout ingress_metadata_t ingress,
    inout egress_metadata_t egress,
) {
}
