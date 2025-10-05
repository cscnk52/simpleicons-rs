use std::{env, fs::OpenOptions, io::Write};

use clap::Parser;
use log::info;
use semver::Version;

use crate::{
    files::{download_and_extract_npm_tarball, generate_file, replace_version},
    types::{CratesIOInformation, NpmInformation},
    versions::{get_crates_io_version, get_npm_version},
};

mod constants;
mod files;
mod simple_icons;
mod types;
mod versions;

#[derive(Parser, Debug)]
#[command(version, long_about = None)]
struct Args {
    /// Force generate crate, ignore version compare
    #[arg(long, short = 'f')]
    force: bool,
}

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Stdout)
        .filter_level(log::LevelFilter::Debug)
        .init();
    #[cfg(not(debug_assertions))]
    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Stdout)
        .filter_level(log::LevelFilter::Info)
        .init();

    let args = Args::parse();

    let npm_info: NpmInformation = get_npm_version("simple-icons").await.unwrap();
    let crate_info: CratesIOInformation = get_crates_io_version("simpleicons-rs").await.unwrap();

    let npm_version: Version = Version::parse(&npm_info.version).unwrap();
    let crate_version: Version = Version::parse(&crate_info.version).unwrap();

    let github_actions_env = env::var("GITHUB_OUTPUT").unwrap();
    let mut f = OpenOptions::new()
        .append(true)
        .open(github_actions_env)
        .unwrap();

    if !args.force && crate_version >= npm_version {
        info!("crate have update with npm, aboard update");
        writeln!(f, "updated=false").unwrap();
        return;
    }

    writeln!(f, "updated=true").unwrap();
    writeln!(f, "version={}", npm_info.version).unwrap();

    download_and_extract_npm_tarball(&npm_info).await;

    generate_file();
    replace_version(&npm_info);
}
