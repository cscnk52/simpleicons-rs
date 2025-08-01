pub const NPM_BASE_URL: &str = "https://registry.npmjs.org/";
pub const CRATES_IO_BASE_URL: &str = "https://crates.io/api/v1/crates/";

pub const OUTPUT_DIR: &str = "build";
pub const OUTPUT_FILE: &str = "lib.rs";
// this relay on npm package struct
pub const NPM_PACKAGE_PATH: &str = "/package";
// this relay on simple icons package struct
pub const SIMPLE_ICONS_NPM_JSON_PATH: &str = "data/simple-icons.json";

pub const LIB_DEFINE: &str = r###"#[derive(Debug)]
pub struct Icon {
    pub title: &'static str,
    pub slug: &'static str,
    pub hex: &'static str,
    pub source: &'static str,
    pub svg: &'static str,
    pub path: &'static str,
}
"###;
