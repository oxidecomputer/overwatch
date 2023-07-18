// Copyright 2023 Oxide Computer Company

use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::dump;

pub fn run(filename: &str) -> Result<()> {
    dump::sep();
    let hdrs = parse(filename)?;
    for (h, f) in hdrs {
        dump::headers(h, &f);
    }
    Ok(())
}

pub fn parse(filename: &str) -> Result<Vec<(crate::headers_t, Vec<u8>)>> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();

    let mut result = Vec::new();
    let mut frame: Vec<u8> = Vec::new();
    for line in lines {
        let mut line = line?;
        line.retain(|c| !c.is_whitespace());
        if line.is_empty() {
            let hdr = parse_frame(frame.as_mut_slice());
            result.push((hdr, frame.clone()));
            frame = Vec::new();
        } else {
            let data = hex::decode(&line)?;
            frame.extend_from_slice(&data);
        }
    }
    if !frame.is_empty() {
        let hdr = parse_frame(frame.as_mut_slice());
        result.push((hdr, frame.clone()));
    }
    Ok(result)
}

fn parse_frame(f: &mut [u8]) -> crate::headers_t {
    let mut pkt = p4rs::packet_in::new(f);
    let mut hdr = crate::headers_t::default();
    let mut md = crate::ingress_metadata_t::default();
    crate::parse_start(&mut pkt, &mut hdr, &mut md);
    hdr
}
