use crate::dump::{Alp, IpProto};
use clap::{Parser, Subcommand};
use std::net::IpAddr;

pub fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .header(anstyle::Style::new().bold().underline().fg_color(Some(
            anstyle::Color::Rgb(anstyle::RgbColor(245, 207, 101)),
        )))
        .literal(anstyle::Style::new().bold().fg_color(Some(
            anstyle::Color::Rgb(anstyle::RgbColor(72, 213, 151)),
        )))
        .invalid(anstyle::Style::new().bold().fg_color(Some(
            anstyle::Color::Rgb(anstyle::RgbColor(72, 213, 151)),
        )))
        .valid(anstyle::Style::new().bold().fg_color(Some(
            anstyle::Color::Rgb(anstyle::RgbColor(72, 213, 151)),
        )))
        .usage(anstyle::Style::new().bold().fg_color(Some(
            anstyle::Color::Rgb(anstyle::RgbColor(245, 207, 101)),
        )))
        .error(anstyle::Style::new().bold().fg_color(Some(
            anstyle::Color::Rgb(anstyle::RgbColor(232, 104, 134)),
        )))
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, styles = get_styles())]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand, Debug)]
#[command(styles = get_styles())]
pub enum Command {
    /// Snoop a packet stream.
    Snoop(Snoop),

    /// Read and display packets in hex format.
    HexRead(HexRead),
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, styles = get_styles())]
pub struct Snoop {
    /// The data link to snoop.
    pub link: String,

    /// Filter on the provided source IPs.
    #[arg(long)]
    pub ip_src: Vec<IpAddr>,

    /// Filter on the provided destination IPs.
    #[arg(long)]
    pub ip_dst: Vec<IpAddr>,

    /// Filter on the provided IPs for source or destination.
    #[arg(long)]
    pub ip_host: Vec<IpAddr>,

    /// Filter on the provided IP protocol types.
    #[arg(long)]
    pub ip_proto: Vec<IpProto>,

    /// Filter on the provided application layer protocol types.
    #[arg(long)]
    pub alp: Vec<Alp>,

    /// Filter for only IPv4 traffic.
    #[arg(long)]
    pub v4: bool,

    /// Filter for only IPv6 traffic.
    #[arg(long)]
    pub v6: bool,

    /// Filter for only ARP traffic.
    #[arg(long)]
    pub arp: bool,

    /// Filter on the provided source IPs for encapsulated packets.
    #[arg(long)]
    pub inner_ip_src: Vec<IpAddr>,

    /// Filter on the provided destination IPs for encapsulated packets.
    #[arg(long)]
    pub inner_ip_dst: Vec<IpAddr>,

    /// Filter on the provided IPs for source or destination for encapsulated
    /// packets.
    #[arg(long)]
    pub inner_ip_host: Vec<IpAddr>,

    /// Filter on the provided IP protocol types for encapsulated packets.
    #[arg(long)]
    pub inner_ip_proto: Vec<IpProto>,

    /// Filter on the provided application layer protocol types for encapsulated
    /// packets.
    #[arg(long)]
    pub inner_alp: Vec<Alp>,

    /// Filter for only IPv4 traffic for encapsulated packets.
    #[arg(long)]
    pub inner_v4: bool,

    /// Filter for only IPv6 traffic for encapsulated packets.
    #[arg(long)]
    pub inner_v6: bool,

    /// Filter for only ARP traffic for encapsulated packets.
    #[arg(long)]
    pub inner_arp: bool,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, styles = get_styles())]
pub struct HexRead {
    /// File containing the hex encoded packets.
    pub file: String,
}
