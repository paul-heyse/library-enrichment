# `datafusion_common::config::ConfigEntry`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.ConfigEntry.json).

<a id="op-e610e9614a810518fe36454b"></a>
## ConfigEntry

`struct` · `datafusion_common::config::ConfigEntry` · datafusion-common 55.1.0

```rust
struct ConfigEntry
```

Source: `src/config.rs:1901`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A key value pair, with a corresponding description

<a id="op-ccfd13af5b4b658e099d1583"></a>
## clone

`function` · `datafusion_common::config::ConfigEntry::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> ConfigEntry
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigEntry", "path": "ConfigEntry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1900, 17], "end": [1900, 22], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config.rs:1900`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-821335165885be623e2718b6"></a>
## description

`struct_field` · `datafusion_common::config::ConfigEntry::description` · datafusion-common 55.1.0

```rust
description: &'static str
```

Source: `src/config.rs:1909`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A description of this configuration entry

<a id="op-2b05ea304b7063d6c2415865"></a>
## eq

`function` · `datafusion_common::config::ConfigEntry::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &ConfigEntry) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigEntry", "path": "ConfigEntry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1900, 30], "end": [1900, 39], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/config.rs:1900`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-097c2c45e072c0c7f0e49221"></a>
## fmt

`function` · `datafusion_common::config::ConfigEntry::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigEntry", "path": "ConfigEntry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1900, 10], "end": [1900, 15], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config.rs:1900`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a573f9d6b8f0e1b2f3e3f45a"></a>
## hash

`function` · `datafusion_common::config::ConfigEntry::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigEntry", "path": "ConfigEntry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1900, 24], "end": [1900, 28], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/config.rs:1900`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a406889102d8c8477797dcb"></a>
## key

`struct_field` · `datafusion_common::config::ConfigEntry::key` · datafusion-common 55.1.0

```rust
key: String
```

Source: `src/config.rs:1903`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A unique string to identify this config value

<a id="op-efd80b2c083a930e587da781"></a>
## value

`struct_field` · `datafusion_common::config::ConfigEntry::value` · datafusion-common 55.1.0

```rust
value: Option<String>
```

Source: `src/config.rs:1906`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The value if any
