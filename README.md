<div align="center">
    <img loading="lazy" src="https://github.com/EinStack/glide-python/blob/main/docs/glide_logo.png?raw=1" alt="Glide Logo" width="200px" height="200px" />
    <h1>Glide Rust Client</h1>
    <p>🦀 An official Rust client for <a href="https://github.com/EinStack/glide">Glide, an open reliable fast model gateway</a>.</p>
    <a href="https://discord.gg/pt53Ej7rrc"><img src="https://img.shields.io/discord/1181281407813828710" alt="Discord" /></a>
    <a href="https://glide.einstack.ai/"><img src="https://img.shields.io/badge/build-view-violet%20?style=flat&logo=books&label=docs&link=https%3A%2F%2Fglide.einstack.ai%2F" alt="Glide Docs" /></a>
    <a href="https://artifacthub.io/packages/helm/einstack/glide"><img src="https://img.shields.io/endpoint?url=https://artifacthub.io/badge/repository/einstack" alt="ArtifactHub" /></a>
	<br/>
	<a href="https://github.com/einstack/glide-rs/actions/workflows/build.yaml"><img src="https://img.shields.io/github/actions/workflow/status/einstack/glide-rs/build.yaml?branch=main&label=build&logo=github&style=flat-square" alt="Github Action" /></a>
	<a href="https://crates.io/crates/glide-rs"><img src="https://img.shields.io/crates/v/glide-rs.svg?logo=rust&style=flat-square" alt="Crates Io" /></a>
	<a href="https://docs.rs/glide-rs"><img src="https://img.shields.io/docsrs/glide-rs?logo=Docs.rs&style=flat-square" alt="Cargo Docs" /></a>
</div>

---

> Glide is under active development right now 🛠️

> Give us a star ⭐ to support the project and watch 👀 our repositories not to
> miss any update

## Features

- `streaming` to enable WebSocket chat support.
- `native-tls` to use system-native TLS. **Enabled by default**.
- `rustls-tls` for TLS backed by rustls.

## Installation

```cmd
cargo add glide-rs
```

## Usage

For a full example take a look at [`hello.rs`](examples/hello.rs).

```rust
use glide_rs::{Client, Result};
use glide_rs::lang::chat::ChatRequest;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::default();

    let request = ChatRequest::new("Hello!");
    let response = client.lang.chat("myrouter", request).await?;
    println!("response: {}", response.content());

    Ok(())
}
```
