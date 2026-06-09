use flate2::read::GzDecoder;

#[derive(Debug)]
pub struct Icon {
    pub title: &'static str,
    pub hex: &'static str,
    pub source: &'static str,
    pub slug: &'static str,
    pub svg: &'static str,
    pub guidelines: Option<&'static str>,
    pub license: Option<License>,
    pub aliases: Option<Aliases>,
}

#[derive(Debug)]
pub struct Aliases {
    pub aka: Option<Vec<&'static str>>,
    pub dup: Vec<DuplicatedAlias>,
}

#[derive(Debug)]
pub struct DuplicatedAlias {
    pub title: &'static str,
    pub hex: Option<&'static str>,
    pub loc: &'static [(&'static str, &'static str)],
    pub old: Option<Vec<&'static str>>,
}

#[derive(Debug)]
pub struct License {
    pub types: &'static str,
    pub url: &'static str,
}

#[derive(Debug, serde::Deserialize)]
struct CrateInfo {
    max_version: String,
}

#[derive(Debug, serde::Deserialize)]
struct CrateResponse {
    #[serde(rename = "crate")]
    krate: CrateInfo,
}

fn check_crate_version() -> Result<semver::Version, reqwest::Error> {
    let body = reqwest::blocking::Client::new()
        .get("https://crates.io/api/v1/crates/simpleicons-rs")
        .header(reqwest::header::USER_AGENT, "simpleicons-rs-build-script")
        .send()?
        .json::<CrateResponse>()?;
    Ok(semver::Version::parse(&body.krate.max_version).unwrap())
}

#[derive(serde::Deserialize)]
struct NpmResponse {
    version: String,
    dist: Dist,
}

#[derive(serde::Deserialize)]
struct Dist {
    tarball: String,
}

fn check_npm_version() -> Result<semver::Version, reqwest::Error> {
    let body = reqwest::blocking::get("https://registry.npmjs.org/simple-icons/latest")?
        .json::<NpmResponse>()?;
    Ok(semver::Version::parse(&body.version).unwrap())
}

fn download_and_extract_tarball() -> Result<(), reqwest::Error> {
    let tarball_url = reqwest::blocking::get("https://registry.npmjs.org/simple-icons/latest")?
        .json::<NpmResponse>()?
        .dist
        .tarball;
    let bytes = reqwest::blocking::get(tarball_url)?.bytes()?;
    let decoder = GzDecoder::new(std::io::Cursor::new(bytes));
    let mut archive = tar::Archive::new(decoder);
    archive.unpack("npm").unwrap();
    Ok(())
}

fn main() -> Result<(), reqwest::Error> {
    check_npm_version()?;
    download_and_extract_tarball()?;
    Ok(())
}
