<div align="center">
    <img loading="lazy" src="https://github.com/EinStack/glide-python/blob/main/docs/glide_logo.png?raw=1" alt="Glide Logo" width="200px" height="200px" />
    <h1>Glide Rust Client</h1>
    <p>🦀 An official Rust client for <a href="https://github.com/EinStack/glide">Glide, an open reliable fast model gateway</a>.</p>
    <a href="https://discord.gg/pt53Ej7rrc"><img src="https://img.shields.io/discord/1181281407813828710" alt="Discord" /></a>
    <a href="https://glide.einstack.ai/"><img src="https://img.shields.io/badge/build-view-violet%20?style=flat&logo=books&label=docs&link=https%3A%2F%2Fglide.einstack.ai%2F" alt="Glide Docs" /></a>
    <a href="https://artifacthub.io/packages/helm/einstack/glide"><img src="https://img.shields.io/endpoint?url=https://artifacthub.io/badge/repository/einstack" alt="ArtifactHub" /></a>
</div>

---

> Glide is under active development right now 🛠️

> Give us a star⭐ to support the project and watch👀 our repositories not to miss any update

## Features

- `native-tls` to use system-native TLS. **Enabled by default**.
- `rustls-tls` to use TLS backed by rustls .

## Installation

```cmd
cargo add glide-rs
```

## Variables

- `GLIDE_BASE_URL` to override the default base address:
  `http://127.0.0.1:9099` (Optional).
- `GLIDE_USER_AGENT` to override the default `User-Agent`: `glide-rs/0.1.0`
  (Optional).
