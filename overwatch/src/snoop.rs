// Copyright 2023 Oxide Computer Company

use crate::cli::Snoop;
use crate::{dump, link};
use crate::{main_pipeline, packet_in};
use anyhow::Result;
use dlpi::{
    recv,
    sys::{dlpi_recvinfo_t, DLPI_PHYSADDR_MAX},
};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

//TODO figure out from OS MTU
const MAX_FRAME: usize = 9000;

pub fn run(s: &Snoop) -> Result<()> {
    let mut pipeline = init_pipeline(s);
    let lnk = link::open(&s.link)?;

    dump::sep();
    loop {
        let mut src = [0u8; DLPI_PHYSADDR_MAX];
        let mut msg = vec![0u8; MAX_FRAME];
        let mut recvinfo = dlpi_recvinfo_t::default();
        let n = match recv(lnk, &mut src, &mut msg, -1, Some(&mut recvinfo)) {
            Ok((_, n)) => n,
            Err(e) => {
                eprintln!("rx error: {}", e);
                continue;
            }
        };
        let mut pkt = packet_in::new(&msg[..n]);
        let hdrs = pipeline.process_packet_headers(0, &mut pkt);
        for (h, _) in hdrs {
            dump::frame(h, &msg[..n], s.hex);
        }
    }
}

fn init_pipeline(cfg: &Snoop) -> main_pipeline {
    let mut pipeline = main_pipeline::new(2);

    // outer
    if let Some(eth_type) = cfg.eth_type {
        ethtype_only(&mut pipeline, eth_type as u16, false);
    }
    for src in &cfg.ip_src {
        set_ip_src(&mut pipeline, *src, false);
    }
    for dst in &cfg.ip_dst {
        set_ip_dst(&mut pipeline, *dst, false);
    }
    for host in &cfg.ip_host {
        set_ip_host(&mut pipeline, *host, false);
    }
    for proto in &cfg.ip_proto {
        ip_proto(&mut pipeline, *proto as u8, false);
    }
    for src in &cfg.src_port {
        src_port(&mut pipeline, *src, false);
    }
    for dst in &cfg.dst_port {
        dst_port(&mut pipeline, *dst, false);
    }
    for p in &cfg.port {
        port(&mut pipeline, *p, false);
    }
    for alp in &cfg.alp {
        app_proto(&mut pipeline, *alp as u8, false);
    }
    if cfg.vlan {
        vlan_only(&mut pipeline);
    }
    for vid in &cfg.vid {
        vlan_vid(&mut pipeline, *vid);
    }
    if cfg.v4 {
        v4_only(&mut pipeline, false);
    }
    if cfg.v6 {
        v6_only(&mut pipeline, false);
    }
    if cfg.arp {
        arp_only(&mut pipeline, false);
    }

    // inner
    if let Some(eth_type) = cfg.eth_type {
        ethtype_only(&mut pipeline, eth_type as u16, true);
    }
    for src in &cfg.inner_ip_src {
        set_ip_src(&mut pipeline, *src, true);
    }
    for dst in &cfg.inner_ip_dst {
        set_ip_dst(&mut pipeline, *dst, true);
    }
    for host in &cfg.inner_ip_host {
        set_ip_host(&mut pipeline, *host, true);
    }
    for proto in &cfg.inner_ip_proto {
        ip_proto(&mut pipeline, *proto as u8, true)
    }
    for src in &cfg.src_port {
        src_port(&mut pipeline, *src, true);
    }
    for dst in &cfg.dst_port {
        dst_port(&mut pipeline, *dst, true);
    }
    for p in &cfg.port {
        port(&mut pipeline, *p, true);
    }
    for alp in &cfg.inner_alp {
        app_proto(&mut pipeline, *alp as u8, true)
    }
    if cfg.inner_v4 {
        v4_only(&mut pipeline, true);
    }
    if cfg.inner_v6 {
        v6_only(&mut pipeline, true);
    }
    if cfg.inner_arp {
        arp_only(&mut pipeline, true);
    }

    pipeline
}

fn set_ip_src(pipeline: &mut main_pipeline, src: IpAddr, encap: bool) {
    match src {
        IpAddr::V4(v4) => set_ip4_src(pipeline, v4, encap),
        IpAddr::V6(v6) => set_ip6_src(pipeline, v6, encap),
    }
}

