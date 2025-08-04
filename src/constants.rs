use once_cell::sync::Lazy;
use phf::phf_map;
use regex::Regex;

pub const NPM_BASE_URL: &str = "https://registry.npmjs.org/";
pub const CRATES_IO_BASE_URL: &str = "https://crates.io/api/v1/crates/";

pub const OUTPUT_DIR: &str = "build";
pub const OUTPUT_FILE: &str = "lib.rs";
// this relay on npm package struct
pub const NPM_PACKAGE_PATH: &str = "package";
// this relay on simple icons package struct
pub const SIMPLE_ICONS_NPM_JSON_RELATIVE_DIR: &str = "data";
pub const SIMPLE_ICONS_NPM_JSON_FILENAME: &str = "simple-icons.json";

pub static TITLE_TO_SLUG_REPLACEMENTS: phf::Map<char, &str> = phf_map! {
    '+' => "plus",
    '.' => "dot",
    '&' => "and",
    // undocumented
    '#' => "sharp",
    'đ' => "d",
    'ħ' => "h",
    'ı' => "i",
    'ĸ' => "k",
    '\u{0140}' => "l", // ŀ
    'ł' => "l",
    'ß' => "ss",
    'ŧ' => "t",
    'ø' => "o",
};

pub static REMOVE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^a-z0-9]").unwrap());

pub const LIB_DEFINE: &str = r###"#[derive(Debug)]
pub struct Icon {
    pub title: &'static str,
    pub slug: &'static str,
    pub hex: &'static str,
    pub source: &'static str,
    pub svg: &'static str,
}

pub struct SimpleIcons;

impl SimpleIcons {
"###;
