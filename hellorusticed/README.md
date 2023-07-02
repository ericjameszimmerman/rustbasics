# Basic Rust Iced GUI Framework Example
See https://echozulu.hashnode.dev/getting-started-with-iced-gui-library-for-rust for more info.

As committed, this uses `Example 2`.

## Getting Started

```bash
# create the project
cargo new hellorusticed
cd hellorusticed
code .

## Example 1
(See https://github.com/iced-rs/iced/blob/0.9/examples/slider/src/main.rs)

# add dependencies
cargo add iced
cargo add iced_lazy
cargo add iced_native

# build
cargo build

## Example 2
(See https://github.com/iced-rs/iced/tree/0.9/examples/download_progress)

For the download bar example, add the following dependencies
```bash
cargo add iced --features tokio
cargo add iced_native
cargo add iced_futures
cargo add reqwest --no-default-features --features rustls-tls
```
