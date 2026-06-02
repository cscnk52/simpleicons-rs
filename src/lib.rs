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

#[cfg(test)]
mod tests {
    use super::{Icon, SIDOTENV, SIGITHUB, slug};

    fn assert_github_icon(icon: &Icon) {
        assert_eq!(icon.title, "GitHub");
        assert_eq!(icon.slug, "github");
        assert_eq!(icon.hex, "181717");
        assert_eq!(icon.source, "https://github.com/logos");
        assert!(
            icon.svg
                .starts_with("<svg role=\"img\" viewBox=\"0 0 24 24\"")
        );
    }

    fn assert_dotenv_icon(icon: &Icon) {
        assert_eq!(icon.title, ".ENV");
        assert_eq!(icon.slug, "dotenv");
        assert_eq!(icon.hex, "ECD53F");
        assert!(
            icon.svg
                .starts_with("<svg role=\"img\" viewBox=\"0 0 24 24\"")
        );
    }

    #[test]
    fn github_icon_static() {
        assert_github_icon(&SIGITHUB);
    }

    #[test]
    fn github_icon_slug() {
        let icon = slug("github").unwrap();
        assert_github_icon(icon);
    }

    #[test]
    fn github_slug_matches_exported_constant() {
        let icon = slug("github").unwrap();

        assert_eq!(icon.title, SIGITHUB.title);
        assert_eq!(icon.slug, SIGITHUB.slug);
        assert_eq!(icon.hex, SIGITHUB.hex);
        assert_eq!(icon.source, SIGITHUB.source);
        assert_eq!(icon.svg, SIGITHUB.svg);
    }

    #[test]
    fn github_icon_slug_colored_hex() {
        let svg = slug_colored("github", "#000000").unwrap();
        assert!(svg.starts_with("<svg fill=\"#000000\" role=\"img\" viewBox=\"0 0 24 24\""));
    }

    #[test]
    fn github_icon_slug_colored_named_color() {
        let svg = slug_colored("github", "black").unwrap();
        assert!(svg.starts_with("<svg fill=\"#000000\" role=\"img\" viewBox=\"0 0 24 24\""));
    }

    #[test]
    fn github_icon_slug_colored_default_color() {
        let svg = slug_colored("github", "default").unwrap();
        assert!(svg.starts_with("<svg fill=\"#181717\" role=\"img\" viewBox=\"0 0 24 24\""));
    }

    #[test]
    fn github_icon_slug_colored_invalid_color_falls_back_to_black() {
        let svg = slug_colored("github", "not-a-real-color").unwrap();
        assert!(svg.starts_with("<svg fill=\"#000000\" role=\"img\" viewBox=\"0 0 24 24\""));
    }

    #[test]
    fn github_icon_slug_colored_missing_slug_returns_none() {
        assert!(slug_colored("__not_a_icon__", "black").is_none());
    }

    #[test]
    fn github_slug_lookup_is_case_sensitive() {
        assert!(slug("GitHub").is_none());
    }

    #[test]
    fn github_original_svg_is_not_modified_by_coloring() {
        let _ = slug_colored("github", "black").unwrap();

        assert!(
            SIGITHUB
                .svg
                .starts_with("<svg role=\"img\" viewBox=\"0 0 24 24\"")
        );
        assert!(!SIGITHUB.svg.starts_with("<svg fill="));
    }

    #[test]
    fn github_colored_svg_inserts_fill_only_on_svg_tag() {
        let svg = slug_colored("github", "black").unwrap();
        assert_eq!(svg.matches("<svg fill=").count(), 1);
    }

    #[test]
    fn dotenv_icon_static() {
        assert_dotenv_icon(&SIDOTENV);
    }

    #[test]
    fn dotenv_icon_slug() {
        let icon = slug("dotenv").unwrap();
        assert_dotenv_icon(icon);
    }

    #[test]
    fn dotenv_slug_matches_exported_constant() {
        let icon = slug("dotenv").unwrap();

        assert_eq!(icon.title, SIDOTENV.title);
        assert_eq!(icon.slug, SIDOTENV.slug);
        assert_eq!(icon.hex, SIDOTENV.hex);
        assert_eq!(icon.source, SIDOTENV.source);
        assert_eq!(icon.svg, SIDOTENV.svg);
    }

    #[test]
    fn dotenv_default_color_uses_icon_hex() {
        let svg = slug_colored("dotenv", "default").unwrap();
        assert!(svg.starts_with("<svg fill=\"#ECD53F\" role=\"img\" viewBox=\"0 0 24 24\""));
    }

    #[test]
    fn empty_icon() {
        assert!(slug("__not_a_icon__").is_none());
    }
}
