use std::{
    error::Error,
    fs::File,
    io::{BufWriter, Cursor, Write},
    path::{Path, PathBuf},
};

use bytes::Bytes;
use flate2::read::GzDecoder;
use log::info;
use reqwest::Client;
use serde_json::Value;
use tar::Archive;

mod constrants;
use constrants::*;

mod types;
use types::*;

async fn get_npm_version(package: &str) -> Result<NpmInformation, Box<dyn Error>> {
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

async fn download_and_extract_npm_tarball(npm_package: NpmInformation) {
    let dir: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join(OUTPUT_DIR);
    // if !dir.exists() {
    //     let _ = std::fs::create_dir(&dir);
    // }

    let response_bytes: Bytes = reqwest::get(&npm_package.tarball)
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    // 'bytes::Bytes' can't direct read by 'GzDecoder', need 'Cursor' work-around
    let cursor = Cursor::new(response_bytes);
    let tar = GzDecoder::new(cursor);
    let mut archive = Archive::new(tar);
    let _ = archive.unpack(&dir);
    info!("npm package already download and decompress under 'build' folder");
}

async fn get_crates_io_version(package: &str) -> Result<CratesIOInformation, Box<dyn Error>> {
    let package_url = format!("{}{}", CRATES_IO_BASE_URL, package);

    let client: Client = Client::new();
    // crates.io require add "User-Agent" header to track usage
    let response: Value = client
        .get(package_url)
        .header("User-Agent", "simple-icons-rs")
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

fn generate_file() {
    let output_dir: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join(OUTPUT_DIR);
    let file_path: PathBuf = output_dir.join(OUTPUT_FILE);

    if !output_dir.exists() {
        std::fs::create_dir(&output_dir).unwrap();
    }
    let file: File = File::create_new(file_path).unwrap();

    let mut content: BufWriter<File> = BufWriter::new(file);

    write!(content, "{}", LIB_DEFINE).unwrap();
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_default_env()
        // change log level here
        .target(env_logger::Target::Stdout)
        .filter_level(log::LevelFilter::Debug)
        .init();

    let npm_info: NpmInformation = get_npm_version("simple-icons").await.unwrap();
    download_and_extract_npm_tarball(npm_info).await;

    generate_file();
}