fn set_ip4_src(pipeline: &mut main_pipeline, src: Ipv4Addr, encap: bool) {
    let mut key = vec![1];
    let mut octets = src.octets().to_vec();
    octets.reverse();
    key.extend_from_slice(octets.as_slice());
    if encap {
        pipeline.add_ingress_inner_ipv4_src_entry(
            "keep",
            key.as_slice(),
            &[],
            100,
        );
    } else {
        pipeline.add_ingress_ipv4_src_entry("keep", key.as_slice(), &[], 100);
    }
    key[0] = 0;
    if encap {
        pipeline.add_ingress_inner_ipv4_src_entry(
            "drop",
            key.as_slice(),
            &[],
            0,
        );
    } else {
        pipeline.add_ingress_ipv4_src_entry("drop", key.as_slice(), &[], 0);
    }
}

fn set_ip6_src(pipeline: &mut main_pipeline, src: Ipv6Addr, encap: bool) {
    let mut key = vec![1];
    let mut octets = src.octets().to_vec();
    octets.reverse();
    key.extend_from_slice(&octets);
    if encap {
        pipeline.add_ingress_inner_ipv6_src_entry(
            "keep",
            key.as_slice(),
            &[],
            100,
        );
    } else {
        pipeline.add_ingress_ipv6_src_entry("keep", key.as_slice(), &[], 100);
    }
    key[0] = 0;
    if encap {
        pipeline.add_ingress_inner_ipv6_src_entry(
            "drop",
            key.as_slice(),
            &[],
            0,
        );
    } else {
        pipeline.add_ingress_ipv6_src_entry("drop", key.as_slice(), &[], 0);
    }
}

fn set_ip_dst(pipeline: &mut main_pipeline, dst: IpAddr, encap: bool) {
    match dst {
        IpAddr::V4(v4) => set_ip4_dst(pipeline, v4, encap),
        IpAddr::V6(v6) => set_ip6_dst(pipeline, v6, encap),
    }
}

fn set_ip4_dst(pipeline: &mut main_pipeline, dst: Ipv4Addr, encap: bool) {
    let mut key = vec![1];
    let mut octets = dst.octets().to_vec();
    octets.reverse();
    key.extend_from_slice(octets.as_slice());
    if encap {
        pipeline.add_ingress_inner_ipv4_dst_entry(
            "keep",
            key.as_slice(),
            &[],
            100,
        );
    } else {
        pipeline.add_ingress_ipv4_dst_entry("keep", key.as_slice(), &[], 100);
    }

    key[0] = 0;
    if encap {
        pipeline.add_ingress_inner_ipv4_dst_entry(
            "drop",
            key.as_slice(),
            &[],
            0,
        );
    } else {
        pipeline.add_ingress_ipv4_dst_entry("drop", key.as_slice(), &[], 0);
    }
}

fn set_ip6_dst(pipeline: &mut main_pipeline, dst: Ipv6Addr, encap: bool) {
    let mut key = vec![1];
    let mut octets = dst.octets().to_vec();
    octets.reverse();
    key.extend_from_slice(&octets);
    if encap {
        pipeline.add_ingress_inner_ipv6_dst_entry(
            "keep",
            key.as_slice(),
            &[],
            100,
        );
    } else {
        pipeline.add_ingress_ipv6_dst_entry("keep", key.as_slice(), &[], 100);
    }
    key[0] = 0;
    if encap {
        pipeline.add_ingress_inner_ipv6_dst_entry(
            "drop",
            key.as_slice(),
            &[],
            0,
        );
    } else {
        pipeline.add_ingress_ipv6_dst_entry("drop", key.as_slice(), &[], 0);
    }
}

fn set_ip_host(pipeline: &mut main_pipeline, host: IpAddr, encap: bool) {
    match host {
        IpAddr::V4(v4) => set_ip4_host(pipeline, v4, encap),
        IpAddr::V6(v6) => set_ip6_host(pipeline, v6, encap),
    }
}

