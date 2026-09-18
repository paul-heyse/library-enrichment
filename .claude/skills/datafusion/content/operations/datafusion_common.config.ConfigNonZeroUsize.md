# `datafusion_common::config::ConfigNonZeroUsize`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.ConfigNonZeroUsize.json).

<a id="op-128a200167641f5c99ea9808"></a>
## ConfigNonZeroUsize

`struct` · `datafusion_common::config::ConfigNonZeroUsize` · datafusion-common 55.1.0

```rust
struct ConfigNonZeroUsize
```

Source: `src/config.rs:591`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A `usize` configuration value that rejects zero when set from strings.

Use this for options where zero is never a meaningful runtime value.
Invalid values return a configuration error through [`ConfigField`](../operations/datafusion_common.config.ConfigField.md#op-679f6f8b2eea5f3367d79f93).

<a id="op-b450057ad4431fa99ffa74bb"></a>
## Err

`assoc_type` · `datafusion_common::config::ConfigNonZeroUsize::Err` · datafusion-common 55.1.0

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigNonZeroUsize", "path": "ConfigNonZeroUsize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [623, 1], "end": [629, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/config.rs:624`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de78daed2c637c6c219a3201"></a>
## clone

`function` · `datafusion_common::config::ConfigNonZeroUsize::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> ConfigNonZeroUsize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigNonZeroUsize", "path": "ConfigNonZeroUsize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [590, 17], "end": [590, 22], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config.rs:590`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3f25ca19bd3648151fd115f"></a>
## cmp

`function` · `datafusion_common::config::ConfigNonZeroUsize::cmp` · datafusion-common 55.1.0

```rust
fn cmp(&self, other: &ConfigNonZeroUsize) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigNonZeroUsize", "path": "ConfigNonZeroUsize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [590, 57], "end": [590, 60], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/config.rs:590`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6f84a8b4c4cad23e445b34a"></a>
## eq

`function` · `datafusion_common::config::ConfigNonZeroUsize::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &ConfigNonZeroUsize) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigNonZeroUsize", "path": "ConfigNonZeroUsize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [590, 30], "end": [590, 39], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/config.rs:590`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f2103b960b09ba427ed2f3c"></a>
## fmt

`function` · `datafusion_common::config::ConfigNonZeroUsize::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigNonZeroUsize", "path": "ConfigNonZeroUsize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [660, 1], "end": [664, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/config.rs:661`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6795ec9476408ef2ecec936c"></a>
## fmt

`function` · `datafusion_common::config::ConfigNonZeroUsize::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigNonZeroUsize", "path": "ConfigNonZeroUsize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [590, 10], "end": [590, 15], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config.rs:590`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14c616ab308fe50db931f0c3"></a>
## from_str

`function` · `datafusion_common::config::ConfigNonZeroUsize::from_str` · datafusion-common 55.1.0

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigNonZeroUsize", "path": "ConfigNonZeroUsize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [623, 1], "end": [629, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/config.rs:626`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-15c9d8756160cb00b1c26938"></a>
## get

`function` · `datafusion_common::config::ConfigNonZeroUsize::get` · datafusion-common 55.1.0

```rust
const fn get(self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigNonZeroUsize", "path": "ConfigNonZeroUsize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [602, 1], "end": [615, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:612`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns the wrapped `usize`.

<a id="op-9f0336af021be328aa8d7ac4"></a>
## hash

`function` · `datafusion_common::config::ConfigNonZeroUsize::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigNonZeroUsize", "path": "ConfigNonZeroUsize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [590, 62], "end": [590, 66], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/config.rs:590`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6cf2d621a3e07cd3f32bc603"></a>
## partial_cmp

`function` · `datafusion_common::config::ConfigNonZeroUsize::partial_cmp` · datafusion-common 55.1.0

```rust
fn partial_cmp(&self, other: &ConfigNonZeroUsize) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigNonZeroUsize", "path": "ConfigNonZeroUsize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [590, 45], "end": [590, 55], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/config.rs:590`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86888560517252206fcec89b"></a>
## reset

`function` · `datafusion_common::config::ConfigNonZeroUsize::reset` · datafusion-common 55.1.0

```rust
fn reset(&mut self, key: &str) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigNonZeroUsize", "path": "ConfigNonZeroUsize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [631, 1], "end": [658, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:648`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d00b7764313be6b5710b1599"></a>
## set

`function` · `datafusion_common::config::ConfigNonZeroUsize::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, key: &str, value: &str) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigNonZeroUsize", "path": "ConfigNonZeroUsize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [631, 1], "end": [658, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:636`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e9d7de438042f38f9397fa4"></a>
## try_new

`function` · `datafusion_common::config::ConfigNonZeroUsize::try_new` · datafusion-common 55.1.0

```rust
fn try_new(value: usize) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigNonZeroUsize", "path": "ConfigNonZeroUsize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [602, 1], "end": [615, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:605`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Creates a [`ConfigNonZeroUsize`](../operations/datafusion_common.config.ConfigNonZeroUsize.md#op-128a200167641f5c99ea9808), returning a configuration error if
`value` is zero.

<a id="op-b33c31eee1c55bb3d1f4d0b8"></a>
## visit

`function` · `datafusion_common::config::ConfigNonZeroUsize::visit` · datafusion-common 55.1.0

```rust
fn visit<V: Visit>(&self, v: &mut V, key: &str, description: &'static str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigNonZeroUsize", "path": "ConfigNonZeroUsize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [631, 1], "end": [658, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:632`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
