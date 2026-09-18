# `datafusion_functions_aggregate::min_max::SlidingMaxAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.min_max.SlidingMaxAccumulator.json).

<a id="op-2e4bfe1446cb7979bff38335"></a>
## SlidingMaxAccumulator

`struct` · `datafusion_functions_aggregate::min_max::SlidingMaxAccumulator` · datafusion-functions-aggregate 55.1.0

```rust
struct SlidingMaxAccumulator
```

Source: `src/min_max.rs:383`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5fb077848bb44cfaf1ecff4d"></a>
## evaluate

`function` · `datafusion_functions_aggregate::min_max::SlidingMaxAccumulator::evaluate` · datafusion-functions-aggregate 55.1.0

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::SlidingMaxAccumulator", "path": "SlidingMaxAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [406, 1], "end": [450, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/min_max.rs:437`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-372a5d219767291874c5b06f"></a>
## fmt

`function` · `datafusion_functions_aggregate::min_max::SlidingMaxAccumulator::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::SlidingMaxAccumulator", "path": "SlidingMaxAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [382, 10], "end": [382, 15], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/min_max.rs:382`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-958537cb0459b9d57aa07a72"></a>
## merge_batch

`function` · `datafusion_functions_aggregate::min_max::SlidingMaxAccumulator::merge_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::SlidingMaxAccumulator", "path": "SlidingMaxAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [406, 1], "end": [450, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/min_max.rs:429`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-445bf8813d41596d959fa9be"></a>
## retract_batch

`function` · `datafusion_functions_aggregate::min_max::SlidingMaxAccumulator::retract_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn retract_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::SlidingMaxAccumulator", "path": "SlidingMaxAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [406, 1], "end": [450, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/min_max.rs:417`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c6ae936ac963d5437365c56"></a>
## size

`function` · `datafusion_functions_aggregate::min_max::SlidingMaxAccumulator::size` · datafusion-functions-aggregate 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::SlidingMaxAccumulator", "path": "SlidingMaxAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [406, 1], "end": [450, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/min_max.rs:445`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23a1d675d1960ff841bc9b54"></a>
## state

`function` · `datafusion_functions_aggregate::min_max::SlidingMaxAccumulator::state` · datafusion-functions-aggregate 55.1.0

```rust
fn state(&mut self) -> Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::SlidingMaxAccumulator", "path": "SlidingMaxAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [406, 1], "end": [450, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/min_max.rs:433`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1020ba0c1f8117218083ba5d"></a>
## supports_retract_batch

`function` · `datafusion_functions_aggregate::min_max::SlidingMaxAccumulator::supports_retract_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn supports_retract_batch(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::SlidingMaxAccumulator", "path": "SlidingMaxAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [406, 1], "end": [450, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/min_max.rs:441`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc25665fa19de24564cb1dfe"></a>
## try_new

`function` · `datafusion_functions_aggregate::min_max::SlidingMaxAccumulator::try_new` · datafusion-functions-aggregate 55.1.0

```rust
fn try_new(datatype: &DataType) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::SlidingMaxAccumulator", "path": "SlidingMaxAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [389, 1], "end": [404, 2], "filename": "src/min_max.rs"}, "trait": null, "trait_path": null}`

Source: `src/min_max.rs:391`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

new max accumulator

<a id="op-133051effb42a02ecc5e4663"></a>
## update_batch

`function` · `datafusion_functions_aggregate::min_max::SlidingMaxAccumulator::update_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::SlidingMaxAccumulator", "path": "SlidingMaxAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [406, 1], "end": [450, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/min_max.rs:407`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
