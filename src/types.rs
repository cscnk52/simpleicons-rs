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
