# `datafusion_common::config::ConfigFileType`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.ConfigFileType.json).

<a id="op-4726b3cde33c7be1f02ee7c4"></a>
## ConfigFileType

`enum` · `datafusion_common::config::ConfigFileType` · datafusion-common 55.1.0

```rust
enum ConfigFileType
```

Source: `src/config.rs:2670`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

These file types have special built in behavior for configuration.
Use TableOptions::Extensions for configuring other file types.

<a id="op-bd6a70a17111f2819a510cf5"></a>
## CSV

`variant` · `datafusion_common::config::ConfigFileType::CSV` · datafusion-common 55.1.0

```rust
CSV
```

Source: `src/config.rs:2671`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-925d9256be2c5f8223324a94"></a>
## JSON

`variant` · `datafusion_common::config::ConfigFileType::JSON` · datafusion-common 55.1.0

```rust
JSON
```

Source: `src/config.rs:2674`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-831a8210f8ecf67aba5fbcf1"></a>
## PARQUET

`variant` · `datafusion_common::config::ConfigFileType::PARQUET` · datafusion-common 55.1.0

```rust
PARQUET
```

Source: `src/config.rs:2673`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e0a211a2afaff4baa2dd94e8"></a>
## clone

`function` · `datafusion_common::config::ConfigFileType::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> ConfigFileType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigFileType", "path": "ConfigFileType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2669, 17], "end": [2669, 22], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config.rs:2669`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9386ef2383f0a4b0d96c2fee"></a>
## fmt

`function` · `datafusion_common::config::ConfigFileType::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigFileType", "path": "ConfigFileType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2669, 10], "end": [2669, 15], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config.rs:2669`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
