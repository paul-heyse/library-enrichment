# `datafusion_physical_expr_common::metrics::value::Count`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.metrics.value.Count.json).

<a id="op-2921678004049ad87901f32b"></a>
## Count

`struct` · `datafusion_physical_expr_common::metrics::value::Count` · datafusion-physical-expr-common 55.1.0

```rust
struct Count
```

Source: `src/metrics/value.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

A counter to record things such as number of input or output rows

Note `clone`ing counters update the same underlying metrics

<a id="op-6ab86288d94783bc55f275ae"></a>
## add

`function` · `datafusion_physical_expr_common::metrics::value::Count::add` · datafusion-physical-expr-common 55.1.0

```rust
fn add(&self, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [82, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Add `n` to the metric's value

<a id="op-ea6142c75de0c5c938d79d1c"></a>
## clone

`function` · `datafusion_physical_expr_common::metrics::value::Count::clone` · datafusion-physical-expr-common 55.1.0

```rust
fn clone(&self) -> Count
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 17], "end": [39, 22], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/metrics/value.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e7eaa26e4b1b72f430a9412"></a>
## default

`function` · `datafusion_physical_expr_common::metrics::value::Count::default` · datafusion-physical-expr-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [61, 2], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/metrics/value.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41b5a9ec6ffb9fcb4d7204eb"></a>
## eq

`function` · `datafusion_physical_expr_common::metrics::value::Count::eq` · datafusion-physical-expr-common 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [49, 2], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/metrics/value.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-488bfe53f6ecac15d53e4213"></a>
## fmt

`function` · `datafusion_physical_expr_common::metrics::value::Count::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [55, 2], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/metrics/value.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52d0e87d6a3beb8d54dfdf96"></a>
## fmt

`function` · `datafusion_physical_expr_common::metrics::value::Count::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 10], "end": [39, 15], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/metrics/value.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5c7c69ea1b410b6d54bdc3f"></a>
## new

`function` · `datafusion_physical_expr_common::metrics::value::Count::new` · datafusion-physical-expr-common 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [82, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

create a new counter

<a id="op-951828ea4ff5e599ed6b489d"></a>
## value

`function` · `datafusion_physical_expr_common::metrics::value::Count::value` · datafusion-physical-expr-common 55.1.0

```rust
fn value(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [82, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Get the current value
