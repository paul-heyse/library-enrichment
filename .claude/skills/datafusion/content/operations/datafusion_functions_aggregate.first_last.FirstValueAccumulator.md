# `datafusion_functions_aggregate::first_last::FirstValueAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.first_last.FirstValueAccumulator.json).

<a id="op-9abf9815e0528df48d20724b"></a>
## FirstValueAccumulator

`struct` · `datafusion_functions_aggregate::first_last::FirstValueAccumulator` · datafusion-functions-aggregate 55.1.0

```rust
struct FirstValueAccumulator
```

Source: `src/first_last.rs:849`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a2e45212e8d6f6801e3df52"></a>
## evaluate

`function` · `datafusion_functions_aggregate::first_last::FirstValueAccumulator::evaluate` · datafusion-functions-aggregate 55.1.0

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::FirstValueAccumulator", "path": "FirstValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [945, 1], "end": [1013, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/first_last.rs:1003`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2348e8952e0faadbd5417ff8"></a>
## fmt

`function` · `datafusion_functions_aggregate::first_last::FirstValueAccumulator::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::FirstValueAccumulator", "path": "FirstValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [848, 10], "end": [848, 15], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/first_last.rs:848`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02ad00270095b28e3f1802c7"></a>
## merge_batch

`function` · `datafusion_functions_aggregate::first_last::FirstValueAccumulator::merge_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::FirstValueAccumulator", "path": "FirstValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [945, 1], "end": [1013, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/first_last.rs:967`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c765ef044f7cca4fa9e8c31"></a>
## size

`function` · `datafusion_functions_aggregate::first_last::FirstValueAccumulator::size` · datafusion-functions-aggregate 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::FirstValueAccumulator", "path": "FirstValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [945, 1], "end": [1013, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/first_last.rs:1007`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb6bb1259681097cec5887a5"></a>
## state

`function` · `datafusion_functions_aggregate::first_last::FirstValueAccumulator::state` · datafusion-functions-aggregate 55.1.0

```rust
fn state(&mut self) -> Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::FirstValueAccumulator", "path": "FirstValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [945, 1], "end": [1013, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/first_last.rs:946`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb76c5f985ee26c3aaf89cb1"></a>
## try_new

`function` · `datafusion_functions_aggregate::first_last::FirstValueAccumulator::try_new` · datafusion-functions-aggregate 55.1.0

```rust
fn try_new(data_type: &DataType, ordering_dtypes: &[DataType], ordering_req: LexOrdering, is_input_pre_ordered: bool, ignore_nulls: bool) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::FirstValueAccumulator", "path": "FirstValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [866, 1], "end": [943, 2], "filename": "src/first_last.rs"}, "trait": null, "trait_path": null}`

Source: `src/first_last.rs:868`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Creates a new `FirstValueAccumulator` for the given `data_type`.

<a id="op-c3c3ef16f71f686d653c7098"></a>
## update_batch

`function` · `datafusion_functions_aggregate::first_last::FirstValueAccumulator::update_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::FirstValueAccumulator", "path": "FirstValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [945, 1], "end": [1013, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/first_last.rs:953`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
