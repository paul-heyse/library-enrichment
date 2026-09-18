# `datafusion_functions_aggregate::covariance::CovarianceAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.covariance.CovarianceAccumulator.json).

<a id="op-ad761a4727a32834ff7b6ddd"></a>
## CovarianceAccumulator

`struct` · `datafusion_functions_aggregate::covariance::CovarianceAccumulator` · datafusion-functions-aggregate 55.1.0

```rust
struct CovarianceAccumulator
```

Source: `src/covariance.rs:225`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

An accumulator to compute covariance
The algorithm used is an online implementation and numerically stable. It is derived from the following paper
for calculating variance:
Welford, B. P. (1962). "Note on a method for calculating corrected sums of squares and products".
Technometrics. 4 (3): 419–420. doi:10.2307/1266577. JSTOR 1266577.

The algorithm has been analyzed here:
Ling, Robert F. (1974). "Comparison of Several Algorithms for Computing Sample Means and Variances".
Journal of the American Statistical Association. 69 (348): 859–866. doi:10.2307/2286154. JSTOR 2286154.

Though it is not covered in the original paper but is based on the same idea, as a result the algorithm is online,
parallelize and numerically stable.

<a id="op-6c7813bf8f81deff24bebaa8"></a>
## evaluate

`function` · `datafusion_functions_aggregate::covariance::CovarianceAccumulator::evaluate` · datafusion-functions-aggregate 55.1.0

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovarianceAccumulator", "path": "CovarianceAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [388, 2], "filename": "src/covariance.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/covariance.rs:362`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05b0fb69d3a2e16d9c9d3df2"></a>
## fmt

`function` · `datafusion_functions_aggregate::covariance::CovarianceAccumulator::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovarianceAccumulator", "path": "CovarianceAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [224, 10], "end": [224, 15], "filename": "src/covariance.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/covariance.rs:224`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5037e9559536ee5ee9706cf3"></a>
## get_algo_const

`function` · `datafusion_functions_aggregate::covariance::CovarianceAccumulator::get_algo_const` · datafusion-functions-aggregate 55.1.0

```rust
fn get_algo_const(&self) -> f64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovarianceAccumulator", "path": "CovarianceAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [233, 1], "end": [260, 2], "filename": "src/covariance.rs"}, "trait": null, "trait_path": null}`

Source: `src/covariance.rs:257`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc25265dc276106e10e8dc67"></a>
## get_count

`function` · `datafusion_functions_aggregate::covariance::CovarianceAccumulator::get_count` · datafusion-functions-aggregate 55.1.0

```rust
fn get_count(&self) -> u64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovarianceAccumulator", "path": "CovarianceAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [233, 1], "end": [260, 2], "filename": "src/covariance.rs"}, "trait": null, "trait_path": null}`

Source: `src/covariance.rs:245`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1439039b5f564092962edc6"></a>
## get_mean1

`function` · `datafusion_functions_aggregate::covariance::CovarianceAccumulator::get_mean1` · datafusion-functions-aggregate 55.1.0

```rust
fn get_mean1(&self) -> f64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovarianceAccumulator", "path": "CovarianceAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [233, 1], "end": [260, 2], "filename": "src/covariance.rs"}, "trait": null, "trait_path": null}`

Source: `src/covariance.rs:249`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7528bd16e4661d0d02eaa162"></a>
## get_mean2

`function` · `datafusion_functions_aggregate::covariance::CovarianceAccumulator::get_mean2` · datafusion-functions-aggregate 55.1.0

```rust
fn get_mean2(&self) -> f64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovarianceAccumulator", "path": "CovarianceAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [233, 1], "end": [260, 2], "filename": "src/covariance.rs"}, "trait": null, "trait_path": null}`

Source: `src/covariance.rs:253`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-047502a111ffda1aa1d72d52"></a>
## merge_batch

`function` · `datafusion_functions_aggregate::covariance::CovarianceAccumulator::merge_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovarianceAccumulator", "path": "CovarianceAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [388, 2], "filename": "src/covariance.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/covariance.rs:332`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4128f9a5b410960db7ccf69e"></a>
## retract_batch

`function` · `datafusion_functions_aggregate::covariance::CovarianceAccumulator::retract_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn retract_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovarianceAccumulator", "path": "CovarianceAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [388, 2], "filename": "src/covariance.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/covariance.rs:298`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e77bdf1a6081c30864a1b002"></a>
## size

`function` · `datafusion_functions_aggregate::covariance::CovarianceAccumulator::size` · datafusion-functions-aggregate 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovarianceAccumulator", "path": "CovarianceAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [388, 2], "filename": "src/covariance.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/covariance.rs:381`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c9372afa36ac32a259e9636e"></a>
## state

`function` · `datafusion_functions_aggregate::covariance::CovarianceAccumulator::state` · datafusion-functions-aggregate 55.1.0

```rust
fn state(&mut self) -> Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovarianceAccumulator", "path": "CovarianceAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [388, 2], "filename": "src/covariance.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/covariance.rs:263`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3fbbba6e624cc77bf1cfba92"></a>
## supports_retract_batch

`function` · `datafusion_functions_aggregate::covariance::CovarianceAccumulator::supports_retract_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn supports_retract_batch(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovarianceAccumulator", "path": "CovarianceAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [388, 2], "filename": "src/covariance.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/covariance.rs:385`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13b099c9178022a9b8370cb9"></a>
## try_new

`function` · `datafusion_functions_aggregate::covariance::CovarianceAccumulator::try_new` · datafusion-functions-aggregate 55.1.0

```rust
fn try_new(s_type: StatsType) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovarianceAccumulator", "path": "CovarianceAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [233, 1], "end": [260, 2], "filename": "src/covariance.rs"}, "trait": null, "trait_path": null}`

Source: `src/covariance.rs:235`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Creates a new `CovarianceAccumulator`

<a id="op-33ca74e34a1a66de12bc2fd0"></a>
## update_batch

`function` · `datafusion_functions_aggregate::covariance::CovarianceAccumulator::update_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::covariance::CovarianceAccumulator", "path": "CovarianceAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [388, 2], "filename": "src/covariance.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/covariance.rs:272`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
