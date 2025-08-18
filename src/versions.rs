use std::error::Error;

use log::info;
use reqwest::Client;
use serde_json::Value;

use crate::{
    constants::{CRATES_IO_BASE_URL, NPM_BASE_URL},
    types::{CratesIOInformation, NpmInformation},
};

pub async fn get_npm_version(package: &str) -> Result<NpmInformation, Box<dyn Error>> {
    let package_url: String = format!("{}{}", NPM_BASE_URL, package);

    let response: Value = reqwest::get(package_url).await?.json().await?;

    let version: String = response["dist-tags"]["latest"]
        .as_str()
        .unwrap()
        .to_string();
    let tarball: String = response["versions"][&version]["dist"]["tarball"]
        .as_str()
        .unwrap()
        .to_string();

    let info: NpmInformation = NpmInformation {
        package: package.to_string(),
        version,
        tarball,
    };
    info!("npm version found: {}@{}", info.package, info.version);
    Ok(info)
}

pub async fn get_crates_io_version(package: &str) -> Result<CratesIOInformation, Box<dyn Error>> {
    let package_url = format!("{}{}", CRATES_IO_BASE_URL, package);

    let client: Client = Client::new();
    // crates.io require add "User-Agent" header to track usage
    let response: Value = client
        .get(package_url)
        .header("User-Agent", "github.com/cscnk52/simpleicons-rs-builder")
        .send()
        .await?
        .json()
        .await?;

    let version: String = response["crate"]["max_stable_version"]
        .as_str()
        .unwrap()
        .to_string();

    let info: CratesIOInformation = CratesIOInformation {
        package: package.to_string(),
        version,
    };
    info!("crates.io version found: {}@{}", info.package, info.version);
    Ok(info)
}
