mod icons;
pub use icons::*;

pub fn slug_colored(slug: &str, color: &str) -> Option<Icon> {
    if let Some(icon) = icons::slug(slug) {
        let hex_color = if color == "default" {
            format!("#{}", icon.hex)
        } else {
            csscolorparser::parse(color).map_or("#000000".to_string(), |c| c.to_css_hex())
        };

        let colored_svg: String = icon
            .svg
            .replace("<svg", &format!("<svg fill=\"{}\"", hex_color));

        let colored_icon = Icon {
            svg: Box::leak(colored_svg.into_boxed_str()),
            ..*icon
        };

        println!("{}", colored_icon.svg);
        Some(colored_icon)
    } else {
        None
    }
}
