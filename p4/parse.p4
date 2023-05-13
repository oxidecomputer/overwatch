// Link layer ethertypes.
#define IPV4_ETHERTYPE      16w0x0800
#define IPV6_ETHERTYPE      16w0x86dd
#define ARP_ETHERTYPE       16w0x0806
#define SIDECAR_ETHERTYPE   16w0x0901
#define VLAN_ETHERTYPE      16w0x8100

// Network layer protocol numbers.
#define ICMP_IPPROTO    8w1
#define ICMP6_IPPROTO   8w58
#define UDP_IPPROTO     8w17
#define TCP_IPPROTO     8w6

// Transport layer port numbers.
#define GENEVE_PORT         16w6081
#define DDM_DISCOVERY_PORT  16w0xddd

// Application layer protocol identifirs.
#define ALP_GENEVE          8w0x1
#define ALP_BGP             8w0x2
#define ALP_HTTP            8w0x3
#define ALP_DDM_DISCOVERY   8w0x4
#define ALP_DDM_EXCHANGE    8w0x5

parser parse(
    packet_in pkt,
    out headers_t hdr,
    inout ingress_metadata_t ingress,
) {
    state start {
        pkt.extract(hdr.ethernet);
        if (hdr.ethernet.ether_type == IPV4_ETHERTYPE) {
            transition ipv4;
        }
        if (hdr.ethernet.ether_type == IPV6_ETHERTYPE) {
            transition ipv6;
        }
        if (hdr.ethernet.ether_type == SIDECAR_ETHERTYPE) {
            transition sidecar;
        }
        if (hdr.ethernet.ether_type == ARP_ETHERTYPE) {
            transition arp;
        }
        if (hdr.ethernet.ether_type == VLAN_ETHERTYPE) {
            transition vlan;
        }
        transition reject;
    }

    state vlan {
        pkt.extract(hdr.vlan);
        if (hdr.vlan.ether_type == IPV4_ETHERTYPE) {
            transition ipv4;
        }
        if (hdr.vlan.ether_type == IPV6_ETHERTYPE) {
            transition ipv6;
        }
        if (hdr.vlan.ether_type == SIDECAR_ETHERTYPE) {
            transition sidecar;
        }
        if (hdr.vlan.ether_type == ARP_ETHERTYPE) {
            transition arp;
        }
        transition reject;
    }

    state sidecar {
        pkt.extract(hdr.sidecar);
        if (hdr.sidecar.sc_ether_type == IPV4_ETHERTYPE) {
            transition ipv4;
        }
        if (hdr.sidecar.sc_ether_type == IPV6_ETHERTYPE) {
            transition ipv6;
        }
        transition reject;
    }

    state arp {
        pkt.extract(hdr.arp);
        transition accept;
    }

    state ipv6 {
        pkt.extract(hdr.ipv6);
        if (hdr.ipv6.next_hdr == ICMP_IPPROTO) {
            transition icmp;
        }
        if (hdr.ipv6.next_hdr == UDP_IPPROTO) {
            transition udp;
        }
        if (hdr.ipv6.next_hdr == TCP_IPPROTO) {
            transition tcp;
        }
        transition accept;
    }

    state icmp {
        pkt.extract(hdr.icmp);
        transition accept;
    }

    state ipv4 {
        pkt.extract(hdr.ipv4);
        if (hdr.ipv4.protocol == ICMP_IPPROTO) {
            transition icmp;
        }
        if (hdr.ipv4.protocol == UDP_IPPROTO) {
            transition udp;
        }
        if (hdr.ipv4.protocol == TCP_IPPROTO) {
            transition tcp;
        }
        transition accept;
    }

    state udp {
        pkt.extract(hdr.udp);
        ingress.src_port = hdr.udp.src_port;
        ingress.dst_port = hdr.udp.dst_port;
        if (hdr.udp.dst_port == GENEVE_PORT) {
            transition geneve;
        }
        if (hdr.udp.dst_port == DDM_DISCOVERY_PORT) {
            transition ddm_discovery;
        }
        transition accept;
    }

    state tcp {
        pkt.extract(hdr.tcp);
        ingress.src_port = hdr.tcp.src_port;
        ingress.dst_port = hdr.tcp.dst_port;
        transition accept;
    }

    state geneve {
        pkt.extract(hdr.geneve);
        ingress.alp = ALP_GENEVE;
        transition inner_eth;
    }

    state ddm_discovery {
        pkt.extract(hdr.ddm_discovery);
        ingress.alp = ALP_DDM_DISCOVERY;
        transition accept;
    }

    state inner_eth {
        pkt.extract(hdr.inner_eth);
        if (hdr.inner_eth.ether_type == IPV4_ETHERTYPE) {
            transition inner_ipv4;
        }
        if (hdr.inner_eth.ether_type == IPV6_ETHERTYPE) {
            transition inner_ipv6;
        }
        transition reject;
    }
    
    state inner_ipv4 {
        pkt.extract(hdr.inner_ipv4);
        if (hdr.inner_ipv4.protocol == ICMP_IPPROTO) {
            transition inner_icmp;
        }
        if (hdr.inner_ipv4.protocol == UDP_IPPROTO) {
            transition inner_udp;
        }
        if (hdr.inner_ipv4.protocol == TCP_IPPROTO) {
            transition inner_tcp;
        }
        transition accept;
    }

    state inner_ipv6 {
        pkt.extract(hdr.inner_ipv6);
        if (hdr.inner_ipv6.next_hdr == ICMP_IPPROTO) {
            transition inner_icmp;
        }
        if (hdr.inner_ipv6.next_hdr == UDP_IPPROTO) {
            transition inner_udp;
        }
        if (hdr.inner_ipv6.next_hdr == TCP_IPPROTO) {
            transition inner_tcp;
        }
        transition accept;
    }

    state inner_icmp {
        pkt.extract(hdr.inner_icmp);
        transition accept;
    }

    state inner_udp {
        pkt.extract(hdr.inner_udp);
        ingress.inner_src_port = hdr.udp.src_port;
        ingress.inner_dst_port = hdr.udp.dst_port;
        transition accept;
    }

    state inner_tcp {
        pkt.extract(hdr.inner_tcp);
        ingress.inner_src_port = hdr.tcp.src_port;
        ingress.inner_dst_port = hdr.tcp.dst_port;
        transition accept;
    }

}
