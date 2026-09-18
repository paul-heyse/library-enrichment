# `datafusion_functions_aggregate_common::min_max::MinAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.min_max.MinAccumulator.json).

<a id="op-739285b8a6810def4a6b2c05"></a>
## MinAccumulator

`struct` · `datafusion_functions_aggregate_common::min_max::MinAccumulator` · datafusion-functions-aggregate-common 55.1.0

```rust
struct MinAccumulator
```

Source: `src/min_max.rs:436`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

An accumulator to compute the minimum value

<a id="op-a2d33ce4c8e0e3ed4e78e221"></a>
## clone

`function` · `datafusion_functions_aggregate_common::min_max::MinAccumulator::clone` · datafusion-functions-aggregate-common 55.1.0

```rust
fn clone(&self) -> MinAccumulator
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::min_max::MinAccumulator", "path": "MinAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [435, 17], "end": [435, 22], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/min_max.rs:435`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52f9f4ef999fb430add36022"></a>
## evaluate

`function` · `datafusion_functions_aggregate_common::min_max::MinAccumulator::evaluate` · datafusion-functions-aggregate-common 55.1.0

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::min_max::MinAccumulator", "path": "MinAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [449, 1], "end": [474, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/min_max.rs:467`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b04089055b574e13433dc5de"></a>
## fmt

`function` · `datafusion_functions_aggregate_common::min_max::MinAccumulator::fmt` · datafusion-functions-aggregate-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::min_max::MinAccumulator", "path": "MinAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [435, 10], "end": [435, 15], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/min_max.rs:435`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bdeff0e55779d19595d6cd61"></a>
## merge_batch

`function` · `datafusion_functions_aggregate_common::min_max::MinAccumulator::merge_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::min_max::MinAccumulator", "path": "MinAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [449, 1], "end": [474, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/min_max.rs:463`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-205d669a77d50f1332a3561d"></a>
## size

`function` · `datafusion_functions_aggregate_common::min_max::MinAccumulator::size` · datafusion-functions-aggregate-common 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::min_max::MinAccumulator", "path": "MinAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [449, 1], "end": [474, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/min_max.rs:471`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c9c93b69e75a6e2e311d165"></a>
## state

`function` · `datafusion_functions_aggregate_common::min_max::MinAccumulator::state` · datafusion-functions-aggregate-common 55.1.0

```rust
fn state(&mut self) -> Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::min_max::MinAccumulator", "path": "MinAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [449, 1], "end": [474, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/min_max.rs:450`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-215d6515b5a6d53575b2e50c"></a>
## try_new

`function` · `datafusion_functions_aggregate_common::min_max::MinAccumulator::try_new` · datafusion-functions-aggregate-common 55.1.0

```rust
fn try_new(datatype: &DataType) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::min_max::MinAccumulator", "path": "MinAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [440, 1], "end": [447, 2], "filename": "src/min_max.rs"}, "trait": null, "trait_path": null}`

Source: `src/min_max.rs:442`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

new min accumulator

<a id="op-82738a2af2c4b37065877e8d"></a>
## update_batch

`function` · `datafusion_functions_aggregate_common::min_max::MinAccumulator::update_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::min_max::MinAccumulator", "path": "MinAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [449, 1], "end": [474, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/min_max.rs:454`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
