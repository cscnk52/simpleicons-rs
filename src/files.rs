use std::{
    fs::{self, File, OpenOptions},
    io::{BufWriter, Cursor, Write},
    path::{Path, PathBuf},
};

use bytes::Bytes;
use flate2::bufread::GzDecoder;
use log::{debug, info, warn};
use tar::Archive;
use toml_edit::{value, DocumentMut};

use crate::{
    constants::{
        CRATES_ICON_FILE_NAME, CRATES_LIB_RELATIVE_PATH, CRATES_METADATA_FILE_NAME, CRATES_PACKAGE_PATH,
        LIB_DEFINE, NPM_PACKAGE_PATH, OUTPUT_DIR, SIMPLE_ICONS_NPM_JSON_FILENAME,
        SIMPLE_ICONS_NPM_JSON_RELATIVE_DIR,
    },
    simple_icons::file_to_json,
    types::{Icon, Icons, JsonIcons, NpmInformation},
};

pub async fn download_and_extract_npm_tarball(npm_package: &NpmInformation) {
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

pub fn generate_file() {
    let output_dir: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join(OUTPUT_DIR);
    debug!("output_dir: {:#?}", output_dir);
    let file_path: PathBuf = output_dir
        .join(CRATES_PACKAGE_PATH)
        .join(CRATES_LIB_RELATIVE_PATH)
        .join(CRATES_ICON_FILE_NAME);
    debug!("file_path: {:#?}", file_path);

    fs::create_dir_all(file_path.parent().unwrap()).unwrap();

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
    info!("file have been written in {}", CRATES_ICON_FILE_NAME);
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

pub fn replace_version(npm_info: &NpmInformation) {
    let crate_metedata_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(OUTPUT_DIR)
        .join(CRATES_PACKAGE_PATH)
        .join(CRATES_METADATA_FILE_NAME);

    let content = fs::read_to_string(&crate_metedata_path).unwrap();
    let mut updated = content.parse::<DocumentMut>().expect("Invalid doc");
    updated["package"]["version"] = value(&npm_info.version);

    fs::write(&crate_metedata_path, updated.to_string()).unwrap();
}
