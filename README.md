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
### Sync version
Add this to your `Cargo.toml`:
```toml
[dependencies]
mimetype = "0.1.6"
```

Then you can use it like this:
```rust
fn main() {
    let file = std::fs::read("test.jpg").unwrap();

    let mime = mimetype::detect(&file);

    println!("{:?}", mime);
}
```
or use reader
```rust
fn main() {
    let file = std::fs::File::open("Cargo.toml").unwrap();

    let mime = mimetype::detect(&file);

    println!("{:?}", mime);
}
```

### Async version
Add this to your `Cargo.toml`:
```toml
[dependencies]
mimetype = { version = "0.1.6", features = ["async"] }
```

Then you can use it like this:
```rust
#[tokio::main]
async fn main() {
    let file = tokio::fs::read("test.jpg").await.unwrap();

    let mime = mimetype::detect_async(&file).await;

    println!("{:?}", mime);
}
```
or use reader
```rust
#[tokio::main]
async fn main() {
    let file = tokio::fs::File::open("test.jpg").await.unwrap();

    let mime = mimetype::detect_async(&file).await;

    println!("{:?}", mime);
}
```

The output will be like:
```shell
Mime { mime: "image/jpeg", aliases: [], extension: ".jpg" }
```