# `datafusion_expr_common::statistics::ExponentialDistribution`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.statistics.ExponentialDistribution.json).

<a id="op-fef3d10918b05446d917611d"></a>
## ExponentialDistribution

`struct` · `datafusion_expr_common::statistics::ExponentialDistribution` · datafusion-expr-common 55.1.0

```rust
struct ExponentialDistribution
```

Source: `src/statistics.rs:262`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Exponential distribution with an optional shift. The probability density
function (PDF) is defined as follows:

For a positive tail (when `positive_tail` is `true`):

`f(x; λ, offset) = λ exp(-λ (x - offset))    for x ≥ offset`

For a negative tail (when `positive_tail` is `false`):

`f(x; λ, offset) = λ exp(-λ (offset - x))    for x ≤ offset`


In both cases, the PDF is `0` outside the specified domain.

For more information, see:

<https://en.wikipedia.org/wiki/Exponential_distribution>

<a id="op-b3a91ba2007f7dc3656c5897"></a>
## clone

`function` · `datafusion_expr_common::statistics::ExponentialDistribution::clone` · datafusion-expr-common 55.1.0

```rust
fn clone(&self) -> ExponentialDistribution
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::ExponentialDistribution", "path": "ExponentialDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [261, 10], "end": [261, 15], "filename": "src/statistics.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/statistics.rs:261`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df33e9650ed475fe517b966b"></a>
## data_type

`function` · `datafusion_expr_common::statistics::ExponentialDistribution::data_type` · datafusion-expr-common 55.1.0

```rust
fn data_type(&self) -> DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::ExponentialDistribution", "path": "ExponentialDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [370, 1], "end": [455, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:402`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ad7df3f1543ebab1c1de8d6"></a>
## eq

`function` · `datafusion_expr_common::statistics::ExponentialDistribution::eq` · datafusion-expr-common 55.1.0

```rust
fn eq(&self, other: &ExponentialDistribution) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::ExponentialDistribution", "path": "ExponentialDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [261, 24], "end": [261, 33], "filename": "src/statistics.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/statistics.rs:261`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c0192d87e28a0a2f6f9cbef"></a>
## fmt

`function` · `datafusion_expr_common::statistics::ExponentialDistribution::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::ExponentialDistribution", "path": "ExponentialDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [261, 17], "end": [261, 22], "filename": "src/statistics.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/statistics.rs:261`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cfd8ed28756d59c2ec94c27d"></a>
## mean

`function` · `datafusion_expr_common::statistics::ExponentialDistribution::mean` · datafusion-expr-common 55.1.0

```rust
fn mean(&self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::ExponentialDistribution", "path": "ExponentialDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [370, 1], "end": [455, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:418`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-099caa9dc7c2b849eea8642d"></a>
## median

`function` · `datafusion_expr_common::statistics::ExponentialDistribution::median` · datafusion-expr-common 55.1.0

```rust
fn median(&self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::ExponentialDistribution", "path": "ExponentialDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [370, 1], "end": [455, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:429`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-60fc1012db72569759d0d6be"></a>
## offset

`function` · `datafusion_expr_common::statistics::ExponentialDistribution::offset` · datafusion-expr-common 55.1.0

```rust
fn offset(&self) -> &ScalarValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::ExponentialDistribution", "path": "ExponentialDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [370, 1], "end": [455, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:410`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af7f86979f895d96c56a482a"></a>
## positive_tail

`function` · `datafusion_expr_common::statistics::ExponentialDistribution::positive_tail` · datafusion-expr-common 55.1.0

```rust
fn positive_tail(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::ExponentialDistribution", "path": "ExponentialDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [370, 1], "end": [455, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:414`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d0417895c6414e88067021d"></a>
## range

`function` · `datafusion_expr_common::statistics::ExponentialDistribution::range` · datafusion-expr-common 55.1.0

```rust
fn range(&self) -> Result<Interval>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::ExponentialDistribution", "path": "ExponentialDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [370, 1], "end": [455, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:447`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9e5943913d84ce6112c19f3"></a>
## rate

`function` · `datafusion_expr_common::statistics::ExponentialDistribution::rate` · datafusion-expr-common 55.1.0

```rust
fn rate(&self) -> &ScalarValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::ExponentialDistribution", "path": "ExponentialDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [370, 1], "end": [455, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:406`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-67d90b70b732aa080e0f3ae0"></a>
## variance

`function` · `datafusion_expr_common::statistics::ExponentialDistribution::variance` · datafusion-expr-common 55.1.0

```rust
fn variance(&self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::ExponentialDistribution", "path": "ExponentialDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [370, 1], "end": [455, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:440`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
