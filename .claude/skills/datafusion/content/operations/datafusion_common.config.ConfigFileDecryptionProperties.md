# `datafusion_common::config::ConfigFileDecryptionProperties`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.ConfigFileDecryptionProperties.json).

<a id="op-79aa3637941c471ae997de50"></a>
## ConfigFileDecryptionProperties

`struct` · `datafusion_common::config::ConfigFileDecryptionProperties` · datafusion-common 55.1.0

```rust
struct ConfigFileDecryptionProperties
```

Source: `src/config.rs:3410`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8dce3bf9a37eb9987bb0945e"></a>
## Error

`assoc_type` · `datafusion_common::config::ConfigFileDecryptionProperties::Error` · datafusion-common 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigFileDecryptionProperties", "path": "ConfigFileDecryptionProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3536, 1], "end": [3568, 2], "filename": "src/config.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet::encryption::decrypt::FileDecryptionProperties", "path": "FileDecryptionProperties"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/config.rs:3537`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-15adc0e70f5e93e267acb342"></a>
## aad_prefix_as_hex

`struct_field` · `datafusion_common::config::ConfigFileDecryptionProperties::aad_prefix_as_hex` · datafusion-common 55.1.0

```rust
aad_prefix_as_hex: String
```

Source: `src/config.rs:3416`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

AAD prefix string uniquely identifies the file and prevents file swapping

<a id="op-aa5cf75d86af489881e03f0d"></a>
## clone

`function` · `datafusion_common::config::ConfigFileDecryptionProperties::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> ConfigFileDecryptionProperties
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigFileDecryptionProperties", "path": "ConfigFileDecryptionProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3409, 10], "end": [3409, 15], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config.rs:3409`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-43e4dc54e98af81c08276e66"></a>
## column_decryption_properties

`struct_field` · `datafusion_common::config::ConfigFileDecryptionProperties::column_decryption_properties` · datafusion-common 55.1.0

```rust
column_decryption_properties: std::collections::HashMap<String, ColumnDecryptionProperties>
```

Source: `src/config.rs:3414`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

HashMap of column names --> key in hex format

<a id="op-422862f27b4e9579b3ca0fdc"></a>
## default

`function` · `datafusion_common::config::ConfigFileDecryptionProperties::default` · datafusion-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigFileDecryptionProperties", "path": "ConfigFileDecryptionProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3430, 1], "end": [3439, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/config.rs:3431`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0a487b41d2c1f242ee0dead"></a>
## eq

`function` · `datafusion_common::config::ConfigFileDecryptionProperties::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &ConfigFileDecryptionProperties) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigFileDecryptionProperties", "path": "ConfigFileDecryptionProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3409, 24], "end": [3409, 33], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/config.rs:3409`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba6be46b2ada6be7c61198bc"></a>
## fmt

`function` · `datafusion_common::config::ConfigFileDecryptionProperties::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigFileDecryptionProperties", "path": "ConfigFileDecryptionProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3409, 17], "end": [3409, 22], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config.rs:3409`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a179d1b3e38946532f3cc3c"></a>
## footer_key_as_hex

`struct_field` · `datafusion_common::config::ConfigFileDecryptionProperties::footer_key_as_hex` · datafusion-common 55.1.0

```rust
footer_key_as_hex: String
```

Source: `src/config.rs:3412`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Binary string to use for the parquet footer encoded in hex format

<a id="op-5f02d5b88f5279c9cc8cbe15"></a>
## footer_signature_verification

`struct_field` · `datafusion_common::config::ConfigFileDecryptionProperties::footer_signature_verification` · datafusion-common 55.1.0

```rust
footer_signature_verification: bool
```

Source: `src/config.rs:3419`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

If true, then verify signature for files with plaintext footers.
default = true

<a id="op-22e4942c81951ab9a32ab6f4"></a>
## set

`function` · `datafusion_common::config::ConfigFileDecryptionProperties::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, key: &str, value: &str) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigFileDecryptionProperties", "path": "ConfigFileDecryptionProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3441, 1], "end": [3481, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:3459`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21235189eba9e50599266d36"></a>
## try_from

`function` · `datafusion_common::config::ConfigFileDecryptionProperties::try_from` · datafusion-common 55.1.0

```rust
fn try_from(f: &Arc<FileDecryptionProperties>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigFileDecryptionProperties", "path": "ConfigFileDecryptionProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3536, 1], "end": [3568, 2], "filename": "src/config.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet::encryption::decrypt::FileDecryptionProperties", "path": "FileDecryptionProperties"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/config.rs:3539`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fdefd90953d085a5dc34b75a"></a>
## visit

`function` · `datafusion_common::config::ConfigFileDecryptionProperties::visit` · datafusion-common 55.1.0

```rust
fn visit<V: Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigFileDecryptionProperties", "path": "ConfigFileDecryptionProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3441, 1], "end": [3481, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:3442`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
