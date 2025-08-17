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

    let npm_info: NpmInformation = get_npm_version("simple-icons").await.unwrap();
    let crate_info: CratesIOInformation = get_crates_io_version("simpleicons-rs").await.unwrap();

    let npm_version: Version = Version::parse(&npm_info.version).unwrap();
    let crate_version: Version = Version::parse(&crate_info.version).unwrap();

    if crate_version >= npm_version {
        info!("crate have update with npm, aboard update");
        return;
    }

    download_and_extract_npm_tarball(&npm_info).await;

    generate_file();
    replace_version(&npm_info);
}
