use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct NpmInformation {
    pub package: String,
    pub version: String,
    pub tarball: String,
}

#[derive(Debug)]
pub struct CratesIOInformation {
    pub package: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JsonIcon {
    pub title: String,
    pub slug: String,
    pub hex: String,
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Icon {
    pub title: String,
    pub slug: String,
    pub hex: String,
    pub source: String,
    pub svg: String,
}

pub type JsonIcons = Vec<JsonIcon>;
pub type Icons = Vec<Icon>;
