# `datafusion_common::config::ConfigMinTwoUsize`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.ConfigMinTwoUsize.json).

<a id="op-08259ee6e8a591c1bef0b724"></a>
## ConfigMinTwoUsize

`struct` · `datafusion_common::config::ConfigMinTwoUsize` · datafusion-common 55.1.0

```rust
struct ConfigMinTwoUsize
```

Source: `src/config.rs:673`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A `usize` configuration value that rejects 0 and 1 when set from strings.

Use this for options whose consumer divides the value in half to size an
internal buffer (e.g. a bounded channel capacity): values below 2 would
round down to a zero-capacity buffer and panic. Invalid values return a
configuration error through [`ConfigField`](../operations/datafusion_common.config.ConfigField.md#op-679f6f8b2eea5f3367d79f93) instead.

<a id="op-4536765d27c563ac46dd0824"></a>
## Err

`assoc_type` · `datafusion_common::config::ConfigMinTwoUsize::Err` · datafusion-common 55.1.0

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigMinTwoUsize", "path": "ConfigMinTwoUsize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [708, 1], "end": [714, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/config.rs:709`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-adac0dfd633096a143fd310d"></a>
## clone

`function` · `datafusion_common::config::ConfigMinTwoUsize::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> ConfigMinTwoUsize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigMinTwoUsize", "path": "ConfigMinTwoUsize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [672, 17], "end": [672, 22], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config.rs:672`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2edc6f2d4e99d5c37e68b49"></a>
## cmp

`function` · `datafusion_common::config::ConfigMinTwoUsize::cmp` · datafusion-common 55.1.0

```rust
fn cmp(&self, other: &ConfigMinTwoUsize) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigMinTwoUsize", "path": "ConfigMinTwoUsize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [672, 57], "end": [672, 60], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/config.rs:672`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-319e95ddf22d62a8d1fd7e45"></a>
## eq

`function` · `datafusion_common::config::ConfigMinTwoUsize::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &ConfigMinTwoUsize) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigMinTwoUsize", "path": "ConfigMinTwoUsize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [672, 30], "end": [672, 39], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/config.rs:672`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2e4bc402e238d24b815ac1b"></a>
## fmt

`function` · `datafusion_common::config::ConfigMinTwoUsize::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigMinTwoUsize", "path": "ConfigMinTwoUsize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [672, 10], "end": [672, 15], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config.rs:672`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dbe1023a7970ea9030621009"></a>
## fmt

`function` · `datafusion_common::config::ConfigMinTwoUsize::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigMinTwoUsize", "path": "ConfigMinTwoUsize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [745, 1], "end": [749, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/config.rs:746`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5cf522c7028b5989796d957f"></a>
## from_str

`function` · `datafusion_common::config::ConfigMinTwoUsize::from_str` · datafusion-common 55.1.0

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigMinTwoUsize", "path": "ConfigMinTwoUsize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [708, 1], "end": [714, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/config.rs:711`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ab2688a491110a252709691"></a>
## get

`function` · `datafusion_common::config::ConfigMinTwoUsize::get` · datafusion-common 55.1.0

```rust
const fn get(self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigMinTwoUsize", "path": "ConfigMinTwoUsize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [685, 1], "end": [700, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:697`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns the wrapped `usize`.

<a id="op-6c2e0ecd50f584fc1687fdc0"></a>
## hash

`function` · `datafusion_common::config::ConfigMinTwoUsize::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigMinTwoUsize", "path": "ConfigMinTwoUsize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [672, 62], "end": [672, 66], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/config.rs:672`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6b3ba026472a7fd4216f155"></a>
## partial_cmp

`function` · `datafusion_common::config::ConfigMinTwoUsize::partial_cmp` · datafusion-common 55.1.0

```rust
fn partial_cmp(&self, other: &ConfigMinTwoUsize) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigMinTwoUsize", "path": "ConfigMinTwoUsize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [672, 45], "end": [672, 55], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/config.rs:672`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-61486e2eeb52a9b4a023472a"></a>
## reset

`function` · `datafusion_common::config::ConfigMinTwoUsize::reset` · datafusion-common 55.1.0

```rust
fn reset(&mut self, key: &str) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigMinTwoUsize", "path": "ConfigMinTwoUsize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [716, 1], "end": [743, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:733`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a04d64e54194923e505fc57a"></a>
## set

`function` · `datafusion_common::config::ConfigMinTwoUsize::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, key: &str, value: &str) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigMinTwoUsize", "path": "ConfigMinTwoUsize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [716, 1], "end": [743, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:721`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5df378d1fab660d164a77a1"></a>
## try_new

`function` · `datafusion_common::config::ConfigMinTwoUsize::try_new` · datafusion-common 55.1.0

```rust
fn try_new(value: usize) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigMinTwoUsize", "path": "ConfigMinTwoUsize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [685, 1], "end": [700, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:688`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Creates a [`ConfigMinTwoUsize`](../operations/datafusion_common.config.ConfigMinTwoUsize.md#op-08259ee6e8a591c1bef0b724), returning a configuration error if
`value` is less than 2.

<a id="op-48538bdd8dee509ca3be79c8"></a>
## visit

`function` · `datafusion_common::config::ConfigMinTwoUsize::visit` · datafusion-common 55.1.0

```rust
fn visit<V: Visit>(&self, v: &mut V, key: &str, description: &'static str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigMinTwoUsize", "path": "ConfigMinTwoUsize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [716, 1], "end": [743, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:717`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
