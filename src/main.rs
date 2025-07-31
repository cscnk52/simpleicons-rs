use std::{
    error::Error,
    path::{Path, PathBuf},
};

use log::info;
use reqwest::Client;
use serde_json::Value;

async fn get_npm_version(package: &str) -> Result<String, Box<dyn Error>> {
    const NPM_BASE_URL: &str = "https://registry.npmjs.org/";
    let package_url: String = format!("{}{}", NPM_BASE_URL, package);

    let response: Value = reqwest::get(package_url).await?.json().await?;

    if let Some(version) = response["dist-tags"]["latest"].as_str() {
        info!("npm version find: {package}@{version}");
        Ok(version.to_string())
    } else {
        Err("Can't get npm version".into())
    }
}

async fn get_crates_io_version(package: &str) -> Result<String, Box<dyn Error>> {
    const CRATES_IO_BASE_URL: &str = "https://crates.io/api/v1/crates/";
    let package_url = format!("{}{}", CRATES_IO_BASE_URL, package);

    let client = Client::new();
    // crates.io require add "User-Agent" header to track usage
    let response: Value = client
        .get(package_url)
        .header("User-Agent", "simple-icons-rs")
        .send()
        .await?
        .json()
        .await?;

    if let Some(version) = response["crate"]["max_stable_version"].as_str() {
        log::info!("crates.io version find: {package}@{version}");
        Ok(version.to_string())
    } else {
        Err("Can't get crates.io version".into())
    }
}

fn generate_file() {
    let output_dir: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join("build");

    // TODO
    println!("{:?}", output_dir);
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_default_env()
        // change log level here
        .target(env_logger::Target::Stdout)
        .filter_level(log::LevelFilter::Debug)
        .init();

    println!("{}", get_npm_version("simple-icons").await.unwrap());
    println!("{}", get_crates_io_version("tokio").await.unwrap());

    generate_file();
}
