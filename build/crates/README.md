# simpleicons-rs

A [Simple Icons](https://github.com/simple-icons/simple-icons) library for Rust. Provides up-to-date icons with an easy-to-use API, allows you to easily retrieve SVG data for popular brand icons directly from Rust code.

## Usage

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

## Contributing

Contributions are welcome! Please open issues or pull requests.

## License

- [`simpleicons-rs`](https://github.com/cscnk52/simple-icons-rs) build script and crate under [MIT](https://github.com/cscnk52/simple-icons-rs?tab=MIT-1-ov-file) LICENSE.
- [`Simple Icons`](https://github.com/simple-icons/simple-icons) use [CC0-1.0](https://github.com/simple-icons/simple-icons?tab=CC0-1.0-1-ov-file) LICENSE and addition [Legal Disclaimer](https://github.com/simple-icons/simple-icons/blob/develop/DISCLAIMER.md).
