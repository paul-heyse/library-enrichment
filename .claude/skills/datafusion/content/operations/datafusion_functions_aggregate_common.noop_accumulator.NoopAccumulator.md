# `datafusion_functions_aggregate_common::noop_accumulator::NoopAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.noop_accumulator.NoopAccumulator.json).

<a id="op-86313152eb16a09f3a00da25"></a>
## NoopAccumulator

`struct` · `datafusion_functions_aggregate_common::noop_accumulator::NoopAccumulator` · datafusion-functions-aggregate-common 55.1.0

```rust
struct NoopAccumulator
```

Source: `src/noop_accumulator.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

[`Accumulator`](../operations/datafusion_expr_common.accumulator.Accumulator.md#op-2911d7ffb2098886b7dd6ba8) that does no work and always returns a fixed value (default
of `NULL` but can be customized).

Useful for aggregate functions that need to handle an input of [`DataType::Null`]
that does no work.

[`DataType::Null`]: arrow::datatypes::DataType::Null

<a id="op-5186669bed8acc512f801259"></a>
## default

`function` · `datafusion_functions_aggregate_common::noop_accumulator::NoopAccumulator::default` · datafusion-functions-aggregate-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::noop_accumulator::NoopAccumulator", "path": "NoopAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [46, 2], "filename": "src/noop_accumulator.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/noop_accumulator.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-211e57cd65ccfacafb4d0841"></a>
## evaluate

`function` · `datafusion_functions_aggregate_common::noop_accumulator::NoopAccumulator::evaluate` · datafusion-functions-aggregate-common 55.1.0

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::noop_accumulator::NoopAccumulator", "path": "NoopAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [70, 2], "filename": "src/noop_accumulator.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/noop_accumulator.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-047e4d19a708dae8e9741e2d"></a>
## fmt

`function` · `datafusion_functions_aggregate_common::noop_accumulator::NoopAccumulator::fmt` · datafusion-functions-aggregate-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::noop_accumulator::NoopAccumulator", "path": "NoopAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 10], "end": [29, 15], "filename": "src/noop_accumulator.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/noop_accumulator.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7bfbd173d1e1f2b622f26c4"></a>
## merge_batch

`function` · `datafusion_functions_aggregate_common::noop_accumulator::NoopAccumulator::merge_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn merge_batch(&mut self, _states: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::noop_accumulator::NoopAccumulator", "path": "NoopAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [70, 2], "filename": "src/noop_accumulator.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/noop_accumulator.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39f3d8e2199a0877e202bedb"></a>
## new

`function` · `datafusion_functions_aggregate_common::noop_accumulator::NoopAccumulator::new` · datafusion-functions-aggregate-common 55.1.0

```rust
fn new(evaluate_value: ScalarValue) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::noop_accumulator::NoopAccumulator", "path": "NoopAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [38, 2], "filename": "src/noop_accumulator.rs"}, "trait": null, "trait_path": null}`

Source: `src/noop_accumulator.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1717f1f7b21f0d0ef0844898"></a>
## size

`function` · `datafusion_functions_aggregate_common::noop_accumulator::NoopAccumulator::size` · datafusion-functions-aggregate-common 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::noop_accumulator::NoopAccumulator", "path": "NoopAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [70, 2], "filename": "src/noop_accumulator.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/noop_accumulator.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f29231b80b7b7dda129b1187"></a>
## state

`function` · `datafusion_functions_aggregate_common::noop_accumulator::NoopAccumulator::state` · datafusion-functions-aggregate-common 55.1.0

```rust
fn state(&mut self) -> Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::noop_accumulator::NoopAccumulator", "path": "NoopAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [70, 2], "filename": "src/noop_accumulator.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/noop_accumulator.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9124ff8ccc62867c4d32360b"></a>
## update_batch

`function` · `datafusion_functions_aggregate_common::noop_accumulator::NoopAccumulator::update_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn update_batch(&mut self, _values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::noop_accumulator::NoopAccumulator", "path": "NoopAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [70, 2], "filename": "src/noop_accumulator.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/noop_accumulator.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
