# `datafusion_common::config::ParquetEncryptionOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.ParquetEncryptionOptions.json).

<a id="op-cc3870a8619a50ef076fb6fa"></a>
## ParquetEncryptionOptions

`struct` · `datafusion_common::config::ParquetEncryptionOptions` · datafusion-common 55.1.0

```rust
struct ParquetEncryptionOptions
```

Source: `src/config.rs:1426`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Options for configuring Parquet Modular Encryption

To use Parquet encryption, you must enable the `parquet_encryption` feature flag, as it is not activated by default.

<a id="op-4d3cba6131c39c8e14438015"></a>
## clone

`function` · `datafusion_common::config::ParquetEncryptionOptions::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> ParquetEncryptionOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ParquetEncryptionOptions", "path": "ParquetEncryptionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1426, 1], "end": [1445, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config.rs:1426`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b02dc69d01e58f6073e80996"></a>
## configure_factory

`function` · `datafusion_common::config::ParquetEncryptionOptions::configure_factory` · datafusion-common 55.1.0

```rust
fn configure_factory(&mut self, factory_id: &str, config: &impl ExtensionOptions)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ParquetEncryptionOptions", "path": "ParquetEncryptionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1447, 1], "end": [1462, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:1449`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Specify the encryption factory to use for Parquet modular encryption, along with its configuration

<a id="op-b3e933290cded9a549867100"></a>
## default

`function` · `datafusion_common::config::ParquetEncryptionOptions::default` · datafusion-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ParquetEncryptionOptions", "path": "ParquetEncryptionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1426, 1], "end": [1445, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/config.rs:1426`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-87d9c4cf2413a8b16ae91ef4"></a>
## eq

`function` · `datafusion_common::config::ParquetEncryptionOptions::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &ParquetEncryptionOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ParquetEncryptionOptions", "path": "ParquetEncryptionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1426, 1], "end": [1445, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/config.rs:1426`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c19e06baa944d09e9b329725"></a>
## factory_id

`struct_field` · `datafusion_common::config::ParquetEncryptionOptions::factory_id` · datafusion-common 55.1.0

```rust
factory_id: Option<String>
```

Source: `src/config.rs:1426`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Identifier for the encryption factory to use to create file encryption and decryption properties.
Encryption factories can be registered in the runtime environment with
`RuntimeEnv::register_parquet_encryption_factory`.

<a id="op-fa32023226b5566709d15721"></a>
## factory_options

`struct_field` · `datafusion_common::config::ParquetEncryptionOptions::factory_options` · datafusion-common 55.1.0

```rust
factory_options: EncryptionFactoryOptions
```

Source: `src/config.rs:1426`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Any encryption factory specific options

<a id="op-c9b1fb2ab8b15c57229cb092"></a>
## file_decryption

`struct_field` · `datafusion_common::config::ParquetEncryptionOptions::file_decryption` · datafusion-common 55.1.0

```rust
file_decryption: Option<ConfigFileDecryptionProperties>
```

Source: `src/config.rs:1426`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Optional file decryption properties

<a id="op-0891e272881b0f39f154d736"></a>
## file_encryption

`struct_field` · `datafusion_common::config::ParquetEncryptionOptions::file_encryption` · datafusion-common 55.1.0

```rust
file_encryption: Option<ConfigFileEncryptionProperties>
```

Source: `src/config.rs:1426`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Optional file encryption properties

<a id="op-0f8500be575880d019b3f80a"></a>
## fmt

`function` · `datafusion_common::config::ParquetEncryptionOptions::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ParquetEncryptionOptions", "path": "ParquetEncryptionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1426, 1], "end": [1445, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config.rs:1426`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7a5f4bf33bed5d3fbfc4127"></a>
## reset

`function` · `datafusion_common::config::ParquetEncryptionOptions::reset` · datafusion-common 55.1.0

```rust
fn reset(&mut self, key: &str) -> error::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ParquetEncryptionOptions", "path": "ParquetEncryptionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1426, 1], "end": [1445, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:1426`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cdea8211fda444aee641c202"></a>
## set

`function` · `datafusion_common::config::ParquetEncryptionOptions::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, key: &str, value: &str) -> error::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ParquetEncryptionOptions", "path": "ParquetEncryptionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1426, 1], "end": [1445, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:1426`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3839fe0a57d5000365328c6d"></a>
## visit

`function` · `datafusion_common::config::ParquetEncryptionOptions::visit` · datafusion-common 55.1.0

```rust
fn visit<V: config::Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ParquetEncryptionOptions", "path": "ParquetEncryptionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1426, 1], "end": [1445, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:1426`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
