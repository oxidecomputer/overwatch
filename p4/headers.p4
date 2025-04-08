// Copyright 2023 Oxide Computer Company

header sidecar_h {
    bit<8> sc_code;
    bit<8> sc_pad;
    bit<16> sc_ingress;
    bit<16> sc_egress;
    bit<16> sc_ether_type;
    bit<128> sc_payload;
}

header ethernet_h {
    bit<48> dst;
    bit<48> src;
    bit<16> ether_type;
}

header vlan_h {
    bit<3> pcp;
    bit<1> dei;
    bit<12> vid;
    bit<16> ether_type;
}

header lldp_h {
}

header ipv6_h {
    bit<4>      version;
    bit<8>      traffic_class;
    bit<20>     flow_label;
    bit<16>     payload_len;
    bit<8>      next_hdr;
    bit<8>      hop_limit;
    bit<128>    src;
    bit<128>    dst;
}

header ipv4_h {
    bit<4>      version;
    bit<4>      ihl;
    bit<8>      diffserv;
    bit<16>     total_len;
    bit<16>     identification;
    bit<3>      flags;
    bit<13>     frag_offset;
    bit<8>      ttl;
    bit<8>      protocol;
    bit<16>     hdr_checksum;
    bit<32>     src;
    bit<32>     dst;
}

header udp_h {
    bit<16> src_port;
    bit<16> dst_port;
    bit<16> len;
    bit<16> checksum;
}

header tcp_h {
    bit<16> src_port;
    bit<16> dst_port;
    bit<32> seq_no;
    bit<32> ack_no;
    bit<4> data_offset;
    bit<4> res;
    bit<8> flags;
    bit<16> window;
    bit<16> checksum;
    bit<16> urgent_ptr;
}

header icmp_h {
    bit<8> typ;
    bit<8> code;
    bit<16> hdr_checksum;
}

header echo_h {
    bit<16> id;
    bit<16> seq;
}

header geneve_h {
    bit<2> version;
    bit<6> opt_len;
    bit<1> ctrl;
    bit<1> crit;
    bit<6> reserved;
    bit<16> protocol;
    bit<24> vni;
    bit<8> reserved2;
}

header arp_h {
	bit<16>		hw_type;
	bit<16>		proto_type;
	bit<8>		hw_addr_len;
	bit<8>		proto_addr_len;
	bit<16>		opcode;

	// In theory, the remaining fields should be <varbit>
	// based on the the two x_len fields.
	bit<48> sender_mac;
	bit<32> sender_ip;
	bit<48> target_mac;
	bit<32> target_ip;
}

header ddm_h {
    bit<8> next_header;
    bit<8> header_length;
    bit<8> version;
    bit<1> ack;
    bit<7> reserved;
}

header ddm_element_t {
    bit<8> id;
    bit<24> timestamp;
}

header ddm_discovery_h {
    bit<8> version;
    bit<8> flags;
    bit<8> router_kind;
    bit<8> hostname_len;
}

header bfd_h {
    bit<3> version;
    bit<5> diag;
    bit<2> status;
    bit<1> poll;
    bit<1> fin;
    bit<1> control_plane_independent;
    bit<1> authentication_present;
    bit<1> demand;
    bit<1> multipoint;
    bit<8> detect_mult;
    bit<8> len;
    bit<32> my_discriminator;
    bit<32> your_discriminator;
    bit<32> desired_min_tx_interval;
    bit<32> required_min_tx_interval;
    bit<32> required_min_echo_rx_interval;
}

struct headers_t {
    // L2
    ethernet_h ethernet;
    vlan_h vlan;
    lldp_h lldp;

    // L2.5
    sidecar_h sidecar;
    arp_h arp;

    // L3
    ipv4_h ipv4;
    ipv6_h ipv6;

    // L4
    icmp_h icmp;
    echo_h echo;
    tcp_h tcp;
    udp_h udp;

    // App
    ddm_discovery_h ddm_discovery;
    bfd_h bfd;

    // Tunnel
    geneve_h geneve;
    ethernet_h inner_eth;
    arp_h inner_arp;
    ipv4_h inner_ipv4;
    ipv6_h inner_ipv6;
    tcp_h inner_tcp;
    udp_h inner_udp;
    icmp_h inner_icmp;
    echo_h inner_echo;
}
