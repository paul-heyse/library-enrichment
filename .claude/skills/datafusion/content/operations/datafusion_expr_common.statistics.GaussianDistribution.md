# `datafusion_expr_common::statistics::GaussianDistribution`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.statistics.GaussianDistribution.json).

<a id="op-ddd36543e5e539b82167af0b"></a>
## GaussianDistribution

`struct` · `datafusion_expr_common::statistics::GaussianDistribution` · datafusion-expr-common 55.1.0

```rust
struct GaussianDistribution
```

Source: `src/statistics.rs:279`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Gaussian (normal) distribution, represented by its mean and variance.
For a more in-depth discussion, see:

<https://en.wikipedia.org/wiki/Normal_distribution>

<a id="op-079f64d604f14add97d789c1"></a>
## clone

`function` · `datafusion_expr_common::statistics::GaussianDistribution::clone` · datafusion-expr-common 55.1.0

```rust
fn clone(&self) -> GaussianDistribution
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::GaussianDistribution", "path": "GaussianDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [278, 10], "end": [278, 15], "filename": "src/statistics.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/statistics.rs:278`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb492c9b4d87151dbdfc08bb"></a>
## data_type

`function` · `datafusion_expr_common::statistics::GaussianDistribution::data_type` · datafusion-expr-common 55.1.0

```rust
fn data_type(&self) -> DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::GaussianDistribution", "path": "GaussianDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [457, 1], "end": [496, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:477`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de9be0715e8da1005d276c35"></a>
## eq

`function` · `datafusion_expr_common::statistics::GaussianDistribution::eq` · datafusion-expr-common 55.1.0

```rust
fn eq(&self, other: &GaussianDistribution) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::GaussianDistribution", "path": "GaussianDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [278, 24], "end": [278, 33], "filename": "src/statistics.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/statistics.rs:278`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df2dba9fe8d55237c6eb59f8"></a>
## fmt

`function` · `datafusion_expr_common::statistics::GaussianDistribution::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::GaussianDistribution", "path": "GaussianDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [278, 17], "end": [278, 22], "filename": "src/statistics.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/statistics.rs:278`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b14abf6ca5a8e1afd0b44322"></a>
## mean

`function` · `datafusion_expr_common::statistics::GaussianDistribution::mean` · datafusion-expr-common 55.1.0

```rust
fn mean(&self) -> &ScalarValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::GaussianDistribution", "path": "GaussianDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [457, 1], "end": [496, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:481`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d986cc86f75ce9a8c6e0186"></a>
## median

`function` · `datafusion_expr_common::statistics::GaussianDistribution::median` · datafusion-expr-common 55.1.0

```rust
fn median(&self) -> &ScalarValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::GaussianDistribution", "path": "GaussianDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [457, 1], "end": [496, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:489`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f31cc866b0cc01be22323fb4"></a>
## range

`function` · `datafusion_expr_common::statistics::GaussianDistribution::range` · datafusion-expr-common 55.1.0

```rust
fn range(&self) -> Result<Interval>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::GaussianDistribution", "path": "GaussianDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [457, 1], "end": [496, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:493`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9da6067b1c8e56ed51b09b66"></a>
## variance

`function` · `datafusion_expr_common::statistics::GaussianDistribution::variance` · datafusion-expr-common 55.1.0

```rust
fn variance(&self) -> &ScalarValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::GaussianDistribution", "path": "GaussianDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [457, 1], "end": [496, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:485`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
