# hyperliquid-metadata Crate Extraction

## Goal

Extract `src/asset_mapping/` into a standalone crate `hyperliquid-metadata` that external repos can depend on without importing the full `hyperliquid-rust-sdk`.

## Architecture

Cargo workspace with two members:
- `hyperliquid-metadata/` — standalone crate, minimal dependencies
- Root `hyperliquid-rust-sdk` — depends on `hyperliquid-metadata`, re-exports its types for backward compatibility

## New Crate: `hyperliquid-metadata`

### Directory layout

```
hyperliquid-metadata/
├── Cargo.toml
└── src/
    ├── lib.rs        (was mod.rs)
    ├── dex.rs
    ├── error.rs
    ├── fetch.rs
    ├── mapping.rs
    └── meta.rs
```

### Cargo.toml

```toml
[package]
name = "hyperliquid-metadata"
version = "0.1.0"
edition = "2021"
license = "MIT"
description = "Hyperliquid asset metadata and symbol mapping"

[dependencies]
reqwest = "0.12.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
```

No `tokio` dependency — async is runtime-agnostic.

### Public API

```rust
pub struct AssetMapping {
    pub symbol_to_asset_id: HashMap<String, u32>,
    pub asset_name_to_symbol: HashMap<String, String>,
}

impl AssetMapping {
    pub async fn new(client: &reqwest::Client, base_url: &str, interested_perp_dexs: Vec<String>) -> Result<Self, AssetMappingError>;
    pub fn asset_id(&self, symbol: &str) -> Option<u32>;
    pub fn symbol(&self, asset_name: &str) -> Option<&String>;
}

pub enum Dex { Hyperliquid, Hyena, Xyz }
pub enum AssetMappingError { Request(reqwest::Error), JsonParse(String), Api { status_code, message } }
```

Meta types (`Meta`, `SpotMeta`, etc.) remain `pub(crate)` — internal only.

### Source files

Moved verbatim from `src/asset_mapping/` with one change: `mod.rs` becomes `lib.rs`.

## SDK Changes

### Cargo.toml (root)

Add workspace and dependency:

```toml
[workspace]
members = [".", "hyperliquid-metadata"]

[dependencies]
hyperliquid-metadata = { path = "hyperliquid-metadata" }
```

### src/lib.rs

Replace:
```rust
pub mod asset_mapping;
pub use asset_mapping::AssetMapping;
```

With:
```rust
pub use hyperliquid_metadata as asset_mapping;
pub use asset_mapping::AssetMapping;
```

The `exchange/mod.rs` re-export `pub use crate::asset_mapping::Dex;` continues to work since `asset_mapping` is now an alias for `hyperliquid_metadata`.

### Delete `src/asset_mapping/`

Entire directory removed — logic now lives in `hyperliquid-metadata/`.

## Backward Compatibility

All existing import paths continue to work:
- `hyperliquid_rust_sdk::AssetMapping` — via re-export
- `hyperliquid_rust_sdk::Dex` — via `exchange::*` → `crate::asset_mapping::Dex`
- `hyperliquid_rust_sdk::asset_mapping::AssetMappingError` — via the `pub use ... as asset_mapping` alias

## External Consumer Usage

```rust
# Cargo.toml
[dependencies]
hyperliquid-metadata = "0.1"
reqwest = "0.12"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

```rust
use hyperliquid_metadata::{AssetMapping, Dex};

let client = reqwest::Client::new();
let mapping = AssetMapping::new(
    &client,
    "https://api.hyperliquid.xyz",
    vec![Dex::Hyperliquid.dex_name().to_string()],
).await?;
```
