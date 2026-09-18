# `datafusion_functions_aggregate_common::aggregate::count_distinct::native::Bitmap65536DistinctCountAccumulatorI16`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.aggregate.count_distinct.native.Bitmap65536DistinctCountAccumulatorI16.json).

<a id="op-2fa269dffc39f22101326c8e"></a>
## Bitmap65536DistinctCountAccumulatorI16

`struct` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::Bitmap65536DistinctCountAccumulatorI16` · datafusion-functions-aggregate-common 55.1.0

```rust
struct Bitmap65536DistinctCountAccumulatorI16
```

Source: `src/aggregate/count_distinct/native.rs:437`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Optimized COUNT DISTINCT accumulator for i16 using a 65536-bit bitmap.
Uses 8KB (1024 x u64) to track all possible i16 values (mapped to 0..65535).

<a id="op-2947d4adb75406020478274a"></a>
## default

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::Bitmap65536DistinctCountAccumulatorI16::default` · datafusion-functions-aggregate-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::Bitmap65536DistinctCountAccumulatorI16", "path": "Bitmap65536DistinctCountAccumulatorI16"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [462, 1], "end": [466, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/aggregate/count_distinct/native.rs:463`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3d96ddaade3eff920d7b74e"></a>
## evaluate

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::Bitmap65536DistinctCountAccumulatorI16::evaluate` · datafusion-functions-aggregate-common 55.1.0

```rust
fn evaluate(&mut self) -> datafusion_common::Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::Bitmap65536DistinctCountAccumulatorI16", "path": "Bitmap65536DistinctCountAccumulatorI16"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [468, 1], "end": [526, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/native.rs:519`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d986b50fad2f0a5d768f7786"></a>
## fmt

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::Bitmap65536DistinctCountAccumulatorI16::fmt` · datafusion-functions-aggregate-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::Bitmap65536DistinctCountAccumulatorI16", "path": "Bitmap65536DistinctCountAccumulatorI16"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [436, 10], "end": [436, 15], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aggregate/count_distinct/native.rs:436`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-956ef90ebfa9129ec2c44275"></a>
## merge_batch

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::Bitmap65536DistinctCountAccumulatorI16::merge_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::Bitmap65536DistinctCountAccumulatorI16", "path": "Bitmap65536DistinctCountAccumulatorI16"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [468, 1], "end": [526, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/native.rs:482`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9152449b0ce68c974f8c8a2"></a>
## new

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::Bitmap65536DistinctCountAccumulatorI16::new` · datafusion-functions-aggregate-common 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::Bitmap65536DistinctCountAccumulatorI16", "path": "Bitmap65536DistinctCountAccumulatorI16"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [441, 1], "end": [460, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate/count_distinct/native.rs:442`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-875f1c604a403c541a23ed76"></a>
## size

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::Bitmap65536DistinctCountAccumulatorI16::size` · datafusion-functions-aggregate-common 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::Bitmap65536DistinctCountAccumulatorI16", "path": "Bitmap65536DistinctCountAccumulatorI16"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [468, 1], "end": [526, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/native.rs:523`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e350fefa64f23919d5289725"></a>
## state

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::Bitmap65536DistinctCountAccumulatorI16::state` · datafusion-functions-aggregate-common 55.1.0

```rust
fn state(&mut self) -> datafusion_common::Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::Bitmap65536DistinctCountAccumulatorI16", "path": "Bitmap65536DistinctCountAccumulatorI16"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [468, 1], "end": [526, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/native.rs:499`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e195098d7e4911def821da6"></a>
## update_batch

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::Bitmap65536DistinctCountAccumulatorI16::update_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef]) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::Bitmap65536DistinctCountAccumulatorI16", "path": "Bitmap65536DistinctCountAccumulatorI16"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [468, 1], "end": [526, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/native.rs:470`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
