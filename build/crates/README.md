# simpleicons-rs

A [Simple Icons](https://github.com/simple-icons/simple-icons) library for Rust. Provides up-to-date icons with an easy-to-use API, allows you to easily retrieve SVG data for popular brand icons directly from Rust code.

## Usage

### Plain SVG

If want dynamicly import icon:

```rust
use simpleicons_rs::slug;

fn main() {
    let slug = "rust"; // or what ever you want
    let svg = slug(slug).unwrap();
    println!("{:#?}", svg);
}
```

or you already know what icon you want, just:

```rust
use simpleicons_rs:SIRUST;

fn main() {
    println!("{:#?}", SIRUST);
}
```

you can freely choose, former have more flexibility, latter is more lightweight.

### Colored SVG

```rust
use simple_icons::slug_colored;

fn main() {
    let slug = "rust";

    // slug_colored(slug, color)

    // return Icon with Simple Icons Color
    let colored_svg_default = slug_colored(slug, "default");

    // or using CSS named Color
    let colored_svg_named = slug_colored(slug, "black");

    // or using hex Color
    let colored_svg_hex = slug_colored(slug, "#181717");

    // or any other color format support by csscolorparser
    // see https://crates.io/crates/csscolorparser
}
```


## Contributing

Contributions are welcome! Please open issues or pull requests.

## License

- [`simpleicons-rs`](https://github.com/cscnk52/simpleicons-rs-builder) build script and crate under [MIT](https://github.com/cscnk52/simpleicons-rs-builder?tab=MIT-1-ov-file) LICENSE.
- [`Simple Icons`](https://github.com/simple-icons/simple-icons) use [CC0-1.0](https://github.com/simple-icons/simple-icons?tab=CC0-1.0-1-ov-file) LICENSE and addition [Legal Disclaimer](https://github.com/simple-icons/simple-icons/blob/develop/DISCLAIMER.md).
