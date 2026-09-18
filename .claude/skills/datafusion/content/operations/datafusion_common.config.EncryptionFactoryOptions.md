# `datafusion_common::config::EncryptionFactoryOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.EncryptionFactoryOptions.json).

<a id="op-738c5ad1402c3f32910b8453"></a>
## EncryptionFactoryOptions

`struct` · `datafusion_common::config::EncryptionFactoryOptions` · datafusion-common 55.1.0

```rust
struct EncryptionFactoryOptions
```

Source: `src/config.rs:3572`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Holds implementation-specific options for an encryption factory

<a id="op-c597a36efe188c3f5740d1c2"></a>
## clone

`function` · `datafusion_common::config::EncryptionFactoryOptions::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> EncryptionFactoryOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::EncryptionFactoryOptions", "path": "EncryptionFactoryOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3571, 10], "end": [3571, 15], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config.rs:3571`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5f95e43ccdb8df0c8268350"></a>
## default

`function` · `datafusion_common::config::EncryptionFactoryOptions::default` · datafusion-common 55.1.0

```rust
fn default() -> EncryptionFactoryOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::EncryptionFactoryOptions", "path": "EncryptionFactoryOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3571, 24], "end": [3571, 31], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/config.rs:3571`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27401af1d5513b13d5d24e5a"></a>
## eq

`function` · `datafusion_common::config::EncryptionFactoryOptions::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &EncryptionFactoryOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::EncryptionFactoryOptions", "path": "EncryptionFactoryOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3571, 33], "end": [3571, 42], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/config.rs:3571`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84203b59f42deeb4682ae25e"></a>
## fmt

`function` · `datafusion_common::config::EncryptionFactoryOptions::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::EncryptionFactoryOptions", "path": "EncryptionFactoryOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3571, 17], "end": [3571, 22], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config.rs:3571`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c3333b1bd4ee810834c9be5"></a>
## options

`struct_field` · `datafusion_common::config::EncryptionFactoryOptions::options` · datafusion-common 55.1.0

```rust
options: std::collections::HashMap<String, String>
```

Source: `src/config.rs:3573`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83d76457ba815b1fd4401d5e"></a>
## set

`function` · `datafusion_common::config::EncryptionFactoryOptions::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, key: &str, value: &str) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::EncryptionFactoryOptions", "path": "EncryptionFactoryOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3576, 1], "end": [3591, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:3587`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18ac1af25490831b84dad629"></a>
## to_extension_options

`function` · `datafusion_common::config::EncryptionFactoryOptions::to_extension_options` · datafusion-common 55.1.0

```rust
fn to_extension_options<T: ExtensionOptions + Default>(&self) -> Result<T>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::EncryptionFactoryOptions", "path": "EncryptionFactoryOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3593, 1], "end": [3602, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:3595`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Convert these encryption factory options to an [`ExtensionOptions`](../operations/datafusion_common.config.ExtensionOptions.md#op-cf0c51673459a43f486d37db) instance.

<a id="op-d8b1bcd5f234a67c4b4b5b95"></a>
## visit

`function` · `datafusion_common::config::EncryptionFactoryOptions::visit` · datafusion-common 55.1.0

```rust
fn visit<V: Visit>(&self, v: &mut V, key: &str, _description: &'static str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::EncryptionFactoryOptions", "path": "EncryptionFactoryOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3576, 1], "end": [3591, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:3577`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
