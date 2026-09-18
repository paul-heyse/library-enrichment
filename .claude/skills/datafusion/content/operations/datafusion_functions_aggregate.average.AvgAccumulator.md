# `datafusion_functions_aggregate::average::AvgAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.average.AvgAccumulator.json).

<a id="op-fc383a25ee3e117dffc34aaa"></a>
## AvgAccumulator

`struct` · `datafusion_functions_aggregate::average::AvgAccumulator` · datafusion-functions-aggregate 55.1.0

```rust
struct AvgAccumulator
```

Source: `src/average.rs:578`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

An accumulator to compute the average

<a id="op-5c96667fd4dbf35eb705b118"></a>
## default

`function` · `datafusion_functions_aggregate::average::AvgAccumulator::default` · datafusion-functions-aggregate 55.1.0

```rust
fn default() -> AvgAccumulator
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::average::AvgAccumulator", "path": "AvgAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [577, 17], "end": [577, 24], "filename": "src/average.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/average.rs:577`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5087df26ec9c306d76c6889"></a>
## evaluate

`function` · `datafusion_functions_aggregate::average::AvgAccumulator::evaluate` · datafusion-functions-aggregate 55.1.0

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::average::AvgAccumulator", "path": "AvgAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [583, 1], "end": [641, 2], "filename": "src/average.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/average.rs:594`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b23d8e61c813f9e547f270bc"></a>
## fmt

`function` · `datafusion_functions_aggregate::average::AvgAccumulator::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::average::AvgAccumulator", "path": "AvgAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [577, 10], "end": [577, 15], "filename": "src/average.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/average.rs:577`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2463071e147fa18d2542c7d"></a>
## merge_batch

`function` · `datafusion_functions_aggregate::average::AvgAccumulator::merge_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::average::AvgAccumulator", "path": "AvgAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [583, 1], "end": [641, 2], "filename": "src/average.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/average.rs:618`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3efb87b987340d4a04c78e7c"></a>
## retract_batch

`function` · `datafusion_functions_aggregate::average::AvgAccumulator::retract_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn retract_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::average::AvgAccumulator", "path": "AvgAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [583, 1], "end": [641, 2], "filename": "src/average.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/average.rs:629`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1deaf3186fecaa1b9e87bb78"></a>
## size

`function` · `datafusion_functions_aggregate::average::AvgAccumulator::size` · datafusion-functions-aggregate 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::average::AvgAccumulator", "path": "AvgAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [583, 1], "end": [641, 2], "filename": "src/average.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/average.rs:607`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2406955d957643545b3697ad"></a>
## state

`function` · `datafusion_functions_aggregate::average::AvgAccumulator::state` · datafusion-functions-aggregate 55.1.0

```rust
fn state(&mut self) -> Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::average::AvgAccumulator", "path": "AvgAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [583, 1], "end": [641, 2], "filename": "src/average.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/average.rs:611`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7f390f2628c453f9c87582f"></a>
## supports_retract_batch

`function` · `datafusion_functions_aggregate::average::AvgAccumulator::supports_retract_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn supports_retract_batch(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::average::AvgAccumulator", "path": "AvgAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [583, 1], "end": [641, 2], "filename": "src/average.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/average.rs:638`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95c4f5c48ae2bb7b2527333a"></a>
## update_batch

`function` · `datafusion_functions_aggregate::average::AvgAccumulator::update_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::average::AvgAccumulator", "path": "AvgAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [583, 1], "end": [641, 2], "filename": "src/average.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/average.rs:584`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
