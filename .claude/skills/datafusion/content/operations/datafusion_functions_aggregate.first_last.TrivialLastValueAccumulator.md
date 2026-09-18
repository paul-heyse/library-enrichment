# `datafusion_functions_aggregate::first_last::TrivialLastValueAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.first_last.TrivialLastValueAccumulator.json).

<a id="op-858398e92e5395875ea8ac4b"></a>
## TrivialLastValueAccumulator

`struct` · `datafusion_functions_aggregate::first_last::TrivialLastValueAccumulator` · datafusion-functions-aggregate 55.1.0

```rust
struct TrivialLastValueAccumulator
```

Source: `src/first_last.rs:1160`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

This accumulator is used when there is no ordering specified for the
`LAST_VALUE` aggregation. It simply updates the last value it sees
according to the pre-existing ordering of the input data, and provides
a fast path for this case without needing to maintain any ordering state.

<a id="op-4b54478aaf4934fe6a470f54"></a>
## evaluate

`function` · `datafusion_functions_aggregate::first_last::TrivialLastValueAccumulator::evaluate` · datafusion-functions-aggregate 55.1.0

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::TrivialLastValueAccumulator", "path": "TrivialLastValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1181, 1], "end": [1233, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/first_last.rs:1226`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2b1d0a85a2d8536db0e0cf1"></a>
## fmt

`function` · `datafusion_functions_aggregate::first_last::TrivialLastValueAccumulator::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::TrivialLastValueAccumulator", "path": "TrivialLastValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1159, 10], "end": [1159, 15], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/first_last.rs:1159`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b172cf03c384ef3f880f3f12"></a>
## merge_batch

`function` · `datafusion_functions_aggregate::first_last::TrivialLastValueAccumulator::merge_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::TrivialLastValueAccumulator", "path": "TrivialLastValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1181, 1], "end": [1233, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/first_last.rs:1210`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc484091cd10f53ed3037333"></a>
## size

`function` · `datafusion_functions_aggregate::first_last::TrivialLastValueAccumulator::size` · datafusion-functions-aggregate 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::TrivialLastValueAccumulator", "path": "TrivialLastValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1181, 1], "end": [1233, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/first_last.rs:1230`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c5f369e422bdc9db57155b6"></a>
## state

`function` · `datafusion_functions_aggregate::first_last::TrivialLastValueAccumulator::state` · datafusion-functions-aggregate 55.1.0

```rust
fn state(&mut self) -> Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::TrivialLastValueAccumulator", "path": "TrivialLastValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1181, 1], "end": [1233, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/first_last.rs:1182`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7b9be5fc6148d978d2038f4"></a>
## try_new

`function` · `datafusion_functions_aggregate::first_last::TrivialLastValueAccumulator::try_new` · datafusion-functions-aggregate 55.1.0

```rust
fn try_new(data_type: &DataType, ignore_nulls: bool) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::TrivialLastValueAccumulator", "path": "TrivialLastValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1170, 1], "end": [1179, 2], "filename": "src/first_last.rs"}, "trait": null, "trait_path": null}`

Source: `src/first_last.rs:1172`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Creates a new `TrivialLastValueAccumulator` for the given `data_type`.

<a id="op-e5be9e2dbd66b79994611a01"></a>
## update_batch

`function` · `datafusion_functions_aggregate::first_last::TrivialLastValueAccumulator::update_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::TrivialLastValueAccumulator", "path": "TrivialLastValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1181, 1], "end": [1233, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/first_last.rs:1186`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