fn set_ip4_host(pipeline: &mut main_pipeline, host: Ipv4Addr, encap: bool) {
    let mut octets = host.octets().to_vec();
    octets.reverse();
    let stride = octets.len() + 1;

    // key 1
    let mut key = vec![1];
    key.extend_from_slice(&octets);
    // key 2
    key.push(0);
    key.extend_from_slice(&octets);

    // The first entry selects the host as a source with a dont-care for the
    // destination.

    if encap {
        pipeline.add_ingress_inner_ipv4_host_entry(
            "keep",
            key.as_slice(),
            &[],
            100,
        );
    } else {
        pipeline.add_ingress_ipv4_host_entry("keep", key.as_slice(), &[], 100);
    }

    // The second entry selects the destination as a source with a dont-care for
    // the source.

    key[0] = 0;
    key[stride] = 1;
    if encap {
        pipeline.add_ingress_inner_ipv4_host_entry(
            "keep",
            key.as_slice(),
            &[],
            100,
        );
    } else {
        pipeline.add_ingress_ipv4_host_entry("keep", key.as_slice(), &[], 100);
    }

    // The third entry is a drop for things that do not match at all with the
    // lowest priority.

    key[0] = 0;
    key[stride] = 0;
    if encap {
        pipeline.add_ingress_inner_ipv4_host_entry(
            "drop",
            key.as_slice(),
            &[],
            0,
        );
    } else {
        pipeline.add_ingress_ipv4_host_entry("drop", key.as_slice(), &[], 0);
    }
}

fn set_ip6_host(pipeline: &mut main_pipeline, host: Ipv6Addr, encap: bool) {
    let mut octets = host.octets().to_vec();
    octets.reverse();
    let stride = octets.len() + 1;

    // key 1
    let mut key = vec![1];
    key.extend_from_slice(&octets);
    // key 2
    key.push(0);
    key.extend_from_slice(&octets);

    // The first entry selects the host as a source with a dont-care for the
    // destination.

    if encap {
        pipeline.add_ingress_inner_ipv6_host_entry(
            "keep",
            key.as_slice(),
            &[],
            100,
        );
    } else {
        pipeline.add_ingress_ipv6_host_entry("keep", key.as_slice(), &[], 100);
    }

    // The second entry selects the destination as a source with a dont-care for
    // the source.

    key[0] = 0;
    key[stride] = 1;
    if encap {
        pipeline.add_ingress_inner_ipv6_host_entry(
            "keep",
            key.as_slice(),
            &[],
            100,
        );
    } else {
        pipeline.add_ingress_ipv6_host_entry("keep", key.as_slice(), &[], 100);
    }

    // The third entry is a drop for things that do not match at all with the
    // lowest priority.

    key[0] = 0;
    key[stride] = 0;
    if encap {
        pipeline.add_ingress_inner_ipv6_host_entry(
            "drop",
            key.as_slice(),
            &[],
            0,
        );
    } else {
        pipeline.add_ingress_ipv6_host_entry("drop", key.as_slice(), &[], 0);
    }
}

fn ip_proto(pipeline: &mut main_pipeline, proto: u8, encap: bool) {
    let key = [1, proto];
    if encap {
        pipeline.add_ingress_inner_ipv4_proto_entry(
            "keep",
            key.as_slice(),
            &[],
            100,
        );
        pipeline.add_ingress_inner_ipv6_proto_entry(
            "keep",
            key.as_slice(),
            &[],
            100,
        );
    } else {
        pipeline.add_ingress_ipv4_proto_entry("keep", key.as_slice(), &[], 100);
        pipeline.add_ingress_ipv6_proto_entry("keep", key.as_slice(), &[], 100);
    }
    let key = [0, proto];
    if encap {
        pipeline.add_ingress_inner_ipv4_proto_entry(
            "drop",
            key.as_slice(),
            &[],
            0,
        );
        pipeline.add_ingress_inner_ipv6_proto_entry(
            "drop",
            key.as_slice(),
            &[],
            0,
        );
    } else {
        pipeline.add_ingress_ipv4_proto_entry("drop", key.as_slice(), &[], 0);
        pipeline.add_ingress_ipv6_proto_entry("drop", key.as_slice(), &[], 0);
    }
}

fn src_port(pipeline: &mut main_pipeline, port: u16, encap: bool) {
    let mut key = vec![1];
    key.extend_from_slice(&port.to_le_bytes());
    if encap {
        pipeline.add_ingress_inner_ports_src_entry(
            "keep",
            key.as_slice(),
            &[],
            100,
        );
    } else {
        pipeline.add_ingress_ports_src_entry("keep", key.as_slice(), &[], 100);
    }

    key[0] = 0;
    if encap {
        pipeline.add_ingress_inner_ports_src_entry(
            "drop",
            key.as_slice(),
            &[],
            0,
        );
    } else {
        pipeline.add_ingress_ports_src_entry("drop", key.as_slice(), &[], 0);
    }
}

