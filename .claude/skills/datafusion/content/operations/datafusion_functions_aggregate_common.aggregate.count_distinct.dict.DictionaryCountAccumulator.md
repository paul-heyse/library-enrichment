# `datafusion_functions_aggregate_common::aggregate::count_distinct::dict::DictionaryCountAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.aggregate.count_distinct.dict.DictionaryCountAccumulator.json).

<a id="op-dbd100a16a63da071e6418f9"></a>
## DictionaryCountAccumulator

`struct` · `datafusion_functions_aggregate_common::aggregate::count_distinct::dict::DictionaryCountAccumulator` · datafusion-functions-aggregate-common 55.1.0

```rust
struct DictionaryCountAccumulator
```

Source: `src/aggregate/count_distinct/dict.rs:25`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2809980be4b2d7d68d0d9685"></a>
## evaluate

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::dict::DictionaryCountAccumulator::evaluate` · datafusion-functions-aggregate-common 55.1.0

```rust
fn evaluate(&mut self) -> datafusion_common::Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::dict::DictionaryCountAccumulator", "path": "DictionaryCountAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [70, 2], "filename": "src/aggregate/count_distinct/dict.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/dict.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a3400ad39eb223061f0b801"></a>
## fmt

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::dict::DictionaryCountAccumulator::fmt` · datafusion-functions-aggregate-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::dict::DictionaryCountAccumulator", "path": "DictionaryCountAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 10], "end": [24, 15], "filename": "src/aggregate/count_distinct/dict.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aggregate/count_distinct/dict.rs:24`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d841f4409a6d433d18cbb9d2"></a>
## merge_batch

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::dict::DictionaryCountAccumulator::merge_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::dict::DictionaryCountAccumulator", "path": "DictionaryCountAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [70, 2], "filename": "src/aggregate/count_distinct/dict.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/dict.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69d82413d0abf6466b98e7ef"></a>
## new

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::dict::DictionaryCountAccumulator::new` · datafusion-functions-aggregate-common 55.1.0

```rust
fn new(inner: Box<dyn Accumulator>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::dict::DictionaryCountAccumulator", "path": "DictionaryCountAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 1], "end": [33, 2], "filename": "src/aggregate/count_distinct/dict.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate/count_distinct/dict.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70fcd62adc8a5428a957eb2a"></a>
## size

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::dict::DictionaryCountAccumulator::size` · datafusion-functions-aggregate-common 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::dict::DictionaryCountAccumulator", "path": "DictionaryCountAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [70, 2], "filename": "src/aggregate/count_distinct/dict.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/dict.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-196efc9e83febbad343b02d3"></a>
## state

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::dict::DictionaryCountAccumulator::state` · datafusion-functions-aggregate-common 55.1.0

```rust
fn state(&mut self) -> datafusion_common::Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::dict::DictionaryCountAccumulator", "path": "DictionaryCountAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [70, 2], "filename": "src/aggregate/count_distinct/dict.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/dict.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6003317e9bf5b98a09cd1f28"></a>
## update_batch

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::dict::DictionaryCountAccumulator::update_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef]) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::dict::DictionaryCountAccumulator", "path": "DictionaryCountAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [70, 2], "filename": "src/aggregate/count_distinct/dict.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/dict.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
