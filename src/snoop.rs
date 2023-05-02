use crate::cli::Snoop;
use crate::{dump, link};
use crate::{main_pipeline, packet_in};
use anyhow::Result;
use dlpi::{
    recv,
    sys::{dlpi_recvinfo_t, DLPI_PHYSADDR_MAX},
};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

//TODO figure out from OS
const MTU: usize = 1500;

pub fn run(s: &Snoop) -> Result<()> {
    let mut pipeline = init_pipeline(s);
    let lnk = link::open(&s.link)?;

    dump::sep();
    loop {
        let mut src = [0u8; DLPI_PHYSADDR_MAX];
        let mut msg = vec![0u8; MTU];
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
            dump::headers(h);
        }
    }
}

fn init_pipeline(cfg: &Snoop) -> main_pipeline {
    let mut pipeline = main_pipeline::new(2);

    if let Some(src) = cfg.ip_src {
        set_ip_src(&mut pipeline, src);
    }
    if let Some(dst) = cfg.ip_dst {
        set_ip_dst(&mut pipeline, dst);
    }
    if cfg.v6 {
        v6_only(&mut pipeline);
    }
    if cfg.v4 {
        v4_only(&mut pipeline);
    }
    if cfg.arp {
        arp_only(&mut pipeline);
    }

    pipeline
}

fn set_ip_src(pipeline: &mut main_pipeline, src: IpAddr) {
    match src {
        IpAddr::V4(v4) => set_ip4_src(pipeline, v4),
        IpAddr::V6(v6) => set_ip6_src(pipeline, v6),
    }
}

fn set_ip4_src(pipeline: &mut main_pipeline, src: Ipv4Addr) {
    let mut key = vec![1];
    let mut octets = src.octets().to_vec();
    octets.reverse();
    key.extend_from_slice(octets.as_slice());
    pipeline.add_ingress_ipv4_src_entry("keep", key.as_slice(), &[], 100);
    key[0] = 0;
    pipeline.add_ingress_ipv4_src_entry("drop", key.as_slice(), &[], 0);
}

fn set_ip6_src(pipeline: &mut main_pipeline, src: Ipv6Addr) {
    let mut key = vec![1];
    let mut octets = src.octets().to_vec();
    octets.reverse();
    key.extend_from_slice(&octets);
    pipeline.add_ingress_ipv6_src_entry("keep", key.as_slice(), &[], 100);
    key[0] = 0;
    pipeline.add_ingress_ipv6_src_entry("drop", key.as_slice(), &[], 0);
}

fn set_ip_dst(pipeline: &mut main_pipeline, dst: IpAddr) {
    match dst {
        IpAddr::V4(v4) => set_ip4_dst(pipeline, v4),
        IpAddr::V6(v6) => set_ip6_dst(pipeline, v6),
    }
}

fn set_ip4_dst(pipeline: &mut main_pipeline, dst: Ipv4Addr) {
    let mut key = vec![1];
    let mut octets = dst.octets().to_vec();
    octets.reverse();
    key.extend_from_slice(octets.as_slice());
    pipeline.add_ingress_ipv4_dst_entry("keep", key.as_slice(), &[], 100);
    key[0] = 0;
    pipeline.add_ingress_ipv4_dst_entry("drop", key.as_slice(), &[], 0);
}

fn set_ip6_dst(pipeline: &mut main_pipeline, dst: Ipv6Addr) {
    let mut key = vec![1];
    let mut octets = dst.octets().to_vec();
    octets.reverse();
    key.extend_from_slice(&octets);
    pipeline.add_ingress_ipv6_dst_entry("keep", key.as_slice(), &[], 100);
    key[0] = 0;
    pipeline.add_ingress_ipv6_dst_entry("drop", key.as_slice(), &[], 0);
}

fn v4_only(pipeline: &mut main_pipeline) {
    let mut key = vec![1];
    key.extend_from_slice(0x800u16.to_le_bytes().as_slice());
    pipeline.add_ingress_eth_ethertype_entry("keep", key.as_slice(), &[], 100);
    key[0] = 0;
    pipeline.add_ingress_eth_ethertype_entry("drop", key.as_slice(), &[], 0);
}

fn v6_only(pipeline: &mut main_pipeline) {
    let mut key = vec![1];
    key.extend_from_slice(0x86ddu16.to_le_bytes().as_slice());
    pipeline.add_ingress_eth_ethertype_entry("keep", key.as_slice(), &[], 100);
    key[0] = 0;
    pipeline.add_ingress_eth_ethertype_entry("drop", key.as_slice(), &[], 0);
}

fn arp_only(pipeline: &mut main_pipeline) {
    let mut key = vec![1];
    key.extend_from_slice(0x806u16.to_le_bytes().as_slice());
    pipeline.add_ingress_eth_ethertype_entry("keep", key.as_slice(), &[], 100);
    key[0] = 0;
    pipeline.add_ingress_eth_ethertype_entry("drop", key.as_slice(), &[], 0);
}
