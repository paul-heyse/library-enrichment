# `datafusion_common::config::ConfigFileEncryptionProperties`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.ConfigFileEncryptionProperties.json).

<a id="op-ea6bb8933e1edd03ac24e431"></a>
## ConfigFileEncryptionProperties

`struct` · `datafusion_common::config::ConfigFileEncryptionProperties` · datafusion-common 55.1.0

```rust
struct ConfigFileEncryptionProperties
```

Source: `src/config.rs:3212`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad7e7f96ba8313e89f1386c0"></a>
## aad_prefix_as_hex

`struct_field` · `datafusion_common::config::ConfigFileEncryptionProperties::aad_prefix_as_hex` · datafusion-common 55.1.0

```rust
aad_prefix_as_hex: String
```

Source: `src/config.rs:3223`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

AAD prefix string uniquely identifies the file and prevents file swapping

<a id="op-fbc6f66351175282a6e71472"></a>
## clone

`function` · `datafusion_common::config::ConfigFileEncryptionProperties::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> ConfigFileEncryptionProperties
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigFileEncryptionProperties", "path": "ConfigFileEncryptionProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3211, 10], "end": [3211, 15], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config.rs:3211`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89631741a4e54e349e81e1b7"></a>
## column_encryption_properties

`struct_field` · `datafusion_common::config::ConfigFileEncryptionProperties::column_encryption_properties` · datafusion-common 55.1.0

```rust
column_encryption_properties: std::collections::HashMap<String, ColumnEncryptionProperties>
```

Source: `src/config.rs:3221`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

HashMap of column names --> (key in hex format, metadata)

<a id="op-079d71b228ed7089480440a7"></a>
## default

`function` · `datafusion_common::config::ConfigFileEncryptionProperties::default` · datafusion-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigFileEncryptionProperties", "path": "ConfigFileEncryptionProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3230, 1], "end": [3241, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/config.rs:3231`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd289aa8e43ab622dc0dbb41"></a>
## encrypt_footer

`struct_field` · `datafusion_common::config::ConfigFileEncryptionProperties::encrypt_footer` · datafusion-common 55.1.0

```rust
encrypt_footer: bool
```

Source: `src/config.rs:3215`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Should the parquet footer be encrypted
default is true

<a id="op-f656ee84178525f8538da963"></a>
## eq

`function` · `datafusion_common::config::ConfigFileEncryptionProperties::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &ConfigFileEncryptionProperties) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigFileEncryptionProperties", "path": "ConfigFileEncryptionProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3211, 24], "end": [3211, 33], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/config.rs:3211`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b824eedd12754c555e1a0050"></a>
## fmt

`function` · `datafusion_common::config::ConfigFileEncryptionProperties::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigFileEncryptionProperties", "path": "ConfigFileEncryptionProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3211, 17], "end": [3211, 22], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config.rs:3211`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9cccb41ae79136b724ebe9c"></a>
## footer_key_as_hex

`struct_field` · `datafusion_common::config::ConfigFileEncryptionProperties::footer_key_as_hex` · datafusion-common 55.1.0

```rust
footer_key_as_hex: String
```

Source: `src/config.rs:3217`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Key to use for the parquet footer encoded in hex format

<a id="op-cb5ab24f610ccab6c699bd79"></a>
## footer_key_metadata_as_hex

`struct_field` · `datafusion_common::config::ConfigFileEncryptionProperties::footer_key_metadata_as_hex` · datafusion-common 55.1.0

```rust
footer_key_metadata_as_hex: String
```

Source: `src/config.rs:3219`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Metadata information for footer key

<a id="op-7c04549a1e90273178a7cbf4"></a>
## from

`function` · `datafusion_common::config::ConfigFileEncryptionProperties::from` · datafusion-common 55.1.0

```rust
fn from(f: &Arc<FileEncryptionProperties>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigFileEncryptionProperties", "path": "ConfigFileEncryptionProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3373, 1], "end": [3407, 2], "filename": "src/config.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet::encryption::encrypt::FileEncryptionProperties", "path": "FileEncryptionProperties"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/config.rs:3374`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d30071eab1e20ba77dac40e"></a>
## set

`function` · `datafusion_common::config::ConfigFileEncryptionProperties::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, key: &str, value: &str) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigFileEncryptionProperties", "path": "ConfigFileEncryptionProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3252, 1], "end": [3303, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:3279`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c41799274a218ec82ba26658"></a>
## store_aad_prefix

`struct_field` · `datafusion_common::config::ConfigFileEncryptionProperties::store_aad_prefix` · datafusion-common 55.1.0

```rust
store_aad_prefix: bool
```

Source: `src/config.rs:3226`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

If true, store the AAD prefix in the file
default is false

<a id="op-36f04d0631b0fff4f12c7bed"></a>
## visit

`function` · `datafusion_common::config::ConfigFileEncryptionProperties::visit` · datafusion-common 55.1.0

```rust
fn visit<V: Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigFileEncryptionProperties", "path": "ConfigFileEncryptionProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3252, 1], "end": [3303, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:3253`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
