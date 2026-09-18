# `datafusion_functions_aggregate::variance::VarianceAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.variance.VarianceAccumulator.json).

<a id="op-e7653ba7f739e82cbc9d49cc"></a>
## VarianceAccumulator

`struct` · `datafusion_functions_aggregate::variance::VarianceAccumulator` · datafusion-functions-aggregate 55.1.0

```rust
struct VarianceAccumulator
```

Source: `src/variance.rs:267`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

An accumulator to compute variance
The algorithm used is an online implementation and numerically stable. It is based on this paper:
Welford, B. P. (1962). "Note on a method for calculating corrected sums of squares and products".
Technometrics. 4 (3): 419–420. doi:10.2307/1266577. JSTOR 1266577.

The algorithm has been analyzed here:
Ling, Robert F. (1974). "Comparison of Several Algorithms for Computing Sample Means and Variances".
Journal of the American Statistical Association. 69 (348): 859–866. doi:10.2307/2286154. JSTOR 2286154.

<a id="op-c3b181941052daf598392c0f"></a>
## evaluate

`function` · `datafusion_functions_aggregate::variance::VarianceAccumulator::evaluate` · datafusion-functions-aggregate 55.1.0

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceAccumulator", "path": "VarianceAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [346, 1], "end": [429, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/variance.rs:397`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d5773051950cebe770450ca"></a>
## fmt

`function` · `datafusion_functions_aggregate::variance::VarianceAccumulator::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceAccumulator", "path": "VarianceAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [266, 10], "end": [266, 15], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/variance.rs:266`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09bb7e0a7d3b29d4ba82e8a3"></a>
## get_count

`function` · `datafusion_functions_aggregate::variance::VarianceAccumulator::get_count` · datafusion-functions-aggregate 55.1.0

```rust
fn get_count(&self) -> u64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceAccumulator", "path": "VarianceAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [274, 1], "end": [296, 2], "filename": "src/variance.rs"}, "trait": null, "trait_path": null}`

Source: `src/variance.rs:285`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e331a06ecd161cf356a5e9b4"></a>
## get_m2

`function` · `datafusion_functions_aggregate::variance::VarianceAccumulator::get_m2` · datafusion-functions-aggregate 55.1.0

```rust
fn get_m2(&self) -> f64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceAccumulator", "path": "VarianceAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [274, 1], "end": [296, 2], "filename": "src/variance.rs"}, "trait": null, "trait_path": null}`

Source: `src/variance.rs:293`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f397a738f22249bba52bbeb1"></a>
## get_mean

`function` · `datafusion_functions_aggregate::variance::VarianceAccumulator::get_mean` · datafusion-functions-aggregate 55.1.0

```rust
fn get_mean(&self) -> f64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceAccumulator", "path": "VarianceAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [274, 1], "end": [296, 2], "filename": "src/variance.rs"}, "trait": null, "trait_path": null}`

Source: `src/variance.rs:289`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-031884f3a91bb26ae5ea2e6a"></a>
## merge_batch

`function` · `datafusion_functions_aggregate::variance::VarianceAccumulator::merge_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceAccumulator", "path": "VarianceAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [346, 1], "end": [429, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/variance.rs:375`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-481d20dd62d876540dfed864"></a>
## retract_batch

`function` · `datafusion_functions_aggregate::variance::VarianceAccumulator::retract_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn retract_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceAccumulator", "path": "VarianceAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [346, 1], "end": [429, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/variance.rs:365`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5ea569d95e7f06471f523f6"></a>
## size

`function` · `datafusion_functions_aggregate::variance::VarianceAccumulator::size` · datafusion-functions-aggregate 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceAccumulator", "path": "VarianceAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [346, 1], "end": [429, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/variance.rs:422`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9dedb7ccb5f8dbea3dbb15f9"></a>
## state

`function` · `datafusion_functions_aggregate::variance::VarianceAccumulator::state` · datafusion-functions-aggregate 55.1.0

```rust
fn state(&mut self) -> Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceAccumulator", "path": "VarianceAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [346, 1], "end": [429, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/variance.rs:347`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6f7779454842300c07ee69f3"></a>
## supports_retract_batch

`function` · `datafusion_functions_aggregate::variance::VarianceAccumulator::supports_retract_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn supports_retract_batch(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceAccumulator", "path": "VarianceAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [346, 1], "end": [429, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/variance.rs:426`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b06ca3823a438df8d868cc37"></a>
## try_new

`function` · `datafusion_functions_aggregate::variance::VarianceAccumulator::try_new` · datafusion-functions-aggregate 55.1.0

```rust
fn try_new(s_type: StatsType) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceAccumulator", "path": "VarianceAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [274, 1], "end": [296, 2], "filename": "src/variance.rs"}, "trait": null, "trait_path": null}`

Source: `src/variance.rs:276`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Creates a new `VarianceAccumulator`

<a id="op-15cca7762e936bd64bad801d"></a>
## update_batch

`function` · `datafusion_functions_aggregate::variance::VarianceAccumulator::update_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceAccumulator", "path": "VarianceAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [346, 1], "end": [429, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/variance.rs:355`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
