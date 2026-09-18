# `datafusion_common::format::MetricType`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.format.MetricType.json).

<a id="op-fb6314931b98a471bb076826"></a>
## MetricType

`enum` · `datafusion_common::format::MetricType` · datafusion-common 55.1.0

```rust
enum MetricType
```

Source: `src/format.rs:228`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Categorizes metrics so the display layer can choose the desired verbosity.

The `datafusion.explain.analyze_level` configuration controls which
type is shown:
- `"dev"` (the default): all metrics are shown.
- `"summary"`: only metrics tagged as `Summary` are shown.

This is orthogonal to [`MetricCategory`](../operations/datafusion_common.format.MetricCategory.md#op-d05732d49bd39fe3321965e4), which filters by *what kind*
of value a metric represents (rows / bytes / timing).

# Difference from `EXPLAIN ANALYZE VERBOSE`

The `VERBOSE` keyword controls whether per-partition metrics are shown
(when specified) or aggregated metrics are displayed (when omitted).
In contrast, `MetricType` determines which *levels* of metrics are
displayed.

<a id="op-3ccd1cdb2d4e12f173262bb0"></a>
## Dev

`variant` · `datafusion_common::format::MetricType::Dev` · datafusion-common 55.1.0

```rust
Dev
```

Source: `src/format.rs:232`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

For deep operator-level introspection for developers

<a id="op-7bcfb00fbf022ad5fa55f4d4"></a>
## Err

`assoc_type` · `datafusion_common::format::MetricType::Err` · datafusion-common 55.1.0

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::MetricType", "path": "MetricType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [248, 1], "end": [260, 2], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/format.rs:249`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84506ba193c3edffebcb9c50"></a>
## Summary

`variant` · `datafusion_common::format::MetricType::Summary` · datafusion-common 55.1.0

```rust
Summary
```

Source: `src/format.rs:230`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Common metrics for high-level insights (answering which operator is slow)

<a id="op-6022d0ba96e52bb27782b4a6"></a>
## clone

`function` · `datafusion_common::format::MetricType::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> MetricType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::MetricType", "path": "MetricType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 17], "end": [227, 22], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/format.rs:227`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c9c0137e77e3977a3270a6b5"></a>
## eq

`function` · `datafusion_common::format::MetricType::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &MetricType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::MetricType", "path": "MetricType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 30], "end": [227, 39], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/format.rs:227`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0efabb9585f67c16d7603753"></a>
## fmt

`function` · `datafusion_common::format::MetricType::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::MetricType", "path": "MetricType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 10], "end": [227, 15], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/format.rs:227`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e77fff5bee42ebfd286d61d6"></a>
## fmt

`function` · `datafusion_common::format::MetricType::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::MetricType", "path": "MetricType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [269, 2], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/format.rs:263`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a173cfa21de9b3ba50cba6a8"></a>
## from_str

`function` · `datafusion_common::format::MetricType::from_str` · datafusion-common 55.1.0

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::MetricType", "path": "MetricType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [248, 1], "end": [260, 2], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/format.rs:251`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-edc1ced25b7c22c7790b6281"></a>
## hash

`function` · `datafusion_common::format::MetricType::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::MetricType", "path": "MetricType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 45], "end": [227, 49], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/format.rs:227`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9b23dde9c6f5a03aa34ccbd"></a>
## included_types

`function` · `datafusion_common::format::MetricType::included_types` · datafusion-common 55.1.0

```rust
fn included_types(self) -> Vec<MetricType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::MetricType", "path": "MetricType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 1], "end": [246, 2], "filename": "src/format.rs"}, "trait": null, "trait_path": null}`

Source: `src/format.rs:240`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns the set of metric types that should be shown for this level.

`Dev` is a superset of `Summary`: when the user selects
`analyze_level = 'dev'`, both `Summary` and `Dev` metrics are shown.

<a id="op-a154c076a223e67b4c045022"></a>
## set

`function` · `datafusion_common::format::MetricType::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, _: &str, value: &str) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::MetricType", "path": "MetricType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [271, 1], "end": [280, 2], "filename": "src/format.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/format.rs:276`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-024e4a61723768edf5827a9d"></a>
## visit

`function` · `datafusion_common::format::MetricType::visit` · datafusion-common 55.1.0

```rust
fn visit<V: Visit>(&self, v: &mut V, key: &str, description: &'static str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::MetricType", "path": "MetricType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [271, 1], "end": [280, 2], "filename": "src/format.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/format.rs:272`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
