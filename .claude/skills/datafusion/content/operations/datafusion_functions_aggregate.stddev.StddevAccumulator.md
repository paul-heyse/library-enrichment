# `datafusion_functions_aggregate::stddev::StddevAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.stddev.StddevAccumulator.json).

<a id="op-5c01620c8a6e3ab54a66d255"></a>
## StddevAccumulator

`struct` · `datafusion_functions_aggregate::stddev::StddevAccumulator` · datafusion-functions-aggregate 55.1.0

```rust
struct StddevAccumulator
```

Source: `src/stddev.rs:247`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

An accumulator to compute the average

<a id="op-702d86cbf73266add8470e40"></a>
## evaluate

`function` · `datafusion_functions_aggregate::stddev::StddevAccumulator::evaluate` · datafusion-functions-aggregate 55.1.0

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::StddevAccumulator", "path": "StddevAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [264, 1], "end": [301, 2], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/stddev.rs:285`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e2f54715cd95606d5cb6bac"></a>
## fmt

`function` · `datafusion_functions_aggregate::stddev::StddevAccumulator::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::StddevAccumulator", "path": "StddevAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [246, 10], "end": [246, 15], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/stddev.rs:246`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9819238b5aafc93c981e949c"></a>
## get_m2

`function` · `datafusion_functions_aggregate::stddev::StddevAccumulator::get_m2` · datafusion-functions-aggregate 55.1.0

```rust
fn get_m2(&self) -> f64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::StddevAccumulator", "path": "StddevAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [262, 2], "filename": "src/stddev.rs"}, "trait": null, "trait_path": null}`

Source: `src/stddev.rs:259`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9da5cbb341a30269c93ade7"></a>
## merge_batch

`function` · `datafusion_functions_aggregate::stddev::StddevAccumulator::merge_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::StddevAccumulator", "path": "StddevAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [264, 1], "end": [301, 2], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/stddev.rs:281`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3bc948935352e591f6de63b4"></a>
## retract_batch

`function` · `datafusion_functions_aggregate::stddev::StddevAccumulator::retract_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn retract_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::StddevAccumulator", "path": "StddevAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [264, 1], "end": [301, 2], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/stddev.rs:277`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7fe645db06fc0ca9633c9dda"></a>
## size

`function` · `datafusion_functions_aggregate::stddev::StddevAccumulator::size` · datafusion-functions-aggregate 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::StddevAccumulator", "path": "StddevAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [264, 1], "end": [301, 2], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/stddev.rs:294`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-558f024fcb5244f766636f08"></a>
## state

`function` · `datafusion_functions_aggregate::stddev::StddevAccumulator::state` · datafusion-functions-aggregate 55.1.0

```rust
fn state(&mut self) -> Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::StddevAccumulator", "path": "StddevAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [264, 1], "end": [301, 2], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/stddev.rs:265`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27eb28c974b3125c08aa1d91"></a>
## supports_retract_batch

`function` · `datafusion_functions_aggregate::stddev::StddevAccumulator::supports_retract_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn supports_retract_batch(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::StddevAccumulator", "path": "StddevAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [264, 1], "end": [301, 2], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/stddev.rs:298`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-400a292a4e8abb20002e1e34"></a>
## try_new

`function` · `datafusion_functions_aggregate::stddev::StddevAccumulator::try_new` · datafusion-functions-aggregate 55.1.0

```rust
fn try_new(s_type: StatsType) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::StddevAccumulator", "path": "StddevAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [262, 2], "filename": "src/stddev.rs"}, "trait": null, "trait_path": null}`

Source: `src/stddev.rs:253`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Creates a new `StddevAccumulator`

<a id="op-294cff71d6990e3ba078187b"></a>
## update_batch

`function` · `datafusion_functions_aggregate::stddev::StddevAccumulator::update_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::StddevAccumulator", "path": "StddevAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [264, 1], "end": [301, 2], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/stddev.rs:273`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
