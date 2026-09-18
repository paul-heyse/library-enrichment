# `datafusion_functions_aggregate_common::min_max::MaxAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.min_max.MaxAccumulator.json).

<a id="op-f117624217d8c0c6e9657855"></a>
## MaxAccumulator

`struct` · `datafusion_functions_aggregate_common::min_max::MaxAccumulator` · datafusion-functions-aggregate-common 55.1.0

```rust
struct MaxAccumulator
```

Source: `src/min_max.rs:395`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

An accumulator to compute the maximum value

<a id="op-c43bf58bbe92b13fde398ca9"></a>
## clone

`function` · `datafusion_functions_aggregate_common::min_max::MaxAccumulator::clone` · datafusion-functions-aggregate-common 55.1.0

```rust
fn clone(&self) -> MaxAccumulator
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::min_max::MaxAccumulator", "path": "MaxAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [394, 17], "end": [394, 22], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/min_max.rs:394`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a6d4156d05459c1f27f6c452"></a>
## evaluate

`function` · `datafusion_functions_aggregate_common::min_max::MaxAccumulator::evaluate` · datafusion-functions-aggregate-common 55.1.0

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::min_max::MaxAccumulator", "path": "MaxAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [408, 1], "end": [432, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/min_max.rs:425`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-579ebc3379607e0f464ed881"></a>
## fmt

`function` · `datafusion_functions_aggregate_common::min_max::MaxAccumulator::fmt` · datafusion-functions-aggregate-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::min_max::MaxAccumulator", "path": "MaxAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [394, 10], "end": [394, 15], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/min_max.rs:394`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e084c1207a114b9fa1952476"></a>
## merge_batch

`function` · `datafusion_functions_aggregate_common::min_max::MaxAccumulator::merge_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::min_max::MaxAccumulator", "path": "MaxAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [408, 1], "end": [432, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/min_max.rs:418`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04a7e531db380811bca9792a"></a>
## size

`function` · `datafusion_functions_aggregate_common::min_max::MaxAccumulator::size` · datafusion-functions-aggregate-common 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::min_max::MaxAccumulator", "path": "MaxAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [408, 1], "end": [432, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/min_max.rs:429`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49cf552b3993556587973da0"></a>
## state

`function` · `datafusion_functions_aggregate_common::min_max::MaxAccumulator::state` · datafusion-functions-aggregate-common 55.1.0

```rust
fn state(&mut self) -> Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::min_max::MaxAccumulator", "path": "MaxAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [408, 1], "end": [432, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/min_max.rs:422`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4c19c7b8fe84b009a59d736"></a>
## try_new

`function` · `datafusion_functions_aggregate_common::min_max::MaxAccumulator::try_new` · datafusion-functions-aggregate-common 55.1.0

```rust
fn try_new(datatype: &DataType) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::min_max::MaxAccumulator", "path": "MaxAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [399, 1], "end": [406, 2], "filename": "src/min_max.rs"}, "trait": null, "trait_path": null}`

Source: `src/min_max.rs:401`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

new max accumulator

<a id="op-0770d5bb4bf5f2b74974f3e4"></a>
## update_batch

`function` · `datafusion_functions_aggregate_common::min_max::MaxAccumulator::update_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::min_max::MaxAccumulator", "path": "MaxAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [408, 1], "end": [432, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/min_max.rs:409`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
