mod icons {
    include!(concat!(env!("OUT_DIR"), "/icons.rs"));
}

pub use icons::*;

pub fn slug_colored(slug: &str, color: &str) -> Option<String> {
    let icon = icons::slug(slug)?;
    let fill = match color {
        "default" => format!("#{}", icon.hex),
        color => csscolorparser::parse(color)
            .map(|c| c.to_css_hex())
            .unwrap_or_else(|_| "#000000".to_string()),
    };

    Some(icon.svg.replace("<svg", &format!("<svg fill=\"{fill}\"")))
}
