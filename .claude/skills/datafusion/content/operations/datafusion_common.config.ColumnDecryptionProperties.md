# `datafusion_common::config::ColumnDecryptionProperties`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.ColumnDecryptionProperties.json).

<a id="op-caf0404e314689229942f638"></a>
## ColumnDecryptionProperties

`struct` · `datafusion_common::config::ColumnDecryptionProperties` · datafusion-common 55.1.0

```rust
struct ColumnDecryptionProperties
```

Source: `src/config.rs:3422`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a96d6b41e642f13e5bd4c762"></a>
## clone

`function` · `datafusion_common::config::ColumnDecryptionProperties::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> ColumnDecryptionProperties
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ColumnDecryptionProperties", "path": "ColumnDecryptionProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3422, 1], "end": [3427, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config.rs:3422`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01da2f107483414e30b74061"></a>
## column_key_as_hex

`struct_field` · `datafusion_common::config::ColumnDecryptionProperties::column_key_as_hex` · datafusion-common 55.1.0

```rust
column_key_as_hex: String
```

Source: `src/config.rs:3422`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Per column encryption key

<a id="op-2f7cb6254c422228121b986a"></a>
## default

`function` · `datafusion_common::config::ColumnDecryptionProperties::default` · datafusion-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ColumnDecryptionProperties", "path": "ColumnDecryptionProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3422, 1], "end": [3427, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/config.rs:3422`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99d82c67f99c0f305ef949b4"></a>
## eq

`function` · `datafusion_common::config::ColumnDecryptionProperties::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &ColumnDecryptionProperties) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ColumnDecryptionProperties", "path": "ColumnDecryptionProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3422, 1], "end": [3427, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/config.rs:3422`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9914289fd5434d27133adc55"></a>
## fmt

`function` · `datafusion_common::config::ColumnDecryptionProperties::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ColumnDecryptionProperties", "path": "ColumnDecryptionProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3422, 1], "end": [3427, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config.rs:3422`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da8d91b4c08ef4930105b58d"></a>
## set

`function` · `datafusion_common::config::ColumnDecryptionProperties::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, key: &str, value: &str) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ColumnDecryptionProperties", "path": "ColumnDecryptionProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3422, 1], "end": [3427, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:3422`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1253801731b66c1ac6e38d2e"></a>
## visit

`function` · `datafusion_common::config::ColumnDecryptionProperties::visit` · datafusion-common 55.1.0

```rust
fn visit<V: Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ColumnDecryptionProperties", "path": "ColumnDecryptionProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3422, 1], "end": [3427, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:3422`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
