# `datafusion_expr_common::statistics::UniformDistribution`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.statistics.UniformDistribution.json).

<a id="op-5f09f04776462ff4268e9ae7"></a>
## UniformDistribution

`struct` · `datafusion_expr_common::statistics::UniformDistribution` · datafusion-expr-common 55.1.0

```rust
struct UniformDistribution
```

Source: `src/statistics.rs:236`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Uniform distribution, represented by its range. If the given range extends
towards infinity, the distribution will be improper -- which is OK. For a
more in-depth discussion, see:

<https://en.wikipedia.org/wiki/Continuous_uniform_distribution>
<https://en.wikipedia.org/wiki/Prior_probability#Improper_priors>

<a id="op-d6ce9d25e842ecd7b0921550"></a>
## clone

`function` · `datafusion_expr_common::statistics::UniformDistribution::clone` · datafusion-expr-common 55.1.0

```rust
fn clone(&self) -> UniformDistribution
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::UniformDistribution", "path": "UniformDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 10], "end": [235, 15], "filename": "src/statistics.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/statistics.rs:235`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92de665a6ac513601c6764ba"></a>
## data_type

`function` · `datafusion_expr_common::statistics::UniformDistribution::data_type` · datafusion-expr-common 55.1.0

```rust
fn data_type(&self) -> DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::UniformDistribution", "path": "UniformDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [368, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:324`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1ee45919f5c1e642536e7b8"></a>
## eq

`function` · `datafusion_expr_common::statistics::UniformDistribution::eq` · datafusion-expr-common 55.1.0

```rust
fn eq(&self, other: &UniformDistribution) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::UniformDistribution", "path": "UniformDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 24], "end": [235, 33], "filename": "src/statistics.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/statistics.rs:235`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5afb3f0593811e95c30c73b5"></a>
## fmt

`function` · `datafusion_expr_common::statistics::UniformDistribution::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::UniformDistribution", "path": "UniformDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 17], "end": [235, 22], "filename": "src/statistics.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/statistics.rs:235`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27d5fb2d4dd64598ee3752a6"></a>
## mean

`function` · `datafusion_expr_common::statistics::UniformDistribution::mean` · datafusion-expr-common 55.1.0

```rust
fn mean(&self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::UniformDistribution", "path": "UniformDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [368, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:331`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Computes the mean value of this distribution. In case of improper
distributions (i.e. when the range is unbounded), the function returns
a `NULL` `ScalarValue`.

<a id="op-8aa3e3c19524a684ed2d3e63"></a>
## median

`function` · `datafusion_expr_common::statistics::UniformDistribution::median` · datafusion-expr-common 55.1.0

```rust
fn median(&self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::UniformDistribution", "path": "UniformDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [368, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:346`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-177ba44c3c7a365f7c173149"></a>
## range

`function` · `datafusion_expr_common::statistics::UniformDistribution::range` · datafusion-expr-common 55.1.0

```rust
fn range(&self) -> &Interval
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::UniformDistribution", "path": "UniformDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [368, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:365`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c284cbb82318f92ff009af32"></a>
## variance

`function` · `datafusion_expr_common::statistics::UniformDistribution::variance` · datafusion-expr-common 55.1.0

```rust
fn variance(&self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::UniformDistribution", "path": "UniformDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [368, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:353`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Computes the variance value of this distribution. In case of improper
distributions (i.e. when the range is unbounded), the function returns
a `NULL` `ScalarValue`.
