# `datafusion_functions_aggregate::min_max::SlidingMinAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.min_max.SlidingMinAccumulator.json).

<a id="op-71a944c8479b8764ccf6c7ba"></a>
## SlidingMinAccumulator

`struct` · `datafusion_functions_aggregate::min_max::SlidingMinAccumulator` · datafusion-functions-aggregate 55.1.0

```rust
struct SlidingMinAccumulator
```

Source: `src/min_max.rs:678`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a25c3883cb6b91b9f88b1124"></a>
## evaluate

`function` · `datafusion_functions_aggregate::min_max::SlidingMinAccumulator::evaluate` · datafusion-functions-aggregate 55.1.0

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::SlidingMinAccumulator", "path": "SlidingMinAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [700, 1], "end": [744, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/min_max.rs:731`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51c90b89e9f913dee26f1d25"></a>
## fmt

`function` · `datafusion_functions_aggregate::min_max::SlidingMinAccumulator::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::SlidingMinAccumulator", "path": "SlidingMinAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [677, 10], "end": [677, 15], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/min_max.rs:677`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63e368cd36cf83ee985457ca"></a>
## merge_batch

`function` · `datafusion_functions_aggregate::min_max::SlidingMinAccumulator::merge_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::SlidingMinAccumulator", "path": "SlidingMinAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [700, 1], "end": [744, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/min_max.rs:727`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0cc376c9ac879596a70ad11d"></a>
## retract_batch

`function` · `datafusion_functions_aggregate::min_max::SlidingMinAccumulator::retract_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn retract_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::SlidingMinAccumulator", "path": "SlidingMinAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [700, 1], "end": [744, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/min_max.rs:715`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6d7eb233f633d3385147ac3"></a>
## size

`function` · `datafusion_functions_aggregate::min_max::SlidingMinAccumulator::size` · datafusion-functions-aggregate 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::SlidingMinAccumulator", "path": "SlidingMinAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [700, 1], "end": [744, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/min_max.rs:739`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5de4c13e2ab2f5b15711bb12"></a>
## state

`function` · `datafusion_functions_aggregate::min_max::SlidingMinAccumulator::state` · datafusion-functions-aggregate 55.1.0

```rust
fn state(&mut self) -> Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::SlidingMinAccumulator", "path": "SlidingMinAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [700, 1], "end": [744, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/min_max.rs:701`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7d25ccf649b2eec5b34e835"></a>
## supports_retract_batch

`function` · `datafusion_functions_aggregate::min_max::SlidingMinAccumulator::supports_retract_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn supports_retract_batch(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::SlidingMinAccumulator", "path": "SlidingMinAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [700, 1], "end": [744, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/min_max.rs:735`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f4d7deb71dd7d82a3f06930"></a>
## try_new

`function` · `datafusion_functions_aggregate::min_max::SlidingMinAccumulator::try_new` · datafusion-functions-aggregate 55.1.0

```rust
fn try_new(datatype: &DataType) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::SlidingMinAccumulator", "path": "SlidingMinAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [684, 1], "end": [698, 2], "filename": "src/min_max.rs"}, "trait": null, "trait_path": null}`

Source: `src/min_max.rs:685`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4301c4ff5106fa48b837d6e5"></a>
## update_batch

`function` · `datafusion_functions_aggregate::min_max::SlidingMinAccumulator::update_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::SlidingMinAccumulator", "path": "SlidingMinAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [700, 1], "end": [744, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/min_max.rs:705`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
