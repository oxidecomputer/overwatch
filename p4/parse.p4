// Application layer protocol identifirs
#define ALP_GENEVE  8w0x1
#define ALP_DDM     8w0x2
#define ALP_BGP     8w0x3
#define ALP_HTTP    8w0x4

parser parse(
    packet_in pkt,
    out headers_t hdr,
    inout ingress_metadata_t ingress,
) {
    state start {
        pkt.extract(hdr.ethernet);
        if (hdr.ethernet.ether_type == 16w0x0800) {
            transition ipv4;
        }
        if (hdr.ethernet.ether_type == 16w0x86dd) {
            transition ipv6;
        }
        if (hdr.ethernet.ether_type == 16w0x0901) {
            transition sidecar;
        }
        if (hdr.ethernet.ether_type == 16w0x0806) {
            transition arp;
        }
        transition reject;
    }

    state sidecar {
        pkt.extract(hdr.sidecar);
        if (hdr.sidecar.sc_ether_type == 16w0x86dd) {
            transition ipv6;
        }
        if (hdr.sidecar.sc_ether_type == 16w0x0800) {
            transition ipv4;
        }
        transition reject;
    }

    state arp {
        pkt.extract(hdr.arp);
        transition accept;
    }

    state ipv6 {
        pkt.extract(hdr.ipv6);
        if (hdr.ipv6.next_hdr == 8w58) {
            transition icmp;
        }
        if (hdr.ipv6.next_hdr == 8w17) {
            transition udp;
        }
        if (hdr.ipv6.next_hdr == 8w6) {
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
        if (hdr.ipv4.protocol == 8w1) {
            transition icmp;
        }
        if (hdr.ipv4.protocol == 8w17) {
            transition udp;
        }
        if (hdr.ipv4.protocol == 8w6) {
            transition tcp;
        }
        transition accept;
    }

    state udp {
        pkt.extract(hdr.udp);
        if (hdr.udp.dst_port == 16w6081) {
            transition geneve;
        }
        transition accept;
    }

    state tcp {
        pkt.extract(hdr.tcp);
        transition accept;
    }

    state geneve {
        pkt.extract(hdr.geneve);
        ingress.alp = ALP_GENEVE;
        transition inner_eth;
    }

    state inner_eth {
        pkt.extract(hdr.inner_eth);
        if (hdr.inner_eth.ether_type == 16w0x0800) {
            transition inner_ipv4;
        }
        if (hdr.inner_eth.ether_type == 16w0x86dd) {
            transition inner_ipv6;
        }
        transition reject;
    }
    
    state inner_ipv4 {
        pkt.extract(hdr.inner_ipv4);
        if (hdr.inner_ipv4.protocol == 8w17) {
            transition inner_udp;
        }
        if (hdr.inner_ipv4.protocol == 8w6) {
            transition inner_tcp;
        }
        transition accept;
    }

    state inner_ipv6 {
        pkt.extract(hdr.inner_ipv6);
        if (hdr.inner_ipv6.next_hdr == 8w17) {
            transition inner_udp;
        }
        if (hdr.inner_ipv6.next_hdr == 8w6) {
            transition inner_tcp;
        }
        transition accept;
    }

    state inner_udp {
        pkt.extract(hdr.inner_udp);
        transition accept;
    }

    state inner_tcp {
        pkt.extract(hdr.inner_tcp);
        transition accept;
    }

}
