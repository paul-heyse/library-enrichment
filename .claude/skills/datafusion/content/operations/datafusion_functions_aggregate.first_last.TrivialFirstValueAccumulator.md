# `datafusion_functions_aggregate::first_last::TrivialFirstValueAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.first_last.TrivialFirstValueAccumulator.json).

<a id="op-0868a899f6e1b976c024ec07"></a>
## TrivialFirstValueAccumulator

`struct` · `datafusion_functions_aggregate::first_last::TrivialFirstValueAccumulator` · datafusion-functions-aggregate 55.1.0

```rust
struct TrivialFirstValueAccumulator
```

Source: `src/first_last.rs:770`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

This accumulator is used when there is no ordering specified for the
`FIRST_VALUE` aggregation. It simply returns the first value it sees
according to the pre-existing ordering of the input data, and provides
a fast path for this case without needing to maintain any ordering state.

<a id="op-f79fad048cc64e5d9b6d20f6"></a>
## evaluate

`function` · `datafusion_functions_aggregate::first_last::TrivialFirstValueAccumulator::evaluate` · datafusion-functions-aggregate 55.1.0

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::TrivialFirstValueAccumulator", "path": "TrivialFirstValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [789, 1], "end": [846, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/first_last.rs:839`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f831d898eb8837a141989b4"></a>
## fmt

`function` · `datafusion_functions_aggregate::first_last::TrivialFirstValueAccumulator::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::TrivialFirstValueAccumulator", "path": "TrivialFirstValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [769, 10], "end": [769, 15], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/first_last.rs:769`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27387478c0ee6fb83de40165"></a>
## merge_batch

`function` · `datafusion_functions_aggregate::first_last::TrivialFirstValueAccumulator::merge_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::TrivialFirstValueAccumulator", "path": "TrivialFirstValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [789, 1], "end": [846, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/first_last.rs:820`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12266dfb4947fdc9aba921ac"></a>
## size

`function` · `datafusion_functions_aggregate::first_last::TrivialFirstValueAccumulator::size` · datafusion-functions-aggregate 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::TrivialFirstValueAccumulator", "path": "TrivialFirstValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [789, 1], "end": [846, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/first_last.rs:843`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-460b29180a2e57498201fb43"></a>
## state

`function` · `datafusion_functions_aggregate::first_last::TrivialFirstValueAccumulator::state` · datafusion-functions-aggregate 55.1.0

```rust
fn state(&mut self) -> Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::TrivialFirstValueAccumulator", "path": "TrivialFirstValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [789, 1], "end": [846, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/first_last.rs:790`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6134eee6bf65fdbce484b62"></a>
## try_new

`function` · `datafusion_functions_aggregate::first_last::TrivialFirstValueAccumulator::try_new` · datafusion-functions-aggregate 55.1.0

```rust
fn try_new(data_type: &DataType, ignore_nulls: bool) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::TrivialFirstValueAccumulator", "path": "TrivialFirstValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [778, 1], "end": [787, 2], "filename": "src/first_last.rs"}, "trait": null, "trait_path": null}`

Source: `src/first_last.rs:780`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Creates a new `TrivialFirstValueAccumulator` for the given `data_type`.

<a id="op-51c2d40ac741ae7840a8d278"></a>
## update_batch

`function` · `datafusion_functions_aggregate::first_last::TrivialFirstValueAccumulator::update_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::TrivialFirstValueAccumulator", "path": "TrivialFirstValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [789, 1], "end": [846, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/first_last.rs:794`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