fn dst_port(pipeline: &mut main_pipeline, port: u16, encap: bool) {
    let mut key = vec![1];
    key.extend_from_slice(&port.to_le_bytes());
    if encap {
        pipeline.add_ingress_inner_ports_dst_entry(
            "keep",
            key.as_slice(),
            &[],
            100,
        );
    } else {
        pipeline.add_ingress_ports_dst_entry("keep", key.as_slice(), &[], 100);
    }

    key[0] = 0;
    if encap {
        pipeline.add_ingress_inner_ports_dst_entry(
            "drop",
            key.as_slice(),
            &[],
            0,
        );
    } else {
        pipeline.add_ingress_ports_dst_entry("drop", key.as_slice(), &[], 0);
    }
}

fn port(pipeline: &mut main_pipeline, port: u16, encap: bool) {
    // key1
    let mut key = vec![1];
    key.extend_from_slice(&port.to_le_bytes());
    // key2
    key.push(0);
    key.extend_from_slice(&port.to_le_bytes());
    let stride = std::mem::size_of::<u16>() + 1;

    if encap {
        pipeline.add_ingress_inner_ports_port_entry(
            "keep",
            key.as_slice(),
            &[],
            100,
        );
    } else {
        pipeline.add_ingress_ports_port_entry("keep", key.as_slice(), &[], 100);
    }

    key[0] = 0;
    key[stride] = 1;

    if encap {
        pipeline.add_ingress_inner_ports_port_entry(
            "keep",
            key.as_slice(),
            &[],
            100,
        );
    } else {
        pipeline.add_ingress_ports_port_entry("keep", key.as_slice(), &[], 100);
    }

    key[stride] = 0;
    if encap {
        pipeline.add_ingress_inner_ports_port_entry(
            "drop",
            key.as_slice(),
            &[],
            0,
        );
    } else {
        pipeline.add_ingress_ports_port_entry("drop", key.as_slice(), &[], 0);
    }
}

fn app_proto(pipeline: &mut main_pipeline, proto: u8, encap: bool) {
    let key = [1, proto];
    if encap {
        pipeline.add_ingress_inner_app_proto_entry(
            "keep",
            key.as_slice(),
            &[],
            100,
        );
    } else {
        pipeline.add_ingress_app_proto_entry("keep", key.as_slice(), &[], 100);
    }
    let key = [0, proto];
    if encap {
        pipeline.add_ingress_inner_app_proto_entry(
            "drop",
            key.as_slice(),
            &[],
            0,
        );
    } else {
        pipeline.add_ingress_app_proto_entry("drop", key.as_slice(), &[], 0);
    }
}

fn vlan_vid(pipeline: &mut main_pipeline, vid: u16) {
    let mut key = vec![1];
    key.extend_from_slice(vid.to_le_bytes().as_slice());
    pipeline.add_ingress_vlan_vid_entry("keep", key.as_slice(), &[], 100);
    key[0] = 0;
    pipeline.add_ingress_vlan_vid_entry("drop", key.as_slice(), &[], 0);
}

fn ethtype_only(pipeline: &mut main_pipeline, ethtype: u16, encap: bool) {
    let mut key = vec![1];
    key.extend_from_slice(ethtype.to_le_bytes().as_slice());
    if encap {
        pipeline.add_ingress_inner_eth_ethertype_entry(
            "keep",
            key.as_slice(),
            &[],
            100,
        );
    } else {
        pipeline.add_ingress_eth_ethertype_entry(
            "keep",
            key.as_slice(),
            &[],
            100,
        );
    }
    key[0] = 0;
    if encap {
        pipeline.add_ingress_inner_eth_ethertype_entry(
            "drop",
            key.as_slice(),
            &[],
            0,
        );
    } else {
        pipeline.add_ingress_eth_ethertype_entry(
            "drop",
            key.as_slice(),
            &[],
            0,
        );
    }
}

fn vlan_only(pipeline: &mut main_pipeline) {
    ethtype_only(pipeline, 0x8100, false)
}

fn v4_only(pipeline: &mut main_pipeline, encap: bool) {
    ethtype_only(pipeline, 0x0800, encap)
}

fn v6_only(pipeline: &mut main_pipeline, encap: bool) {
    ethtype_only(pipeline, 0x86dd, encap)
}

fn arp_only(pipeline: &mut main_pipeline, encap: bool) {
    ethtype_only(pipeline, 0x0806, encap)
}
