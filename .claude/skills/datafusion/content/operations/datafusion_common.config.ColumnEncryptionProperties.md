# `datafusion_common::config::ColumnEncryptionProperties`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.ColumnEncryptionProperties.json).

<a id="op-8b3dbb6d377f0d46b8b55332"></a>
## ColumnEncryptionProperties

`struct` · `datafusion_common::config::ColumnEncryptionProperties` · datafusion-common 55.1.0

```rust
struct ColumnEncryptionProperties
```

Source: `src/config.rs:3243`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dca8d42fd8d29d9d58aeb923"></a>
## clone

`function` · `datafusion_common::config::ColumnEncryptionProperties::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> ColumnEncryptionProperties
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ColumnEncryptionProperties", "path": "ColumnEncryptionProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3243, 1], "end": [3250, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config.rs:3243`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e4127f00fce5cd1f44df8ab"></a>
## column_key_as_hex

`struct_field` · `datafusion_common::config::ColumnEncryptionProperties::column_key_as_hex` · datafusion-common 55.1.0

```rust
column_key_as_hex: String
```

Source: `src/config.rs:3243`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Per column encryption key

<a id="op-0e0e50a97925db930f1ee3cc"></a>
## column_metadata_as_hex

`struct_field` · `datafusion_common::config::ColumnEncryptionProperties::column_metadata_as_hex` · datafusion-common 55.1.0

```rust
column_metadata_as_hex: Option<String>
```

Source: `src/config.rs:3243`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Per column encryption key metadata

<a id="op-72b841c18e8d9aa2eadc6d56"></a>
## default

`function` · `datafusion_common::config::ColumnEncryptionProperties::default` · datafusion-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ColumnEncryptionProperties", "path": "ColumnEncryptionProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3243, 1], "end": [3250, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/config.rs:3243`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ecde171007cb823e7d5e4afd"></a>
## eq

`function` · `datafusion_common::config::ColumnEncryptionProperties::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &ColumnEncryptionProperties) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ColumnEncryptionProperties", "path": "ColumnEncryptionProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3243, 1], "end": [3250, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/config.rs:3243`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-363d39f8cb996222bd773bf0"></a>
## fmt

`function` · `datafusion_common::config::ColumnEncryptionProperties::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ColumnEncryptionProperties", "path": "ColumnEncryptionProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3243, 1], "end": [3250, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config.rs:3243`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70585f2cb5a49d198e225ce0"></a>
## set

`function` · `datafusion_common::config::ColumnEncryptionProperties::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, key: &str, value: &str) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ColumnEncryptionProperties", "path": "ColumnEncryptionProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3243, 1], "end": [3250, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:3243`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92695daec6d42a3f567d6163"></a>
## visit

`function` · `datafusion_common::config::ColumnEncryptionProperties::visit` · datafusion-common 55.1.0

```rust
fn visit<V: Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ColumnEncryptionProperties", "path": "ColumnEncryptionProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3243, 1], "end": [3250, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:3243`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
