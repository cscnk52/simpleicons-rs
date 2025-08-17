use std::{
    env,
    error::Error,
    fs::{self, File, OpenOptions},
    io::{BufWriter, Cursor, Write},
    path::{Path, PathBuf},
};

use bytes::Bytes;
use flate2::read::GzDecoder;
use log::{debug, info, warn};
use reqwest::Client;
use serde_json::Value;
use tar::Archive;

use crate::{
    constants::{
        CRATES_FILE_NAME, CRATES_IO_BASE_URL, CRATES_LIB_RELATIVE_PATH, CRATES_METADATA_FILE_NAME,
        CRATES_PACKAGE_PATH, LIB_DEFINE, NPM_BASE_URL, NPM_PACKAGE_PATH, OUTPUT_DIR,
        SIMPLE_ICONS_NPM_JSON_FILENAME, SIMPLE_ICONS_NPM_JSON_RELATIVE_DIR,
    },
    simple_icons::file_to_json,
    types::{CratesIOInformation, Icon, Icons, JsonIcons, NpmInformation},
};

mod constants;
mod simple_icons;
mod types;

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

async fn download_and_extract_npm_tarball(npm_package: &NpmInformation) {
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
    info!(
        "npm package already download and decompress under '{}' folder",
        OUTPUT_DIR
    );
}

async fn get_crates_io_version(package: &str) -> Result<CratesIOInformation, Box<dyn Error>> {
    let package_url = format!("{}{}", CRATES_IO_BASE_URL, package);

    let client: Client = Client::new();
    // crates.io require add "User-Agent" header to track usage
    let response: Value = client
        .get(package_url)
        .header("User-Agent", "github.com/cscnk52/simple-icons-rs")
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
    debug!("output_dir: {:#?}", output_dir);
    let file_path: PathBuf = output_dir
        .join(CRATES_PACKAGE_PATH)
        .join(CRATES_LIB_RELATIVE_PATH)
        .join(CRATES_FILE_NAME);
    debug!("file_path: {:#?}", file_path);

    fs::create_dir_all(&file_path.parent().unwrap()).unwrap();

    let file: File = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        // remove existed content if exist
        .truncate(true)
        .open(&file_path)
        .unwrap();

    let simple_icons_json_path = output_dir
        .join(NPM_PACKAGE_PATH)
        .join(SIMPLE_ICONS_NPM_JSON_RELATIVE_DIR)
        .join(SIMPLE_ICONS_NPM_JSON_FILENAME);
    debug!("simple_icons_json_path: {:#?}", simple_icons_json_path);

    let json = file_to_json(simple_icons_json_path).unwrap();

    assert!(
        json.is_array(),
        "json should be array, but actually be {}",
        json
    );

    let json_icons: JsonIcons = serde_json::from_value(json.clone()).expect("Failed to parse json");

    let icons: Icons = json_icons
        .into_iter()
        .map(|icon| {
            let svg = read_svg(&icon.slug);
            Icon {
                title: icon.title,
                hex: icon.hex,
                source: icon.source,
                slug: icon.slug,
                svg,
            }
        })
        .collect();

    let mut content: BufWriter<File> = BufWriter::new(file);

    writeln!(content, "{}", LIB_DEFINE).unwrap();
    for icon in &icons {
        writeln!(
            content,
            "pub const SI{}: Icon = Icon {{\n\ttitle: \"{}\",\n\tslug: \"{}\",\n\thex: \"{}\",\n\tsource: \"{}\",\n\tsvg: r###\"{}\"###,}};",
            icon.slug.to_uppercase(),
            icon.title,
            icon.slug,
            icon.hex,
            icon.source,
            icon.svg,
        )
        .unwrap();
    }

    writeln!(
        content,
        "pub fn slug(slug: &str) -> Option<&'static Icon> {{\n\tmatch slug {{"
    )
    .unwrap();

    for icon in &icons {
        let name = format!("SI{}", icon.slug.to_uppercase());
        writeln!(content, "\t\t\"{}\" => Some(&{}),", icon.slug, name).unwrap();
    }

    writeln!(content, "\t\t_ => None,\n\t}}\n}}").unwrap();
    info!("file have been written in {}", CRATES_FILE_NAME);
}

pub fn read_svg(slug: &str) -> String {
    let path: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(OUTPUT_DIR)
        .join(NPM_PACKAGE_PATH)
        .join("icons")
        .join(format!("{}.svg", slug));

    fs::read_to_string(path).unwrap_or_else(|_| {
        warn!("Failed to read SVG with slug: {}", slug);
        String::new()
    })
}

fn replace_version(npm_info: &NpmInformation) {
    let crate_metedata_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(OUTPUT_DIR)
        .join(CRATES_PACKAGE_PATH)
        .join(CRATES_METADATA_FILE_NAME);

    let content = fs::read_to_string(&crate_metedata_path).unwrap();

    let updated = content.replace("0.0.1", &npm_info.version);

    fs::write(&crate_metedata_path, updated).unwrap();
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

    let npm_info: NpmInformation = get_npm_version("simple-icons").await.unwrap();
    download_and_extract_npm_tarball(&npm_info).await;

    generate_file();
    replace_version(&npm_info);
}
