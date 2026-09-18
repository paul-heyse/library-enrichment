# `datafusion_common::format::ExplainAnalyzeCategories`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.format.ExplainAnalyzeCategories.json).

<a id="op-4332623cb5931495db7b9adf"></a>
## ExplainAnalyzeCategories

`enum` · `datafusion_common::format::ExplainAnalyzeCategories` · datafusion-common 55.1.0

```rust
enum ExplainAnalyzeCategories
```

Source: `src/format.rs:373`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Controls which [`MetricCategory`](../operations/datafusion_common.format.MetricCategory.md#op-d05732d49bd39fe3321965e4) values are shown in `EXPLAIN ANALYZE`.

Set via `SET datafusion.explain.analyze_categories = '...'`.

See [`MetricCategory`](../operations/datafusion_common.format.MetricCategory.md#op-d05732d49bd39fe3321965e4) for the determinism properties that motivate
this filter.

<a id="op-3fcba406c4a66dc1df6c8596"></a>
## All

`variant` · `datafusion_common::format::ExplainAnalyzeCategories::All` · datafusion-common 55.1.0

```rust
All
```

Source: `src/format.rs:376`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Show all metrics regardless of category (the default).

<a id="op-8bd79878c69b3741c6b4e79e"></a>
## Err

`assoc_type` · `datafusion_common::format::ExplainAnalyzeCategories::Err` · datafusion-common 55.1.0

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::ExplainAnalyzeCategories", "path": "ExplainAnalyzeCategories"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [385, 1], "end": [403, 2], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/format.rs:386`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c331861bf96258d32a057a3"></a>
## Only

`variant` · `datafusion_common::format::ExplainAnalyzeCategories::Only` · datafusion-common 55.1.0

```rust
Only
```

Source: `src/format.rs:382`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Show only metrics whose category is in the list.
Metrics with no declared category are treated as
[`Uncategorized`](MetricCategory::Uncategorized) for filtering.

An **empty** vec means "plan only" — suppress all metrics.

<a id="op-f7bfad6ec35c05cac5e24774"></a>
## clone

`function` · `datafusion_common::format::ExplainAnalyzeCategories::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> ExplainAnalyzeCategories
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::ExplainAnalyzeCategories", "path": "ExplainAnalyzeCategories"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [372, 17], "end": [372, 22], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/format.rs:372`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f16a5bf0a9cb0f9afeb31789"></a>
## default

`function` · `datafusion_common::format::ExplainAnalyzeCategories::default` · datafusion-common 55.1.0

```rust
fn default() -> ExplainAnalyzeCategories
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::ExplainAnalyzeCategories", "path": "ExplainAnalyzeCategories"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [372, 45], "end": [372, 52], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/format.rs:372`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5dc0d101c746c0bc6b72ed8e"></a>
## eq

`function` · `datafusion_common::format::ExplainAnalyzeCategories::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &ExplainAnalyzeCategories) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::ExplainAnalyzeCategories", "path": "ExplainAnalyzeCategories"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [372, 24], "end": [372, 33], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/format.rs:372`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-123064501d919afb8b199d59"></a>
## fmt

`function` · `datafusion_common::format::ExplainAnalyzeCategories::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::ExplainAnalyzeCategories", "path": "ExplainAnalyzeCategories"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [405, 1], "end": [423, 2], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/format.rs:406`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e213d5cd75d9288afc7d59b8"></a>
## fmt

`function` · `datafusion_common::format::ExplainAnalyzeCategories::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::ExplainAnalyzeCategories", "path": "ExplainAnalyzeCategories"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [372, 10], "end": [372, 15], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/format.rs:372`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04344244512335146ef89397"></a>
## from_str

`function` · `datafusion_common::format::ExplainAnalyzeCategories::from_str` · datafusion-common 55.1.0

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::ExplainAnalyzeCategories", "path": "ExplainAnalyzeCategories"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [385, 1], "end": [403, 2], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/format.rs:388`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b62da643ec30e5e7f4063bb"></a>
## hash

`function` · `datafusion_common::format::ExplainAnalyzeCategories::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::ExplainAnalyzeCategories", "path": "ExplainAnalyzeCategories"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [372, 39], "end": [372, 43], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/format.rs:372`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ffd2ed9926bf853da27a077"></a>
## set

`function` · `datafusion_common::format::ExplainAnalyzeCategories::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, _: &str, value: &str) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::ExplainAnalyzeCategories", "path": "ExplainAnalyzeCategories"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [425, 1], "end": [434, 2], "filename": "src/format.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/format.rs:430`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-93e0200ed95765d6949d3d46"></a>
## visit

`function` · `datafusion_common::format::ExplainAnalyzeCategories::visit` · datafusion-common 55.1.0

```rust
fn visit<V: Visit>(&self, v: &mut V, key: &str, description: &'static str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::ExplainAnalyzeCategories", "path": "ExplainAnalyzeCategories"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [425, 1], "end": [434, 2], "filename": "src/format.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/format.rs:426`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
