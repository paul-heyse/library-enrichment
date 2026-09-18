# `datafusion_physical_expr_common::metrics::Label`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.metrics.Label.json).

<a id="op-32cbece590fe957187c93920"></a>
## Label

`struct` · `datafusion_physical_expr_common::metrics::Label` · datafusion-physical-expr-common 55.1.0

```rust
struct Label
```

Source: `src/metrics/mod.rs:544`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

`name=value` pairs identifying a metric. This concept is called various things
in various different systems:

"labels" in
[prometheus](https://prometheus.io/docs/concepts/data_model/) and
"tags" in
[InfluxDB](https://docs.influxdata.com/influxdb/v1.8/write_protocols/line_protocol_tutorial/)
, "attributes" in [open
telemetry]<https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/metrics/data-model.md>,
etc.

As the name and value are expected to often be constant strings, borrowed
static strings avoid allocations in that common case. Dynamic strings are
stored behind [`Arc<str>`] so cloning labels does not copy the underlying
string data.

Unresolved upstream links (retained, not inferred): ``Arc<str>``.

<a id="op-23614de21ff5016d00cfbf17"></a>
## clone

`function` · `datafusion_physical_expr_common::metrics::Label::clone` · datafusion-physical-expr-common 55.1.0

```rust
fn clone(&self) -> Label
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::Label", "path": "Label"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [543, 17], "end": [543, 22], "filename": "src/metrics/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/metrics/mod.rs:543`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8423f25829d3c254a1e04fc9"></a>
## eq

`function` · `datafusion_physical_expr_common::metrics::Label::eq` · datafusion-physical-expr-common 55.1.0

```rust
fn eq(&self, other: &Label) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::Label", "path": "Label"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [543, 24], "end": [543, 33], "filename": "src/metrics/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/metrics/mod.rs:543`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a365fd09bea7ac5636e2dfe"></a>
## fmt

`function` · `datafusion_physical_expr_common::metrics::Label::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::Label", "path": "Label"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [543, 10], "end": [543, 15], "filename": "src/metrics/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/metrics/mod.rs:543`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e20fb185c6d90149ef54b031"></a>
## fmt

`function` · `datafusion_physical_expr_common::metrics::Label::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::Label", "path": "Label"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [568, 1], "end": [572, 2], "filename": "src/metrics/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/metrics/mod.rs:569`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12a9e1aa9d59de82dc3d6849"></a>
## hash

`function` · `datafusion_physical_expr_common::metrics::Label::hash` · datafusion-physical-expr-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::Label", "path": "Label"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [543, 39], "end": [543, 43], "filename": "src/metrics/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/metrics/mod.rs:543`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-570e3482c81de934ca460eb4"></a>
## name

`function` · `datafusion_physical_expr_common::metrics::Label::name` · datafusion-physical-expr-common 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::Label", "path": "Label"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [549, 1], "end": [566, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:558`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Returns the name of this label

<a id="op-26586ed29dfc946ea7b73c59"></a>
## new

`function` · `datafusion_physical_expr_common::metrics::Label::new` · datafusion-physical-expr-common 55.1.0

```rust
fn new(name: impl Into<LabelValue>, value: impl Into<LabelValue>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::Label", "path": "Label"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [549, 1], "end": [566, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:551`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Create a new [`Label`](../operations/datafusion_physical_expr_common.metrics.Label.md#op-32cbece590fe957187c93920)

<a id="op-088c81655784518933e8083e"></a>
## value

`function` · `datafusion_physical_expr_common::metrics::Label::value` · datafusion-physical-expr-common 55.1.0

```rust
fn value(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::Label", "path": "Label"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [549, 1], "end": [566, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:563`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Returns the value of this label
