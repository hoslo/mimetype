# rdcache
[![Crates.io][crates-badge]][crates-url]
[![MIT/Apache-2 licensed][license-badge]][license-url]

[crates-badge]: https://img.shields.io/crates/v/mimetype.svg
[crates-url]: https://crates.io/crates/mimetype
[license-badge]: https://img.shields.io/crates/l/mimetype.svg
[license-url]: LICENSE

Rust version of [mimetype](https://github.com/gabriel-vasile/mimetype)

## Features
- Detects MIME type based on the "magic bytes" of a file.

## Example
```rust
fn main() {
    let file = std::fs::read("test.jpg").unwrap();

    let mime = mimetype::detect(&file);

    println!("{:?}", mime);
}
```

The output will be like:
```shell
Mime { mime: "image/jpeg", aliases: [], extension: ".jpg", children: [] }
```