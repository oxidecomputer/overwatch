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
    inout ethernet_h ethernet,
    inout egress_metadata_t egress,
) {
    action keep() { egress.port = 16w1; }
    action drop() { egress.drop = true; }

    table ethertype {
        key = { ethernet.ether_type: ternary; }
        actions = { keep; drop; }
        default_action = NoAction;
    }

    apply {
        ethertype.apply();
    }
}

control ipv4(
    inout ipv4_h ipv4,
    inout egress_metadata_t egress,
) {
    action keep() { egress.port = 16w1; }
    action drop() { egress.drop = true; }

    table src {
        key = { ipv4.src: ternary; }
        actions = { keep; drop; }
        default_action = NoAction;
    }

    table dst {
        key = { ipv4.dst: ternary; }
        actions = { keep; drop; }
        default_action = NoAction;
    }

    table host {
        key = { 
            ipv4.src: ternary; 
            ipv4.dst: ternary; 
        }
        actions = { keep; drop; }
        default_action = NoAction;
    }

    table proto {
        key = { ipv4.protocol: ternary; }
        actions = { keep; drop; }
        default_action = NoAction;
    }

    apply {
        src.apply();
        dst.apply();
        host.apply();
        proto.apply();
    }
}

control ipv6(
    inout ipv6_h ipv6,
    inout egress_metadata_t egress,
) {
    action keep() { egress.port = 16w1; }
    action drop() { egress.drop = true; }

    table src {
        key = { ipv6.src: ternary; }
        actions = { keep; drop; }
        default_action = NoAction;
    }

    table dst {
        key = { ipv6.dst: ternary; }
        actions = { keep; drop; }
        default_action = NoAction;
    }

    table host {
        key = { 
            ipv6.src: ternary; 
            ipv6.dst: ternary; 
        }
        actions = { keep; drop; }
        default_action = NoAction;
    }

    table proto {
        key = { ipv6.next_hdr: ternary; }
        actions = { keep; drop; }
        default_action = NoAction;
    }

    apply {
        src.apply();
        dst.apply();
        host.apply();
        proto.apply();
    }
}

control ports(
    inout bit<16> src_port,
    inout bit<16> dst_port,
    inout egress_metadata_t egress,
) {
    action keep() { egress.port = 16w1; }
    action drop() { egress.drop = true; }

    table src {
        key = { src_port: ternary; }
        actions = { keep; drop; }
        default_action = NoAction;
    }

    table dst {
        key = { dst_port: ternary; }
        actions = { keep; drop; }
        default_action = NoAction;
    }

    table port {
        key = { 
            src_port: ternary;
            dst_port: ternary;
        }
        actions = { keep; drop; }
        default_action = NoAction;
    }

    apply {
        src.apply();
        dst.apply();
        port.apply();
    }

}

control app(
    in bit<8> alp,
    inout egress_metadata_t egress,
) {
    action keep() { egress.port = 16w1; }
    action drop() { egress.drop = true; }

    table proto {
        key = {
            alp: ternary;
        }
        actions = { keep; drop; }
        default_action = NoAction;
    }

    apply {
        proto.apply();
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
    ports() ports;
    app() app;

    eth() inner_eth;
    ipv4() inner_ipv4;
    ipv6() inner_ipv6;
    ports() inner_ports;
    app() inner_app;

    apply {
        egress.port = 16w1;

        // outer
        if (hdr.ethernet.isValid()) {
            eth.apply(hdr.ethernet, egress);
        }
        if (hdr.ipv4.isValid()) {
            ipv4.apply(hdr.ipv4, egress);
        }
        if (hdr.ipv6.isValid()) {
            ipv6.apply(hdr.ipv6, egress);
        }
        if (hdr.udp.isValid()) {
            ports.apply(ingress.src_port, ingress.dst_port, egress);
        }
        if (hdr.tcp.isValid()) {
            ports.apply(ingress.src_port, ingress.dst_port, egress);
        }
        app.apply(ingress.alp, egress);

        // inner
        if (hdr.inner_eth.isValid()) {
            inner_eth.apply(hdr.inner_eth, egress);
        }
        if(hdr.inner_ipv4.isValid()) {
            inner_ipv4.apply(hdr.inner_ipv4, egress);
        }
        if(hdr.inner_ipv6.isValid()) {
            inner_ipv6.apply(hdr.inner_ipv6, egress);
        }
        if (hdr.inner_udp.isValid()) {
            ports.apply(ingress.inner_src_port, ingress.inner_dst_port, egress);
        }
        if (hdr.inner_tcp.isValid()) {
            ports.apply(ingress.inner_src_port, ingress.inner_dst_port, egress);
        }
        inner_app.apply(ingress.inner_alp, egress);
    }
}

control egress(
    inout headers_t hdr,
    inout ingress_metadata_t ingress,
    inout egress_metadata_t egress,
) {
}
