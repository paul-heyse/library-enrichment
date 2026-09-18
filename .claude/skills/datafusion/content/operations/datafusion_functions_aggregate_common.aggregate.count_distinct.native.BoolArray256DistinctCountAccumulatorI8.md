# `datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulatorI8`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.aggregate.count_distinct.native.BoolArray256DistinctCountAccumulatorI8.json).

<a id="op-ab603e197af192b749c4bbe4"></a>
## BoolArray256DistinctCountAccumulatorI8

`struct` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulatorI8` · datafusion-functions-aggregate-common 55.1.0

```rust
struct BoolArray256DistinctCountAccumulatorI8
```

Source: `src/aggregate/count_distinct/native.rs:260`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Optimized COUNT DISTINCT accumulator for i8 using a bool array.
Uses 256 bytes to track all possible i8 values (mapped to 0..255).

<a id="op-24b032439748ca7614823f07"></a>
## default

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulatorI8::default` · datafusion-functions-aggregate-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulatorI8", "path": "BoolArray256DistinctCountAccumulatorI8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [275, 1], "end": [279, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/aggregate/count_distinct/native.rs:276`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dbaa83a6769722f5ae0b7530"></a>
## evaluate

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulatorI8::evaluate` · datafusion-functions-aggregate-common 55.1.0

```rust
fn evaluate(&mut self) -> datafusion_common::Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulatorI8", "path": "BoolArray256DistinctCountAccumulatorI8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [281, 1], "end": [339, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/native.rs:332`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c22d0f4e53f3b2fb88efd980"></a>
## fmt

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulatorI8::fmt` · datafusion-functions-aggregate-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulatorI8", "path": "BoolArray256DistinctCountAccumulatorI8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [259, 10], "end": [259, 15], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aggregate/count_distinct/native.rs:259`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9742a7650e971d1da7d073c2"></a>
## merge_batch

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulatorI8::merge_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulatorI8", "path": "BoolArray256DistinctCountAccumulatorI8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [281, 1], "end": [339, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/native.rs:295`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8178e5975b1e9930b682e8d"></a>
## new

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulatorI8::new` · datafusion-functions-aggregate-common 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulatorI8", "path": "BoolArray256DistinctCountAccumulatorI8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [264, 1], "end": [273, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate/count_distinct/native.rs:265`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23371099e1574c4855391278"></a>
## size

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulatorI8::size` · datafusion-functions-aggregate-common 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulatorI8", "path": "BoolArray256DistinctCountAccumulatorI8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [281, 1], "end": [339, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/native.rs:336`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0f27e88de33b661863998d48"></a>
## state

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulatorI8::state` · datafusion-functions-aggregate-common 55.1.0

```rust
fn state(&mut self) -> datafusion_common::Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulatorI8", "path": "BoolArray256DistinctCountAccumulatorI8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [281, 1], "end": [339, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/native.rs:312`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-98cf6467dfeca6e39d742856"></a>
## update_batch

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulatorI8::update_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef]) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulatorI8", "path": "BoolArray256DistinctCountAccumulatorI8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [281, 1], "end": [339, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/native.rs:283`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
