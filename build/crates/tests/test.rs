#[cfg(test)]
use simpleicons_rs::{slug, slug_colored, SIGITHUB};

#[test]
fn github_icon_static() {
    let github_icon = SIGITHUB;
    assert_eq!(github_icon.title, "GitHub");
    assert_eq!(github_icon.slug, "github");
    assert_eq!(github_icon.hex, "181717");
    assert_eq!(github_icon.source, "https://github.com/logos");
    assert!(
        github_icon
            .svg
            .starts_with("<svg role=\"img\" viewBox=\"0 0 24 24\"")
    );
}

#[test]
fn github_icon_slug() {
    let github_icon = slug("github").unwrap();
    assert_eq!(github_icon.title, "GitHub");
    assert_eq!(github_icon.slug, "github");
    assert_eq!(github_icon.hex, "181717");
    assert_eq!(github_icon.source, "https://github.com/logos");
    assert!(
        github_icon
            .svg
            .starts_with("<svg role=\"img\" viewBox=\"0 0 24 24\"")
    );
}

#[test]
fn github_icon_slug_colored_hex() {
    let github_icon = slug_colored("github", "#000000").unwrap();
    assert_eq!(github_icon.title, "GitHub");
    assert_eq!(github_icon.slug, "github");
    assert_eq!(github_icon.hex, "181717");
    assert_eq!(github_icon.source, "https://github.com/logos");
    assert!(
        github_icon
            .svg
            .starts_with("<svg fill=\"#000000\" role=\"img\" viewBox=\"0 0 24 24\"")
    );
}

#[test]
fn github_icon_slug_colored_named_color() {
    let github_icon = slug_colored("github", "black").unwrap();
    assert_eq!(github_icon.title, "GitHub");
    assert_eq!(github_icon.slug, "github");
    assert_eq!(github_icon.hex, "181717");
    assert_eq!(github_icon.source, "https://github.com/logos");
    assert!(
        github_icon
            .svg
            .starts_with("<svg fill=\"#000000\" role=\"img\" viewBox=\"0 0 24 24\"")
    );
}

#[test]
fn github_icon_slug_colored_default_color() {
    let github_icon = slug_colored("github", "default").unwrap();
    assert_eq!(github_icon.title, "GitHub");
    assert_eq!(github_icon.slug, "github");
    assert_eq!(github_icon.hex, "181717");
    assert_eq!(github_icon.source, "https://github.com/logos");
    assert!(
        github_icon
            .svg
            .starts_with("<svg fill=\"#181717\" role=\"img\" viewBox=\"0 0 24 24\"")
    );
}

#[test]
fn empty_icon() {
    assert!(slug("__not_a_icon__").is_none());
}
