use std::{
    collections::HashMap,
    env,
    fmt::Write as _,
    fs::{self, File, OpenOptions},
    io::{Cursor, Write},
    path::{Path, PathBuf},
};

use flate2::bufread::GzDecoder;
use reqwest::blocking::{self, Client};
use semver::Version;
use serde::Deserialize;
use serde_json::Value;
use tar::Archive;
use thiserror::Error;
use toml_edit::{value, DocumentMut};

const PACKAGE_NAME: &str = "simpleicons-rs";
const SIMPLE_ICONS_PACKAGE: &str = "simple-icons";

const NPM_BASE_URL: &str = "https://registry.npmjs.org/";
const CRATES_IO_BASE_URL: &str = "https://crates.io/api/v1/crates/";

const OUTPUT_DIR: &str = "npm";
const NPM_PACKAGE_PATH: &str = "package";
const SIMPLE_ICONS_NPM_JSON_RELATIVE_DIR: &str = "data";
const SIMPLE_ICONS_NPM_JSON_FILENAME: &str = "simple-icons.json";

const CRATES_METADATA_FILE_NAME: &str = "Cargo.toml";
const CRATES_ICON_FILE_NAME: &str = "icons.rs";
const CRATES_LIB_RELATIVE_PATH: &str = "src";

const LIB_DEFINE: &str = r###"#[derive(Debug, Clone, Copy)]
pub struct Icon {
    pub title: &'static str,
    pub slug: &'static str,
    pub hex: &'static str,
    pub source: &'static str,
    pub svg: &'static str,
}"###;

type BuildResult<T> = Result<T, BuildError>;

