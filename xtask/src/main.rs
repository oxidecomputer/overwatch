// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2025 Oxide Computer Company

use anyhow::Context;
use anyhow::Result;
use cargo_metadata::Metadata;
use clap::Parser;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;
use std::sync::OnceLock;

static PUBLISHER: &str = "helios-dev";
static API_VSN: u32 = 1;

static METADATA: OnceLock<Metadata> = OnceLock::new();
fn cargo_meta() -> &'static Metadata {
    METADATA
        .get_or_init(|| cargo_metadata::MetadataCommand::new().exec().unwrap())
}

#[derive(Debug, Parser)]
enum Xtask {
    /// produce an illumos package for overwatch
    Package,
}

fn main() -> anyhow::Result<()> {
    let cmd = Xtask::parse();
    match cmd {
        Xtask::Package => {
            cmd_package()?;
            Ok(())
        }
    }
}

// Remove a file or directory, if it exists.
fn remove_if_exists<P: AsRef<Path>>(path: P) -> Result<()> {
    match fs::metadata(&path) {
        Ok(metadata) => {
            let res = if metadata.is_file() {
                fs::remove_file(&path)
            } else if metadata.is_dir() {
                fs::remove_dir_all(&path)
            } else {
                anyhow::bail!("unknown file type")
            };

            match res {
                Ok(_) => Ok(()),
                Err(e) => {
                    anyhow::bail!("failed to remove: {}", e)
                }
            }
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                Ok(())
            } else {
                anyhow::bail!("failed to remove: {}", e)
            }
        }
    }
}

