# `datafusion_functions_aggregate::correlation::CorrelationAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.correlation.CorrelationAccumulator.json).

<a id="op-f3af4b289c27026604903ec0"></a>
## CorrelationAccumulator

`struct` · `datafusion_functions_aggregate::correlation::CorrelationAccumulator` · datafusion-functions-aggregate 55.1.0

```rust
struct CorrelationAccumulator
```

Source: `src/correlation.rs:152`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

An accumulator to compute correlation

<a id="op-2338d173953679b90cbebf7a"></a>
## evaluate

`function` · `datafusion_functions_aggregate::correlation::CorrelationAccumulator::evaluate` · datafusion-functions-aggregate 55.1.0

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::correlation::CorrelationAccumulator", "path": "CorrelationAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [169, 1], "end": [288, 2], "filename": "src/correlation.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/correlation.rs:192`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fcf920c87609dca5ad5f45e3"></a>
## fmt

`function` · `datafusion_functions_aggregate::correlation::CorrelationAccumulator::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::correlation::CorrelationAccumulator", "path": "CorrelationAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [151, 10], "end": [151, 15], "filename": "src/correlation.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/correlation.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fac31ed2a0f1fa303886883b"></a>
## merge_batch

`function` · `datafusion_functions_aggregate::correlation::CorrelationAccumulator::merge_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::correlation::CorrelationAccumulator", "path": "CorrelationAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [169, 1], "end": [288, 2], "filename": "src/correlation.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/correlation.rs:244`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee88db8c1ca2ad9b34857c02"></a>
## retract_batch

`function` · `datafusion_functions_aggregate::correlation::CorrelationAccumulator::retract_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn retract_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::correlation::CorrelationAccumulator", "path": "CorrelationAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [169, 1], "end": [288, 2], "filename": "src/correlation.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/correlation.rs:268`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b60946663d0787891a7050fa"></a>
## size

`function` · `datafusion_functions_aggregate::correlation::CorrelationAccumulator::size` · datafusion-functions-aggregate 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::correlation::CorrelationAccumulator", "path": "CorrelationAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [169, 1], "end": [288, 2], "filename": "src/correlation.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/correlation.rs:225`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b2cca8d6ef03c5749e67434"></a>
## state

`function` · `datafusion_functions_aggregate::correlation::CorrelationAccumulator::state` · datafusion-functions-aggregate 55.1.0

```rust
fn state(&mut self) -> Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::correlation::CorrelationAccumulator", "path": "CorrelationAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [169, 1], "end": [288, 2], "filename": "src/correlation.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/correlation.rs:233`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-147e5f45c349060f30bb03c8"></a>
## supports_retract_batch

`function` · `datafusion_functions_aggregate::correlation::CorrelationAccumulator::supports_retract_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn supports_retract_batch(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::correlation::CorrelationAccumulator", "path": "CorrelationAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [169, 1], "end": [288, 2], "filename": "src/correlation.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/correlation.rs:285`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d03938683a693006cafcc2a5"></a>
## try_new

`function` · `datafusion_functions_aggregate::correlation::CorrelationAccumulator::try_new` · datafusion-functions-aggregate 55.1.0

```rust
fn try_new() -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::correlation::CorrelationAccumulator", "path": "CorrelationAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [158, 1], "end": [167, 2], "filename": "src/correlation.rs"}, "trait": null, "trait_path": null}`

Source: `src/correlation.rs:160`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Creates a new `CorrelationAccumulator`

<a id="op-0384d522412a4db12936fb58"></a>
## update_batch

`function` · `datafusion_functions_aggregate::correlation::CorrelationAccumulator::update_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::correlation::CorrelationAccumulator", "path": "CorrelationAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [169, 1], "end": [288, 2], "filename": "src/correlation.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/correlation.rs:170`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