#[derive(Debug, Error)]
enum BuildError {
    #[error(transparent)]
    EnvVar(#[from] env::VarError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Semver(#[from] semver::Error),
    #[error(transparent)]
    Toml(#[from] toml_edit::TomlError),
    #[error(transparent)]
    Fmt(#[from] std::fmt::Error),

    #[error("npm latest version not found")]
    MissingNpmLatestVersion,
    #[error("npm tarball url not found for version {0}")]
    MissingNpmTarball(String),
    #[error("crates.io max_stable_version not found")]
    MissingCratesMaxStableVersion,
    #[error("{0}")]
    MissingParentDirectory(&'static str),
}

#[derive(Debug)]
struct NpmInformation {
    package: String,
    version: String,
    tarball: String,
}

#[derive(Debug)]
struct CratesIOInformation {
    version: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct JsonIcon {
    title: String,
    slug: String,
    hex: String,
    source: String,
    #[serde(default)]
    guidelines: Option<String>,
    #[serde(default)]
    license: Option<License>,
    #[serde(default)]
    aliases: Option<Aliases>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct License {
    #[serde(rename = "type")]
    license_type: String,
    #[serde(default)]
    url: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct Aliases {
    #[serde(default)]
    aka: Option<Vec<String>>,
    #[serde(default)]
    dup: Option<Vec<DuplicateAlias>>,
    #[serde(default)]
    loc: Option<HashMap<String, String>>,
    #[serde(default)]
    old: Option<Vec<String>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct DuplicateAlias {
    title: String,
    #[serde(default)]
    hex: Option<String>,
    #[serde(default)]
    guidelines: Option<String>,
    #[serde(default)]
    loc: Option<HashMap<String, String>>,
}

#[derive(Debug)]
struct IconData {
    title: String,
    slug: String,
    hex: String,
    source: String,
    svg: String,
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=GITHUB_OUTPUT");

    if let Err(err) = run() {
        let icons_path = env::var("CARGO_MANIFEST_DIR")
            .ok()
            .map(PathBuf::from)
            .map(|dir| icons_file_path(&dir));

        match icons_path {
            Some(path) if path.exists() => {
                cargo_warning(&format!("simple-icons update skipped: {err}"));
            }
            Some(path) => {
                panic!("failed to generate {}: {err}", path.display());
            }
            None => {
                panic!("failed to run build script: {err}");
            }
        }
    }
}

fn run() -> BuildResult<()> {
    let manifest_dir = manifest_dir()?;
    let mut github_output = get_github_output();
    let icons_path = icons_file_path(&manifest_dir);
    let out_icons_path = out_dir_icons_file_path()?;

    let npm_info = get_npm_version(SIMPLE_ICONS_PACKAGE)?;
    let crate_info = get_crates_io_version(PACKAGE_NAME)?;

    let npm_version = Version::parse(&npm_info.version)?;
    let crate_version = Version::parse(&crate_info.version)?;
    let version_outdated = crate_version < npm_version;
    let icons_missing = !icons_path.exists();
    let out_icons_missing = !out_icons_path.exists();

    write_github_output(
        &mut github_output,
        "updated",
        if version_outdated { "true" } else { "false" },
    );
    if version_outdated {
        write_github_output(&mut github_output, "version", &npm_info.version);
    }

    if !version_outdated && !icons_missing && !out_icons_missing {
        cargo_warning(&format!(
            "crate version {} is already up to date with npm version {}",
            crate_info.version, npm_info.version
        ));
        return Ok(());
    }

    if icons_missing {
        cargo_warning(&format!(
            "{} is missing, regenerating it from simple-icons {}",
            icons_path.display(),
            npm_info.version
        ));
    }
    if out_icons_missing {
        cargo_warning(&format!(
            "{} is missing, regenerating generated build output",
            out_icons_path.display()
        ));
    }

    download_and_extract_npm_tarball(&manifest_dir, &npm_info)?;
    generate_file(&manifest_dir)?;

    if version_outdated {
        replace_version(&manifest_dir, &npm_info)?;
        cargo_warning(&format!("updated icons to simple-icons {}", npm_info.version));
    } else {
        cargo_warning(&format!("regenerated icons from simple-icons {}", npm_info.version));
    }

    Ok(())
}

fn manifest_dir() -> BuildResult<PathBuf> {
    Ok(PathBuf::from(env::var("CARGO_MANIFEST_DIR")?))
}

fn cargo_warning(message: &str) {
    println!("cargo:warning={message}");
}

fn icons_file_path(manifest_dir: &Path) -> PathBuf {
    manifest_dir
        .join(CRATES_LIB_RELATIVE_PATH)
        .join(CRATES_ICON_FILE_NAME)
}

fn get_github_output() -> Option<File> {
    match env::var("GITHUB_OUTPUT") {
        Ok(path) => match OpenOptions::new().append(true).open(&path) {
            Ok(file) => Some(file),
            Err(err) => {
                cargo_warning(&format!("cannot open GITHUB_OUTPUT file '{path}': {err}"));
                None
            }
        },
        Err(_) => {
            cargo_warning("GITHUB_OUTPUT not set, skip CI output");
            None
        }
    }
}

fn write_github_output(output: &mut Option<File>, key: &str, value: &str) {
    if let Some(file) = output.as_mut()
        && let Err(err) = writeln!(file, "{key}={value}") {
            cargo_warning(&format!("failed to write GITHUB_OUTPUT {key}: {err}"));
            *output = None;
        }
}

fn get_npm_version(package: &str) -> BuildResult<NpmInformation> {
    let package_url = format!("{NPM_BASE_URL}{package}");
    let client = Client::new();
    let response: Value = client.get(&package_url).send()?.error_for_status()?.json()?;

    let version = response["dist-tags"]["latest"]
        .as_str()
        .ok_or(BuildError::MissingNpmLatestVersion)?
        .to_string();
    let tarball = response["versions"][&version]["dist"]["tarball"]
        .as_str()
        .ok_or_else(|| BuildError::MissingNpmTarball(version.clone()))?
        .to_string();

    Ok(NpmInformation {
        package: package.to_string(),
        version,
        tarball,
    })
}

fn get_crates_io_version(package: &str) -> BuildResult<CratesIOInformation> {
    let package_url = format!("{CRATES_IO_BASE_URL}{package}");
    let client = Client::new();
    let response: Value = client
        .get(&package_url)
        .header("User-Agent", "github.com/cscnk52/simpleicons-rs-builder")
        .send()?
        .error_for_status()?
        .json()?;

    let version = response["crate"]["max_stable_version"]
        .as_str()
        .ok_or(BuildError::MissingCratesMaxStableVersion)?
        .to_string();

    Ok(CratesIOInformation { version })
}

fn download_and_extract_npm_tarball(
    manifest_dir: &Path,
    npm_package: &NpmInformation,
) -> BuildResult<()> {
    let output_dir = npm_output_dir(manifest_dir);
    let package_dir = output_dir.join(NPM_PACKAGE_PATH);

    fs::create_dir_all(&output_dir)?;
    if package_dir.exists() {
        fs::remove_dir_all(&package_dir)?;
    }

    let response_bytes = blocking::get(&npm_package.tarball)?.error_for_status()?.bytes()?;
    let cursor = Cursor::new(response_bytes);
    let tar = GzDecoder::new(cursor);
    let mut archive = Archive::new(tar);
    archive.unpack(&output_dir)?;

    cargo_warning(&format!(
        "downloaded {}@{} into {}",
        npm_package.package,
        npm_package.version,
        output_dir.display()
    ));

    Ok(())
}

fn generate_file(manifest_dir: &Path) -> BuildResult<()> {
    let src_icons_path = icons_file_path(manifest_dir);
    let out_icons_path = out_dir_icons_file_path()?;

    ensure_parent_directory(&src_icons_path, "generated file path has no parent directory")?;
    ensure_parent_directory(&out_icons_path, "OUT_DIR generated file path has no parent directory")?;

    let icons = load_icons(manifest_dir)?;
    let content = render_icons_file(&icons)?;

    write_generated_icons_file(&out_icons_path, &content)?;

    if let Err(err) = write_generated_icons_file(&src_icons_path, &content) {
        cargo_warning(&format!(
            "failed to mirror generated icons into {}: {err}",
            src_icons_path.display()
        ));
    }

    Ok(())
}

fn load_icons(manifest_dir: &Path) -> BuildResult<Vec<IconData>> {
    let json_path = simple_icons_json_path(manifest_dir);
    let json_icons: Vec<JsonIcon> = serde_json::from_reader(File::open(&json_path)?)?;

    Ok(json_icons
        .into_iter()
        .map(|icon| IconData {
            svg: read_svg(manifest_dir, &icon.slug),
            title: icon.title,
            slug: icon.slug,
            hex: icon.hex,
            source: icon.source,
        })
        .collect())
}

fn render_icons_file(icons: &[IconData]) -> BuildResult<String> {
    let mut content = String::new();

    writeln!(&mut content, "{LIB_DEFINE}")?;
    render_icon_constants(&mut content, icons)?;
    render_slug_lookup(&mut content, icons)?;

    Ok(content)
}

fn render_icon_constants(content: &mut String, icons: &[IconData]) -> BuildResult<()> {
    for icon in icons {
        render_icon_constant(content, icon)?;
    }

    Ok(())
}

fn render_icon_constant(content: &mut String, icon: &IconData) -> BuildResult<()> {
    let const_name = generated_icon_name(&icon.slug);

    writeln!(content, "pub const {const_name}: Icon = Icon {{")?;
    writeln!(content, "\ttitle: {},", rust_string_literal(&icon.title))?;
    writeln!(content, "\tslug: {},", rust_string_literal(&icon.slug))?;
    writeln!(content, "\thex: {},", rust_string_literal(&icon.hex))?;
    writeln!(content, "\tsource: {},", rust_string_literal(&icon.source))?;
    writeln!(content, "\tsvg: r###\"{}\"###,", icon.svg)?;
    writeln!(content, "}};")?;

    Ok(())
}

fn render_slug_lookup(content: &mut String, icons: &[IconData]) -> BuildResult<()> {
    writeln!(content, "pub fn slug(slug: &str) -> Option<&'static Icon> {{")?;
    writeln!(content, "\tmatch slug {{")?;

    for icon in icons {
        render_slug_match_arm(content, icon)?;
    }

    writeln!(content, "\t\t_ => None,")?;
    writeln!(content, "\t}}")?;
    writeln!(content, "}}")?;
    Ok(())
}

fn render_slug_match_arm(content: &mut String, icon: &IconData) -> BuildResult<()> {
    let const_name = generated_icon_name(&icon.slug);

    writeln!(
        content,
        "\t\t{} => Some(&{const_name}),",
        rust_string_literal(&icon.slug)
    )?;

    Ok(())
}

fn out_dir_icons_file_path() -> BuildResult<PathBuf> {
    Ok(PathBuf::from(env::var("OUT_DIR")?).join(CRATES_ICON_FILE_NAME))
}

fn npm_output_dir(manifest_dir: &Path) -> PathBuf {
    manifest_dir.join(OUTPUT_DIR)
}

fn simple_icons_json_path(manifest_dir: &Path) -> PathBuf {
    npm_output_dir(manifest_dir)
        .join(NPM_PACKAGE_PATH)
        .join(SIMPLE_ICONS_NPM_JSON_RELATIVE_DIR)
        .join(SIMPLE_ICONS_NPM_JSON_FILENAME)
}

fn ensure_parent_directory(path: &Path, error_message: &'static str) -> BuildResult<()> {
    let parent = path
        .parent()
        .ok_or(BuildError::MissingParentDirectory(error_message))?;
    fs::create_dir_all(parent)?;
    Ok(())
}

fn write_generated_icons_file(path: &Path, content: &str) -> BuildResult<()> {
    fs::write(path, content)?;
    Ok(())
}

fn generated_icon_name(slug: &str) -> String {
    format!("SI{}", slug.to_uppercase())
}

fn rust_string_literal(value: &str) -> String {
    format!("{value:?}")
}

fn read_svg(manifest_dir: &Path, slug: &str) -> String {
    let path = npm_output_dir(manifest_dir)
        .join(NPM_PACKAGE_PATH)
        .join("icons")
        .join(format!("{slug}.svg"));

    match fs::read_to_string(&path) {
        Ok(svg) => svg,
        Err(err) => {
            cargo_warning(&format!("failed to read SVG for '{slug}': {err}"));
            String::new()
        }
    }
}

fn replace_version(manifest_dir: &Path, npm_info: &NpmInformation) -> BuildResult<()> {
    let manifest_path = manifest_dir.join(CRATES_METADATA_FILE_NAME);
    let content = fs::read_to_string(&manifest_path)?;
    let mut updated = content.parse::<DocumentMut>()?;
    updated["package"]["version"] = value(&npm_info.version);
    fs::write(&manifest_path, updated.to_string())?;
    Ok(())
}
