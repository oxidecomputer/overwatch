use anyhow::{anyhow, Result};
use bitvec::prelude::*;
use colored::Colorize;
use macaddr::MacAddr6;
use num_enum::TryFromPrimitive;
use std::net::{Ipv4Addr, Ipv6Addr};

pub fn bv_to_mac(bv: BitVec<u8, Msb0>) -> Result<MacAddr6> {
    let mut m: Vec<u8> = bv.into_vec();
    m.reverse();
    let m: [u8; 6] = match m.try_into() {
        Ok(m) => m,
        Err(_) => return Err(anyhow!("vec to mac")),
    };
    Ok(macaddr::MacAddr6::from(m))
}

pub fn bv_to_ipv4(bv: BitVec<u8, Msb0>) -> Result<Ipv4Addr> {
    let mut m: Vec<u8> = bv.into_vec();
    m.reverse();
    let m: [u8; 4] = match m.try_into() {
        Ok(m) => m,
        Err(_) => return Err(anyhow!("vec to mac")),
    };
    Ok(Ipv4Addr::from(m))
}

pub fn bv_to_ipv6(bv: BitVec<u8, Msb0>) -> Result<Ipv6Addr> {
    let mut m: Vec<u8> = bv.into_vec();
    m.reverse();
    let m: [u8; 16] = match m.try_into() {
        Ok(m) => m,
        Err(_) => return Err(anyhow!("vec to mac")),
    };
    Ok(Ipv6Addr::from(m))
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u16)]
pub enum Ethertype {
    IPv4 = 0x0800,
    IPv6 = 0x86dd,
    Arp = 0x0806,
    WOL = 0x0842,
    Vlan = 0x8100,
    Pbr = 0x88A8,
    QnQ = 0x9100,
    Sidecar = 0x901,
    Ethernet = 0x6558,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum IpProto {
    IPv6HeaderHopByHop = 0,
    ICMP = 1,
    IGMP = 2,
    GGP = 3,
    IPv4 = 4,
    Stream = 5,
    TCP = 6,
    CBT = 7,
    EGP = 8,
    IGP = 9,
    BbnRccMon = 10,
    NvpII = 11,
    PUP = 12,
    Argus = 13,
    Emcon = 14,
    Xnet = 15,
    Chaos = 16,
    UDP = 17,
    Mux = 18,
    DcnMeas = 19,
    Hmp = 20,
    Prm = 21,
    XnsIdp = 22,
    Trunk1 = 23,
    Trunk2 = 24,
    Leaf1 = 25,
    Leaf2 = 26,
    RDP = 27,
    Irtp = 28,
    IsoTp4 = 29,
    NetBlt = 30,
    MfeNsp = 31,
    MeritInp = 32,
    DCCP = 33,
    ThirdPartyConnectProtocol = 34,
    Idpr = 35,
    Xtp = 36,
    Ddp = 37,
    IdprCmtp = 38,
    TpPlusPlus = 39,
    Il = 40,
    IPv6 = 41,
    Sdrp = 42,
    IPv6RouteHeader = 43,
    IPv6FragmentationHeader = 44,
    Idrp = 45,
    RSVP = 46,
    GRE = 47,
    DSR = 48,
    BNA = 49,
    EncapsulatingSecurityPayload = 50,
    AuthenticationHeader = 51,
    INLSP = 52,
    Swipe = 53,
    NARP = 54,
    Mobile = 55,
    TLSP = 56,
    Skip = 57,
    ICMP6 = 58,
    IPv6NoNextHeader = 59,
    IPv6DestinationOptions = 60,
    AnyHostInternalProtocol = 61,
    CFTP = 62,
    AnyLocalNetwork = 63,
    SatExpak = 64,
    Krytolan = 65,
    RVD = 66,
    IPPC = 67,
    AnyDistributedFileSystem = 68,
    SatMon = 69,
    Visa = 70,
    IPCV = 71,
    CPNX = 72,
    CPHB = 73,
    WSN = 74,
    PVP = 75,
    BrSatMon = 76,
    SunNd = 77,
    WbMon = 78,
    WbExpak = 79,
    IsoIp = 80,
    VMTP = 81,
    SecureVmtp = 82,
    Vines = 83,
    TtpOrIptm = 84,
    NsfnetIgp = 85,
    DGP = 86,
    TCF = 87,
    EIGRP = 88,
    Ospfigp = 89,
    SpriteRpc = 90,
    LARP = 91,
    MTP = 92,
    AX25 = 93,
    IPIP = 94,
    MICP = 95,
    SccSp = 96,
    EtherIp = 97,
    Encap = 98,
    GMTP = 100,
    IFMP = 101,
    PNNI = 102,
    PIM = 103,
    ARIS = 104,
    SCPS = 105,
    QNX = 106,
    ActiveNetworks = 107,
    IpComp = 108,
    SitraNetworksProtocol = 109,
    CompaqPeer = 110,
    IpxInIp = 111,
    VRRP = 112,
    PGM = 113,
    AnyZeroHopProtocol = 114,
    Layer2TunnelingProtocol = 115,
    DDX = 116,
    IATP = 117,
    STP = 118,
    SRP = 119,
    UTI = 120,
    SimpleMessageProtocol = 121,
    SM = 122,
    PTP = 123,
    IsisOverIpv4 = 124,
    Fire = 125,
    CRTP = 126,
    Crudp = 127,
    Sscopmce = 128,
    IPLT = 129,
    SPS = 130,
    Pipe = 131,
    SCTP = 132,
    FC = 133,
    RsvpE2eIgnore = 134,
    MobilityHeader = 135,
    UDPLite = 136,
    MPLSInIp = 137,
    Manet = 138,
    HIP = 139,
    Shim6 = 140,
    WESP = 141,
    ROHC = 142,
    ExperimentalAndTesting0 = 253,
    ExperimentalAndTesting1 = 254,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum IcmpType {
    EchoReply = 0,
    Unassigned1 = 1,
    Unassigned2 = 2,
    DestinationUnreachable = 3,
    SourceQuench = 4,
    Redirect = 5,
    AlternateHostAddress = 6,
    Unassigned7 = 7,
    Echo = 8,
    RouterAdvertisement = 9,
    RouterSolicitation = 10,
    TimeExceeded = 11,
    ParameterProblem = 12,
    Timestamp = 13,
    TimestampReply = 14,
    InformationRequest = 15,
    InformationReply = 16,
    AddressMaskRequest = 17,
    AddressMaskReply = 18,
    Reserved19 = 19,
    Reserved20 = 20,
    Reserved21 = 21,
    Reserved22 = 22,
    Reserved23 = 23,
    Reserved24 = 24,
    Reserved25 = 25,
    Reserved26 = 26,
    Reserved27 = 27,
    Reserved28 = 28,
    Reserved29 = 29,
    Traceroute = 30,
    DatagramConversionError = 31,
    MobileHostRedirect = 32,
    IPv6WhereAreYou = 33,
    IPv6IAmHere = 34,
    MobileRegistrationRequest = 35,
    MobileRegistrationReply = 36,
    DomainNameRequest = 37,
    DomainNameReply = 38,
    SKIP = 39,
    Photuris = 40,
    Experimental41 = 41,
    ExtendedEchoRequest = 42,
    ExtendedEchoReply = 43,
    Experiment1 = 253,
    Experiment2 = 254,
    Reserved = 255,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum IcmpDUCode {
    NetUnreachable = 0,
    HostUnreachable = 1,
    ProtocolUnreachable = 2,
    PortUnreachable = 3,
    FragmentationNeededandDontFragmentWasSet = 4,
    SourceRouteFailed = 5,
    DestinationNetworkUnknown = 6,
    DestinationHostUnknown = 7,
    SourceHostIsolated = 8,
    NetworkAdministrativelyProhibited = 9,
    HostAdministrativelyProhibited = 10,
    NetworkUnreachableTOS = 11,
    DestinationHostUnreachableTOS = 12,
    CommunicationAdministrativelyProhibited = 13,
    HostPrecedenceViolation = 14,
    PrecedenceCutoffInEffect = 15,
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum IcmpRedirectMessageCode {
    RedirectDatagramForNetwork = 0,
    RedirectDatagramForHost = 1,
    RedirectDatagramForToSNetwork = 2,
    RedirectDatagramForToSHost = 3,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum IcmpTimeExceededCode {
    TTLExpiredInTransit = 0,
    FragmentReassemblyTimeExceeded = 1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum IcmpParameterProblemCode {
    PointerIndicatesError = 0,
    MissingRequiredOption = 1,
    BadLength = 2,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum IcmpExtendedEchoReplyCode {
    NoError = 0,
    MalformedQuery = 1,
    NoSuchInterface = 2,
    NoSuchTableEntry = 3,
    MultipleInterfacesSatisfyQuery = 4,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Icmp6Type {
    Reserved = 0,
    DestinationUnreachable = 1,
    PacketTooBig = 2,
    TimeExceeded = 3,
    ParameterProblem = 4,
    PrivateExperimentation100 = 100,
    PrivateExperimentation101 = 101,
    ReservedForExpansion = 127,
    EchoRequest = 128,
    EchoReply = 129,
    MulticastListenerQuery = 130,
    MulticastListenerReport = 131,
    MulticastListenerDone = 132,
    RouterSolicitation = 133,
    RouterAdvertisement = 134,
    NeighborSolicitation = 135,
    NeighborAdvertisement = 136,
    RedirectMessage = 137,
    RouterRenumbering = 138,
    ICMPNodeInformationQuery = 139,
    ICMPNodeInformationResponse = 140,
    InverseNeighborDiscoverySolicitationMessage = 141,
    InverseNeighborDiscoveryAdvertisementMessage = 142,
    Version2MulticastListenerReport = 143,
    HomeAgentAddressDiscoveryRequestMessage = 144,
    HomeAgentAddressDiscoveryReplyMessage = 145,
    MobilePrefixSolicitation = 146,
    MobilePrefixAdvertisement = 147,
    CertificationPathSolicitationMessage = 148,
    CertificationPathAdvertisementMessage = 149,
    ExperimentalMobilityProtocols = 150,
    MulticastRouterAdvertisement = 151,
    MulticastRouterSolicitation = 152,
    MulticastRouterTermination = 153,
    FMIPv6Messages = 154,
    RPLControlMessage = 155,
    ILNPv6LocatorUpdateMessage = 156,
    DuplicateAddressRequest = 157,
    DuplicateAddressConfirmation = 158,
    MPLControlMessage = 159,
    ExtendedEchoRequest = 160,
    ExtendedEchoReply = 161,
    PrivateExperimentation1 = 200,
    PrivateExperimentation2 = 201,
    ReservedForInformationalExpansion = 255,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Icmp6DUCode {
    NoRouteToDestination = 0,
    CommunicationWithDestinationAdministrativelyProhibited = 1,
    BeyondScopeOfSourceAddress = 2,
    AddressUnreachable = 3,
    PortUnreachable = 4,
    SourceAddressFailedIngressEgressPolicy = 5,
    RejectRouteToDestination = 6,
    ErrorInSourceRoutingHeader = 7,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Icmp6TimeExceededCode {
    HopLimitExceededInTransit = 0,
    FragmentReassemblyTimeExceeded = 1,
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Icmp6ParameterProblemCode {
    ErroneousHeaderFieldEncountered = 0,
    UnrecognizedNextHeaderTypeEncountered = 1,
    UnrecognizedIpv6OptionEncountered = 2,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Icmp6RouterRenumberingCode {
    RouterRenumberingCommand = 0,
    RouterRenumberingResult = 1,
    SequenceNumberReset = 255,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Icmp6NodeInformationQueryCode {
    DataFieldV6Address = 0,
    DataFieldNodeName = 1,
    DataFieldV4Address = 2,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Icmp6NodeInformationResponseCode {
    SuccessfulReply = 0,
    ReesponderRefuses = 1,
    QtypeUnknown = 2,
}

pub fn sep() {
    println!("{}", "=====|".dimmed());
}

pub fn headers(h: crate::headers_t) {
    if h.ethernet.isValid() {
        ethernet(h.ethernet);
    }
    if h.ipv4.isValid() {
        ipv4(h.ipv4);
        if h.icmp.isValid() {
            icmp(h.icmp);
        }
    } else if h.ipv6.isValid() {
        ipv6(h.ipv6);
        if h.icmp.isValid() {
            icmp6(h.icmp);
        }
    }
    if h.tcp.isValid() {
        tcp(h.tcp);
    }
    if h.udp.isValid() {
        udp(h.udp);
    }
    if h.geneve.isValid() {
        geneve(h.geneve);
        println!("{}", "-----|".dimmed());
    }
    if h.inner_eth.isValid() {
        ethernet(h.inner_eth);
    }
    if h.inner_ipv4.isValid() {
        ipv4(h.inner_ipv4);
        if h.inner_icmp.isValid() {
            icmp(h.inner_icmp);
        }
    } else if h.inner_ipv6.isValid() {
        ipv6(h.inner_ipv6);
        if h.inner_icmp.isValid() {
            icmp6(h.inner_icmp);
        }
    }
    if h.inner_tcp.isValid() {
        tcp(h.inner_tcp);
    }
    if h.inner_udp.isValid() {
        udp(h.inner_udp);
    }
    sep();
}

macro_rules! field {
    ($label:expr, $value:expr) => {
        format!("{} {}", $label.dimmed(), $value)
    };
}

macro_rules! layer {
    ($layer:expr) => {
        format!("{}{}", format!("{: <5}", $layer).green(), "|".dimmed())
    };
}

macro_rules! from_to {
    ($src:expr, $dst:expr) => {
        format!(
            "{} {} {}",
            $src.to_string().blue(),
            ">".dimmed(),
            $dst.to_string().blue(),
        )
    };
}

pub fn ethernet(h: crate::ethernet_h) {
    let Ok(dst) = bv_to_mac(h.dst) else { return };
    let Ok(src) = bv_to_mac(h.src) else { return };
    let et: u16 = h.ether_type.load_le();
    let et = match Ethertype::try_from(et) {
        Ok(h) => format!("{:?}", h).green(),
        _ => format!("0x{:04x}", et).green(),
    };
    println!(
        "{} {} {}",
        layer!("Eth"),
        from_to!(src, dst),
        field!("et", et),
    );
}

pub fn ipv4(h: crate::ipv4_h) {
    let Ok(src) = bv_to_ipv4(h.src) else { return };
    let Ok(dst) = bv_to_ipv4(h.dst) else { return };
    let _ver: u8 = h.version.load();
    let ihl: u8 = h.ihl.load();
    let ds: u8 = h.diffserv.load();
    let len: u16 = h.total_len.load_le();
    let id: u16 = h.identification.load_le();

    let flags: u8 = h.flags.load();
    let flags = match flags {
        0b010 => "DF",
        0b001 => "MF",
        0b011 => "DF|MF",
        _ => "",
    };

    let frag_off: u16 = h.frag_offset.load_le();
    let ttl: u8 = h.ttl.load();

    let proto: u8 = h.protocol.load();
    let proto = match IpProto::try_from(proto) {
        Ok(proto) => format!("{:?}", proto).green().to_string(),
        _ => format!("{}", proto),
    };

    let chk: u16 = h.hdr_checksum.load_le();

    println!(
        "{} {} {} {} {} {} {} {} {} {} {}",
        layer!("Ip4"),
        from_to!(src, dst),
        //field!("ver", ver),
        field!("ihl", ihl),
        field!("ds", ds),
        field!("len", len),
        field!("id", id),
        field!("flags", flags),
        field!("fo", frag_off),
        field!("ttl", ttl),
        field!("chk", chk),
        field!("proto", proto),
    );
}

pub fn ipv6(h: crate::ipv6_h) {
    let Ok(src) = bv_to_ipv6(h.src) else { return };
    let Ok(dst) = bv_to_ipv6(h.dst) else { return };
    let _ver: u8 = h.version.load();
    let tc: u8 = h.traffic_class.load();
    let fl: u32 = h.flow_label.load_le();
    let len: u16 = h.payload_len.load_le();

    let proto: u8 = h.next_hdr.load();
    let proto = match IpProto::try_from(proto) {
        Ok(proto) => format!("{:?}", proto).green().to_string(),
        _ => format!("{}", proto),
    };

    let ttl: u8 = h.hop_limit.load();

    println!(
        "{} {} {} {} {} {} {}",
        layer!("Ip6"),
        from_to!(src, dst),
        //field!("ver", ver),
        field!("tc", tc),
        field!("fl", fl),
        field!("len", len),
        field!("ttl", ttl),
        field!("proto", proto),
    )
}

pub fn tcp(h: crate::tcp_h) {
    let src: u16 = h.src_port.load_le();
    let dst: u16 = h.dst_port.load_le();
    let seq: u32 = h.seq_no.load_le();
    let ack: u32 = h.ack_no.load_le();
    let off: u8 = h.data_offset.load();
    let res: u8 = h.res.load();

    let mut flags = Vec::new();
    if let Some(&true) = h.flags.get(0).as_deref() {
        flags.push("CWR");
    }
    if let Some(&true) = h.flags.get(1).as_deref() {
        flags.push("ECE");
    }
    if let Some(&true) = h.flags.get(2).as_deref() {
        flags.push("URG");
    }
    if let Some(&true) = h.flags.get(4).as_deref() {
        flags.push("PSH");
    }
    if let Some(&true) = h.flags.get(5).as_deref() {
        flags.push("RST");
    }
    if let Some(&true) = h.flags.get(6).as_deref() {
        flags.push("SYN");
    }
    if let Some(&true) = h.flags.get(3).as_deref() {
        flags.push("ACK");
    }
    if let Some(&true) = h.flags.get(7).as_deref() {
        flags.push("FIN");
    }
    let flags = flags.join("|");

    let win: u16 = h.window.load_le();
    let chk: u16 = h.checksum.load_le();
    let urg: u16 = h.urgent_ptr.load_le();

    println!(
        "{} {} {} {} {} {} {} {} {} {}",
        layer!("TCP"),
        from_to!(src, dst),
        field!("seq", seq),
        field!("ack", ack),
        field!("off", off),
        field!("res", res),
        field!("flags", flags),
        field!("win", win),
        field!("chk", chk),
        field!("urg", urg),
    );
}

pub fn udp(h: crate::udp_h) {
    let src: u16 = h.src_port.load_le();
    let dst: u16 = h.dst_port.load_le();
    let len: u16 = h.len.load_le();
    let chk: u16 = h.checksum.load_le();

    println!(
        "{} {} {} {}",
        layer!("UDP"),
        from_to!(src, dst),
        field!("len", len),
        field!("chk", chk),
    )
}

pub fn icmp(h: crate::icmp_h) {
    let typ: u8 = h.typ.load_le();
    let code: u8 = h.code.load_le();

    let (typ, code) = match IcmpType::try_from(typ) {
        Ok(h) => {
            let code = match h {
                IcmpType::DestinationUnreachable => match IcmpDUCode::try_from(code) {
                    Ok(h) => format!("{:?}", h),
                    _ => format!("{}", typ),
                },
                IcmpType::Redirect => match IcmpRedirectMessageCode::try_from(code) {
                    Ok(h) => format!("{:?}", h),
                    _ => format!("{}", typ),
                },
                IcmpType::TimeExceeded => match IcmpTimeExceededCode::try_from(code) {
                    Ok(h) => format!("{:?}", h),
                    _ => format!("{}", typ),
                },
                IcmpType::ParameterProblem => match IcmpParameterProblemCode::try_from(code) {
                    Ok(h) => format!("{:?}", h),
                    _ => format!("{}", typ),
                },
                IcmpType::ExtendedEchoReply => match IcmpExtendedEchoReplyCode::try_from(code) {
                    Ok(h) => format!("{:?}", h),
                    _ => format!("{}", typ),
                },
                _ => format!("{}", code),
            };
            (format!("{:?}", h), code)
        }
        _ => (format!("{}", typ), format!("{}", code)),
    };

    let chk: u16 = h.hdr_checksum.load_le();
    println!(
        "{} {} {} {}",
        layer!("ICMP"),
        field!("type", typ),
        field!("code", code),
        field!("chk", chk),
    );
}

pub fn icmp6(h: crate::icmp_h) {
    let typ: u8 = h.typ.load_le();
    let code: u8 = h.code.load_le();

    let (typ, code) = match Icmp6Type::try_from(typ) {
        Ok(h) => {
            let code = match h {
                Icmp6Type::DestinationUnreachable => match Icmp6DUCode::try_from(code) {
                    Ok(h) => format!("{:?}", h),
                    _ => format!("{}", typ),
                },
                Icmp6Type::TimeExceeded => match Icmp6TimeExceededCode::try_from(code) {
                    Ok(h) => format!("{:?}", h),
                    _ => format!("{}", typ),
                },
                Icmp6Type::ParameterProblem => match Icmp6ParameterProblemCode::try_from(code) {
                    Ok(h) => format!("{:?}", h),
                    _ => format!("{}", typ),
                },
                Icmp6Type::RouterRenumbering => match Icmp6RouterRenumberingCode::try_from(code) {
                    Ok(h) => format!("{:?}", h),
                    _ => format!("{}", typ),
                },
                Icmp6Type::ICMPNodeInformationQuery => {
                    match Icmp6NodeInformationQueryCode::try_from(code) {
                        Ok(h) => format!("{:?}", h),
                        _ => format!("{}", typ),
                    }
                }
                Icmp6Type::ICMPNodeInformationResponse => {
                    match Icmp6NodeInformationResponseCode::try_from(code) {
                        Ok(h) => format!("{:?}", h),
                        _ => format!("{}", typ),
                    }
                }
                _ => format!("{}", code),
            };
            (format!("{:?}", h), code)
        }
        _ => (format!("{}", typ), format!("{}", code)),
    };

    let chk: u16 = h.hdr_checksum.load_le();
    println!(
        "{} {} {} {}",
        layer!("ICMP6"),
        field!("type", typ),
        field!("code", code),
        field!("chk", chk),
    );
}

pub fn geneve(h: crate::geneve_h) {
    let ver: u8 = h.version.load();
    let olen: u8 = h.opt_len.load();
    let ctrl: bool = *h.ctrl.get(0).unwrap();
    let crit: bool = *h.ctrl.get(0).unwrap();
    let proto: u16 = h.protocol.load_le();
    let proto = match Ethertype::try_from(proto) {
        Ok(h) => format!("{:?}", h).green(),
        _ => format!("0x{:04x}", proto).green(),
    };
    let vni: u32 = h.vni.load_be();

    println!(
        "{} {} {} {} {} {} {}",
        layer!("Gnv"),
        vni.to_string().blue(),
        field!("ver", ver),
        field!("olen", olen),
        field!("ctrl", ctrl),
        field!("crit", crit),
        field!("proto", proto),
    );
}
