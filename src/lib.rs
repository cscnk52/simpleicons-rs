mod icons;

pub use icons::*;

#[derive(Debug, Clone, Copy)]
pub struct Icon {
    pub title: &'static str,
    pub slug: &'static str,
    pub hex: &'static str,
    pub source: &'static str,
    pub svg: &'static str,
}

pub struct Aliases {
    pub aka: Option<Vec<&'static str>>,
    pub dup: Vec<DuplicatedAlias>,
}

pub struct DuplicatedAlias {
    pub title: &'static str,
    pub hex: Option<&'static str>,
    pub loc: &'static [(&'static str, &'static str)],
    pub old: Option<Vec<&'static str>>,
}

pub struct License {
    pub types: &'static str,
    pub url: &'static str,
}

pub type Icons = Vec<Icon>;
