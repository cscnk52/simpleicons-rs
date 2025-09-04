<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/cscnk52/simpleicons-rs/raw/refs/heads/main/assets/img/simpleicons-rs-banner-dark.png" />
  <source media="(prefers-color-scheme: light)" srcset="https://github.com/cscnk52/simpleicons-rs/raw/refs/heads/main/assets/img/simpleicons-rs-banner-light.png" />
  <img alt="simpleicons-rs banner" src="https://github.com/cscnk52/simpleicons-rs/raw/refs/heads/main/assets/img/simpleicons-rs-banner-light.png" />
</picture>

<div align="center">

# simpleicons-rs

Access high‑quality Simple Icons SVGs directly from your Rust code.

[![Crates.io][crates-badge]][crates-url]
[![Docs][docs-badge]][docs-url]
[![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/simpleicons-rs.svg
[crates-url]: https://crates.io/crates/simpleicons-rs
[docs-badge]: https://img.shields.io/docsrs/simpleicons-rs
[docs-url]: https://docs.rs/simpleicons-rs
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/cscnk52/simpleicons-rs/blob/main/LICENSE

</div>

## Overview

`simpleicons-rs` lets you fetch or embed SVGs from the [Simple Icons](https://simpleicons.org/) collection in Rust:

- Runtime lookup by slug.
- Zero‑cost compile‑time constants (no lookup).
- Optional color injection (brand default or any CSS color parseable by [`csscolorparser`](https://crates.io/crates/csscolorparser)).

## Installation

```bash
cargo add simpleicons-rs
```

## Usage

> [!IMPORTANT]
> Please read the [legal disclaimer](https://github.com/simple-icons/simple-icons/blob/develop/DISCLAIMER.md) before using any icon.

function will return Icon as follow:

```rust
pub struct Icon {
    pub title: &'static str,
    pub slug: &'static str,
    pub hex: &'static str,
    pub source: &'static str,
    pub svg: &'static str,
}
```

### Plain SVG

Use runtime lookup for flexibility, or a compile‑time constant for zero lookup:

```rust
use simpleicons_rs::{slug, SIRUST};

fn main() {
    let dynamic = slug("rust").unwrap(); // runtime lookup
    let constant = SIRUST;               // compile-time constant
    println!("{}", dynamic.svg);
    println!("{}", constant.svg);
}
```

Error handling:

```rust
match simpleicons_rs::slug("not-a-slug") {
    Some(icon) => println!("Found {}", icon.title),
    None => eprintln!("Icon not found"),
}
```

### Colored SVG

```rust
use simpleicons_rs::slug_colored;

fn main() {
    let slug = "rust";

    // Official brand color
    let brand = slug_colored(slug, "default").unwrap();

    // CSS named color
    let named = slug_colored(slug, "black").unwrap();

    // Hex
    let hexed = slug_colored(slug, "#181717").unwrap();

    // Any csscolorparser format: #abc, rgb(), rgba(), hsl(), hsla(), etc.
    let hsl = slug_colored(slug, "hsl(10 10% 10%)").unwrap();

    println!("{}", brand.svg);
}
```

## Build

This repo (builder) generates the publishable crate.

```bash
git clone https://github.com/cscnk52/simpleicons-rs.git
cd simpleicons-rs
cargo run
```

Generated crate appears under `build/crates`.
Then:

```bash
cd build/crates
cargo publish --allow-dirty
```

## License

- simpleicons-rs: MIT and CC0-1.0.
- simpleicons-rs-builder: MIT.
- Simple Icons: CC0-1.0 and [legal disclaimer](https://github.com/simple-icons/simple-icons/blob/develop/DISCLAIMER.md).
