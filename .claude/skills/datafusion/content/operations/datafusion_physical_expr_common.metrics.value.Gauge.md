# `datafusion_physical_expr_common::metrics::value::Gauge`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.metrics.value.Gauge.json).

<a id="op-114f4af4648b8e9e64d55ecd"></a>
## Gauge

`struct` · `datafusion_physical_expr_common::metrics::value::Gauge` · datafusion-physical-expr-common 55.1.0

```rust
struct Gauge
```

Source: `src/metrics/value.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

A gauge is the simplest metrics type. It just returns a value.
For example, you can easily expose current memory consumption with a gauge.

Note `clone`ing gauge update the same underlying metrics

<a id="op-a686cd6505c2682e232f46f9"></a>
## add

`function` · `datafusion_physical_expr_common::metrics::value::Gauge::add` · datafusion-physical-expr-common 55.1.0

```rust
fn add(&self, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Gauge", "path": "Gauge"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [150, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Add `n` to the metric's value

<a id="op-06aa6eff794fee93bf734fbf"></a>
## clone

`function` · `datafusion_physical_expr_common::metrics::value::Gauge::clone` · datafusion-physical-expr-common 55.1.0

```rust
fn clone(&self) -> Gauge
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Gauge", "path": "Gauge"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 17], "end": [88, 22], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/metrics/value.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae5402f8c652b1060fc95341"></a>
## default

`function` · `datafusion_physical_expr_common::metrics::value::Gauge::default` · datafusion-physical-expr-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Gauge", "path": "Gauge"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [106, 1], "end": [110, 2], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/metrics/value.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9c69c62a46efe9037bcba16"></a>
## eq

`function` · `datafusion_physical_expr_common::metrics::value::Gauge::eq` · datafusion-physical-expr-common 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Gauge", "path": "Gauge"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [98, 2], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/metrics/value.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b4254054c98d051c0b5388b1"></a>
## fmt

`function` · `datafusion_physical_expr_common::metrics::value::Gauge::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Gauge", "path": "Gauge"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 1], "end": [104, 2], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/metrics/value.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c34c48678cff2a526331d5a8"></a>
## fmt

`function` · `datafusion_physical_expr_common::metrics::value::Gauge::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Gauge", "path": "Gauge"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 10], "end": [88, 15], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/metrics/value.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1682048edc667c01a593ca0a"></a>
## new

`function` · `datafusion_physical_expr_common::metrics::value::Gauge::new` · datafusion-physical-expr-common 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Gauge", "path": "Gauge"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [150, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

create a new gauge

<a id="op-bb244becf86ff1c1beae07df"></a>
## set

`function` · `datafusion_physical_expr_common::metrics::value::Gauge::set` · datafusion-physical-expr-common 55.1.0

```rust
fn set(&self, n: usize) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Gauge", "path": "Gauge"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [150, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:140`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Set the metric's value to `n` and return the previous value

<a id="op-1eeeb0068d8aac5cc248bb61"></a>
## set_max

`function` · `datafusion_physical_expr_common::metrics::value::Gauge::set_max` · datafusion-physical-expr-common 55.1.0

```rust
fn set_max(&self, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Gauge", "path": "Gauge"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [150, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Set metric's value to maximum of `n` and current value

<a id="op-9c5942e6f729c24717132834"></a>
## sub

`function` · `datafusion_physical_expr_common::metrics::value::Gauge::sub` · datafusion-physical-expr-common 55.1.0

```rust
fn sub(&self, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Gauge", "path": "Gauge"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [150, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:128`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Sub `n` from the metric's value

<a id="op-aa80edf92f4ac57bc4dc9f71"></a>
## value

`function` · `datafusion_physical_expr_common::metrics::value::Gauge::value` · datafusion-physical-expr-common 55.1.0

```rust
fn value(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Gauge", "path": "Gauge"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [150, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:147`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Get the current value
