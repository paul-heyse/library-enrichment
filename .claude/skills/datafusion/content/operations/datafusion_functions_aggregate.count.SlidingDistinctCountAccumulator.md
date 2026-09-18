# `datafusion_functions_aggregate::count::SlidingDistinctCountAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.count.SlidingDistinctCountAccumulator.json).

<a id="op-5e74d17807fd441b576ba2f2"></a>
## SlidingDistinctCountAccumulator

`struct` · `datafusion_functions_aggregate::count::SlidingDistinctCountAccumulator` · datafusion-functions-aggregate 55.1.0

```rust
struct SlidingDistinctCountAccumulator
```

Source: `src/count.rs:489`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12b50f1abd0dcf514b6db475"></a>
## evaluate

`function` · `datafusion_functions_aggregate::count::SlidingDistinctCountAccumulator::evaluate` · datafusion-functions-aggregate 55.1.0

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::count::SlidingDistinctCountAccumulator", "path": "SlidingDistinctCountAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [503, 1], "end": [571, 2], "filename": "src/count.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/count.rs:550`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a1563a28102b89723dde630"></a>
## fmt

`function` · `datafusion_functions_aggregate::count::SlidingDistinctCountAccumulator::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::count::SlidingDistinctCountAccumulator", "path": "SlidingDistinctCountAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [488, 10], "end": [488, 15], "filename": "src/count.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/count.rs:488`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ff03be00c4f08d8a70d68b4"></a>
## merge_batch

`function` · `datafusion_functions_aggregate::count::SlidingDistinctCountAccumulator::merge_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::count::SlidingDistinctCountAccumulator", "path": "SlidingDistinctCountAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [503, 1], "end": [571, 2], "filename": "src/count.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/count.rs:539`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79353ae53bf2462b2aaf45e5"></a>
## retract_batch

`function` · `datafusion_functions_aggregate::count::SlidingDistinctCountAccumulator::retract_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn retract_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::count::SlidingDistinctCountAccumulator", "path": "SlidingDistinctCountAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [503, 1], "end": [571, 2], "filename": "src/count.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/count.rs:523`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-964f421abe908f91cf35920c"></a>
## size

`function` · `datafusion_functions_aggregate::count::SlidingDistinctCountAccumulator::size` · datafusion-functions-aggregate 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::count::SlidingDistinctCountAccumulator", "path": "SlidingDistinctCountAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [503, 1], "end": [571, 2], "filename": "src/count.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/count.rs:558`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37e6283169d5077f58e25073"></a>
## state

`function` · `datafusion_functions_aggregate::count::SlidingDistinctCountAccumulator::state` · datafusion-functions-aggregate 55.1.0

```rust
fn state(&mut self) -> Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::count::SlidingDistinctCountAccumulator", "path": "SlidingDistinctCountAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [503, 1], "end": [571, 2], "filename": "src/count.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/count.rs:504`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fdcdacf1e1db866ea4a7e041"></a>
## supports_retract_batch

`function` · `datafusion_functions_aggregate::count::SlidingDistinctCountAccumulator::supports_retract_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn supports_retract_batch(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::count::SlidingDistinctCountAccumulator", "path": "SlidingDistinctCountAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [503, 1], "end": [571, 2], "filename": "src/count.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/count.rs:554`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8f88826685413e7784cee8f"></a>
## try_new

`function` · `datafusion_functions_aggregate::count::SlidingDistinctCountAccumulator::try_new` · datafusion-functions-aggregate 55.1.0

```rust
fn try_new(data_type: &DataType) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::count::SlidingDistinctCountAccumulator", "path": "SlidingDistinctCountAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [494, 1], "end": [501, 2], "filename": "src/count.rs"}, "trait": null, "trait_path": null}`

Source: `src/count.rs:495`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50dc700cfb2596487a5098e0"></a>
## update_batch

`function` · `datafusion_functions_aggregate::count::SlidingDistinctCountAccumulator::update_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::count::SlidingDistinctCountAccumulator", "path": "SlidingDistinctCountAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [503, 1], "end": [571, 2], "filename": "src/count.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/count.rs:512`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