fn cmd_package() -> Result<()> {
    let meta = cargo_meta();

    println!("Remove and make new directories for packaging");
    // This is the directory where the packages are made
    // Start by clearing out any temporary directories and ensuring
    // that files we use are created fresh
    let pkg_dir = meta.workspace_root.join("pkg");
    remove_if_exists(&pkg_dir)?;

    let proto_dir = pkg_dir.join("proto");
    let package_dir = pkg_dir.join("packages");
    fs::create_dir_all(&package_dir)?;

    let base_p5m = pkg_dir.join("overwatch.base.p5m");
    let mut file = fs::File::create(&base_p5m).unwrap();

    let generate_p5m = pkg_dir.join("overwatch.generate.p5m");
    let generate_file = fs::File::create(&generate_p5m).unwrap();

    // Create the final file.
    let final_p5m = pkg_dir.join("overwatch.final.p5m");
    let mut output_file = fs::File::create(&final_p5m)?;

    // Begin the process of creating our package.
    println!("Get git rev-list count");
    let output = Command::new("git")
        .args(["rev-list", "--count", "HEAD"])
        .output()
        .expect("Failed to execute git command");

    // Check if the command succeeded
    if !output.status.success() {
        anyhow::bail!("failed to run git (status {})", output.status)
    }
    let commit_count: u64 = String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse()
        .expect("Failed to parse git output as a number");

    println!("Git rev-list count: {}", commit_count);

    let proto_bin_dir = proto_dir.join("usr").join("bin");
    fs::create_dir_all(&proto_bin_dir)?;

    // cp target/release/overwatch proto/usr/bin/
    let source = meta
        .workspace_root
        .join("target")
        .join("release")
        .join("overwatch");

    let destination = proto_bin_dir.join("overwatch");

    println!("Copy {:?} to {:?}", source, destination);
    fs::copy(&source, &destination)?;

    // Define the file content as a multi-line string
    let content = format!("\
set name=pkg.fmri value=pkg://{}/overwatch@0.{}.{}
set name=pkg.summary value=\"Overwatch packet inspection tool\"
set name=info.classification value=org.opensolaris.category.2008:Network/Application
set name=variant.opensolaris.zone value=global value=nonglobal
set name=variant.arch value=i386
file NOHASH group=bin mode=0755 owner=root path=usr/bin/overwatch
", PUBLISHER, API_VSN, commit_count);

    file.write_all(content.as_bytes()).unwrap();

    println!("Created {}", base_p5m);

    // pkgdepend generate -d proto overwatch.base.p5m > overwatch.generate.p5m
    let mut cmd = Command::new("pkgdepend");
    let status = cmd
        .arg("generate")
        .arg("-d")
        .arg(&proto_dir)
        .arg(&base_p5m)
        .current_dir(meta.workspace_root.join(&pkg_dir))
        .stdout(Stdio::from(generate_file))
        .spawn()
        .context("failed to spawn pkgdepend 1")?
        .wait()
        .context("Failed to run pkgdepend")?;

    if !status.success() {
        anyhow::bail!("failed to run (status {status})")
    }

    // pkgdepend resolve -d packages -s resolve.p5m overwatch.generate.p5m
    let mut cmd = Command::new("pkgdepend");
    cmd.arg("resolve")
        .arg("-d")
        .arg(&package_dir)
        .arg("-s")
        .arg("resolve.p5m")
        .arg(&generate_p5m)
        .current_dir(meta.workspace_root.join(&pkg_dir))
        .output_nocapture()?;

    // cat overwatch.base.p5m
    //    packages/overwatch.generate.p5m.resolve.p5m > overwatch.final.p5m
    let base = pkg_dir.join("overwatch.base.p5m");
    let base_data = fs::read(base)?;

    let resolve = pkg_dir.join("packages/overwatch.generate.p5m.resolve.p5m");
    let resolve_data = fs::read(resolve)?;

    output_file.write_all(&base_data)?;
    output_file.write_all(&resolve_data)?;

    println!("Successfully created {:?}", final_p5m);

    // pkgrepo create $REPO
    let repo_dir = package_dir.join("repo");
    let mut cmd = Command::new("pkgrepo");
    cmd.arg("create")
        .arg(&repo_dir)
        .current_dir(meta.workspace_root.join(&pkg_dir))
        .output_nocapture()?;

    // pkgrepo add-publisher -s $REPO PUBLISHER
    let mut cmd = Command::new("pkgrepo");
    cmd.arg("add-publisher")
        .arg("-s")
        .arg(&repo_dir)
        .arg(PUBLISHER)
        .current_dir(meta.workspace_root.join(&pkg_dir))
        .output_nocapture()?;

    // pkgsend publish -d proto -s repo overwatch_final_p5m
    let mut cmd = Command::new("pkgsend");
    cmd.arg("publish")
        .arg("-d")
        .arg(&proto_dir)
        .arg("-s")
        .arg(&repo_dir)
        .arg(final_p5m)
        .current_dir(meta.workspace_root.join(&pkg_dir))
        .output_nocapture()?;

    // pkgrecv -a -d overwatch-0.$API_VSN.$COMMIT_COUNT.p5p -s $REPO
    //    -v -m latest '*'
    let final_p5p =
        repo_dir.join(format!("overwatch-0.{}.{}.p5p", API_VSN, commit_count));
    let mut cmd = Command::new("pkgrecv");
    cmd.arg("-a")
        .arg("-d")
        .arg(final_p5p)
        .arg("-s")
        .arg(&repo_dir)
        .arg("-v")
        .arg("-m")
        .arg("latest")
        .arg("*")
        .current_dir(meta.workspace_root.join(&pkg_dir))
        .output_nocapture()?;

    Ok(())
}

trait CommandNoCapture {
    fn output_nocapture(&mut self) -> Result<()>;
}

impl CommandNoCapture for Command {
    fn output_nocapture(&mut self) -> Result<()> {
        let status = self
            .spawn()
            .context("failed to spawn child cargo invocation")?
            .wait()
            .context("failed to await child cargo invocation")?;

        if status.success() {
            Ok(())
        } else {
            anyhow::bail!("failed to run (status {status})")
        }
    }
}
