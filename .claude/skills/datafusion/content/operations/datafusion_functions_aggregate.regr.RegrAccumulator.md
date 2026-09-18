# `datafusion_functions_aggregate::regr::RegrAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.regr.RegrAccumulator.json).

<a id="op-486f70431ee5c933b1402e37"></a>
## RegrAccumulator

`struct` · `datafusion_functions_aggregate::regr::RegrAccumulator` · datafusion-functions-aggregate 55.1.0

```rust
struct RegrAccumulator
```

Source: `src/regr.rs:559`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

`RegrAccumulator` is used to compute linear regression aggregate functions
by maintaining statistics needed to compute them in an online fashion.

This struct uses Welford's online algorithm for calculating variance and covariance:
<https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance#Welford's_online_algorithm>

Given the statistics, the following aggregate functions can be calculated:

- `regr_slope(y, x)`: Slope of the linear regression line, calculated as:
  cov_pop(x, y) / var_pop(x).
  It represents the expected change in Y for a one-unit change in X.

- `regr_intercept(y, x)`: Intercept of the linear regression line, calculated as:
  mean_y - (regr_slope(y, x) * mean_x).
  It represents the expected value of Y when X is 0.

- `regr_count(y, x)`: Count of the non-null(both x and y) input rows.

- `regr_r2(y, x)`: R-squared value (coefficient of determination), calculated as:
  (cov_pop(x, y) ^ 2) / (var_pop(x) * var_pop(y)).
  It provides a measure of how well the model's predictions match the observed data.

- `regr_avgx(y, x)`: Average of the independent variable X, calculated as: mean_x.

- `regr_avgy(y, x)`: Average of the dependent variable Y, calculated as: mean_y.

- `regr_sxx(y, x)`: Sum of squares of the independent variable X, calculated as:
  m2_x.

- `regr_syy(y, x)`: Sum of squares of the dependent variable Y, calculated as:
  m2_y.

- `regr_sxy(y, x)`: Sum of products of paired values, calculated as:
  algo_const.

Here's how the statistics maintained in this struct are calculated:
- `cov_pop(x, y)`: algo_const / count.
- `var_pop(x)`: m2_x / count.
- `var_pop(y)`: m2_y / count.

<a id="op-abd499410e774f1baacee828"></a>
## evaluate

`function` · `datafusion_functions_aggregate::regr::RegrAccumulator::evaluate` · datafusion-functions-aggregate 55.1.0

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::regr::RegrAccumulator", "path": "RegrAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [584, 1], "end": [773, 2], "filename": "src/regr.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/regr.rs:728`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11df35b31dd38a109e171d92"></a>
## fmt

`function` · `datafusion_functions_aggregate::regr::RegrAccumulator::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::regr::RegrAccumulator", "path": "RegrAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [558, 10], "end": [558, 15], "filename": "src/regr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/regr.rs:558`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f54a3284785f68d557950db"></a>
## merge_batch

`function` · `datafusion_functions_aggregate::regr::RegrAccumulator::merge_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::regr::RegrAccumulator", "path": "RegrAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [584, 1], "end": [773, 2], "filename": "src/regr.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/regr.rs:666`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af8b83e65748ba6ce31dcef4"></a>
## retract_batch

`function` · `datafusion_functions_aggregate::regr::RegrAccumulator::retract_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn retract_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::regr::RegrAccumulator", "path": "RegrAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [584, 1], "end": [773, 2], "filename": "src/regr.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/regr.rs:629`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a1c5b246fc5cbbfe95b11f1"></a>
## size

`function` · `datafusion_functions_aggregate::regr::RegrAccumulator::size` · datafusion-functions-aggregate 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::regr::RegrAccumulator", "path": "RegrAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [584, 1], "end": [773, 2], "filename": "src/regr.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/regr.rs:770`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7569570f500eb1922fd22c35"></a>
## state

`function` · `datafusion_functions_aggregate::regr::RegrAccumulator::state` · datafusion-functions-aggregate 55.1.0

```rust
fn state(&mut self) -> Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::regr::RegrAccumulator", "path": "RegrAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [584, 1], "end": [773, 2], "filename": "src/regr.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/regr.rs:585`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3ee6c698d6d0ac898c6a256"></a>
## supports_retract_batch

`function` · `datafusion_functions_aggregate::regr::RegrAccumulator::supports_retract_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn supports_retract_batch(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::regr::RegrAccumulator", "path": "RegrAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [584, 1], "end": [773, 2], "filename": "src/regr.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/regr.rs:625`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7140bf85957954300740d99a"></a>
## try_new

`function` · `datafusion_functions_aggregate::regr::RegrAccumulator::try_new` · datafusion-functions-aggregate 55.1.0

```rust
fn try_new(regr_type: &RegrType) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::regr::RegrAccumulator", "path": "RegrAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [569, 1], "end": [582, 2], "filename": "src/regr.rs"}, "trait": null, "trait_path": null}`

Source: `src/regr.rs:571`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Creates a new `RegrAccumulator`

<a id="op-f1f018e4249d594f500bf6c2"></a>
## update_batch

`function` · `datafusion_functions_aggregate::regr::RegrAccumulator::update_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::regr::RegrAccumulator", "path": "RegrAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [584, 1], "end": [773, 2], "filename": "src/regr.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/regr.rs:596`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
