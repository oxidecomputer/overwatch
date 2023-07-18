// Copyright 2023 Oxide Computer Company

#![allow(clippy::too_many_arguments)]

use anyhow::Result;
use clap::Parser;

mod cli;
mod dump;
mod hex_read;
mod link;
mod snoop;

p4_macro::use_p4!(p4 = "p4/overwatch.p4", pipeline_name = "overwatch");

fn main() -> Result<()> {
    let args = cli::Cli::parse();

    match &args.command {
        cli::Command::Snoop(s) => snoop::run(s),
        cli::Command::HexRead(hr) => hex_read::run(&hr.file),
    }
}
