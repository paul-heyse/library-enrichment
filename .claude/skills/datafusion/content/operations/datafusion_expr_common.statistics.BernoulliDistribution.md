# `datafusion_expr_common::statistics::BernoulliDistribution`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.statistics.BernoulliDistribution.json).

<a id="op-4930f386e2a68be31ee5f5b2"></a>
## BernoulliDistribution

`struct` · `datafusion_expr_common::statistics::BernoulliDistribution` · datafusion-expr-common 55.1.0

```rust
struct BernoulliDistribution
```

Source: `src/statistics.rs:293`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Bernoulli distribution with success probability `p`. If `p` has a null value,
the success probability is unknown. For a more in-depth discussion, see:

<https://en.wikipedia.org/wiki/Bernoulli_distribution>

<a id="op-db7889a5f7f80c2694ef5e9a"></a>
## clone

`function` · `datafusion_expr_common::statistics::BernoulliDistribution::clone` · datafusion-expr-common 55.1.0

```rust
fn clone(&self) -> BernoulliDistribution
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::BernoulliDistribution", "path": "BernoulliDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [292, 10], "end": [292, 15], "filename": "src/statistics.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/statistics.rs:292`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-90af83e3ec4d8f8f4330017d"></a>
## data_type

`function` · `datafusion_expr_common::statistics::BernoulliDistribution::data_type` · datafusion-expr-common 55.1.0

```rust
fn data_type(&self) -> DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::BernoulliDistribution", "path": "BernoulliDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [498, 1], "end": [563, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:513`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fdbf2729ae1c4aa0b6b49be2"></a>
## eq

`function` · `datafusion_expr_common::statistics::BernoulliDistribution::eq` · datafusion-expr-common 55.1.0

```rust
fn eq(&self, other: &BernoulliDistribution) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::BernoulliDistribution", "path": "BernoulliDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [292, 24], "end": [292, 33], "filename": "src/statistics.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/statistics.rs:292`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91fb586d8f2a78f3ae2b40ca"></a>
## fmt

`function` · `datafusion_expr_common::statistics::BernoulliDistribution::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::BernoulliDistribution", "path": "BernoulliDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [292, 17], "end": [292, 22], "filename": "src/statistics.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/statistics.rs:292`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-987c74d3cafcfade601662bf"></a>
## mean

`function` · `datafusion_expr_common::statistics::BernoulliDistribution::mean` · datafusion-expr-common 55.1.0

```rust
fn mean(&self) -> &ScalarValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::BernoulliDistribution", "path": "BernoulliDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [498, 1], "end": [563, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:521`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e226659e4ad48f08f8cd58d"></a>
## median

`function` · `datafusion_expr_common::statistics::BernoulliDistribution::median` · datafusion-expr-common 55.1.0

```rust
fn median(&self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::BernoulliDistribution", "path": "BernoulliDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [498, 1], "end": [563, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:527`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Computes the median value of this distribution. In case of an unknown
success probability, the function returns a `NULL` `ScalarValue`.

<a id="op-676a3d3e649286d84cd8fbc4"></a>
## p_value

`function` · `datafusion_expr_common::statistics::BernoulliDistribution::p_value` · datafusion-expr-common 55.1.0

```rust
fn p_value(&self) -> &ScalarValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::BernoulliDistribution", "path": "BernoulliDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [498, 1], "end": [563, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:517`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a90051991b145acf2109cce4"></a>
## range

`function` · `datafusion_expr_common::statistics::BernoulliDistribution::range` · datafusion-expr-common 55.1.0

```rust
fn range(&self) -> Interval
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::BernoulliDistribution", "path": "BernoulliDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [498, 1], "end": [563, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:551`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-54d8db930576e2b536febe81"></a>
## variance

`function` · `datafusion_expr_common::statistics::BernoulliDistribution::variance` · datafusion-expr-common 55.1.0

```rust
fn variance(&self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::BernoulliDistribution", "path": "BernoulliDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [498, 1], "end": [563, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:543`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Computes the variance value of this distribution. In case of an unknown
success probability, the function returns a `NULL` `ScalarValue`.
