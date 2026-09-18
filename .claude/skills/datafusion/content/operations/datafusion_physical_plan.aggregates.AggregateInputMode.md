# `datafusion_physical_plan::aggregates::AggregateInputMode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.aggregates.AggregateInputMode.json).

<a id="op-f7dada427bf42eb004cdb459"></a>
## AggregateInputMode

`enum` · `datafusion_physical_plan::aggregates::AggregateInputMode` · datafusion-physical-plan 55.1.0

```rust
enum AggregateInputMode
```

Source: `src/aggregates/mod.rs:243`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Whether an aggregate stage consumes raw input data or intermediate
accumulator state from a previous aggregation stage.

See the [table on `AggregateMode`](AggregateMode#variants-and-their-inputoutput-modes)
for how this relates to aggregate modes.

<a id="op-783d6990b6659be13d4eaca6"></a>
## Partial

`variant` · `datafusion_physical_plan::aggregates::AggregateInputMode::Partial` · datafusion-physical-plan 55.1.0

```rust
Partial
```

Source: `src/aggregates/mod.rs:249`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The stage consumes intermediate accumulator state from a previous
aggregation stage and calls [`Accumulator::merge_batch`](../operations/datafusion_expr_common.accumulator.Accumulator.md#op-f67447a9bb1b06cf9ea3db0d).

<a id="op-57e312b797102a8a49e297e4"></a>
## Raw

`variant` · `datafusion_physical_plan::aggregates::AggregateInputMode::Raw` · datafusion-physical-plan 55.1.0

```rust
Raw
```

Source: `src/aggregates/mod.rs:246`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The stage consumes raw, unaggregated input data and calls
[`Accumulator::update_batch`](../operations/datafusion_expr_common.accumulator.Accumulator.md#op-1afa6e9a06d15906b5bac2ac).

<a id="op-6fab9697f292421b0ea4432e"></a>
## clone

`function` · `datafusion_physical_plan::aggregates::AggregateInputMode::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> AggregateInputMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateInputMode", "path": "AggregateInputMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 23], "end": [242, 28], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/aggregates/mod.rs:242`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-90f1685fc836eda18b7ff186"></a>
## eq

`function` · `datafusion_physical_plan::aggregates::AggregateInputMode::eq` · datafusion-physical-plan 55.1.0

```rust
fn eq(&self, other: &AggregateInputMode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateInputMode", "path": "AggregateInputMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 30], "end": [242, 39], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/aggregates/mod.rs:242`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-caa6130e3f0b9f43a70306c0"></a>
## fmt

`function` · `datafusion_physical_plan::aggregates::AggregateInputMode::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateInputMode", "path": "AggregateInputMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 10], "end": [242, 15], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aggregates/mod.rs:242`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-506c1e69a5ebb8f1fd95874b"></a>
## hash

`function` · `datafusion_physical_plan::aggregates::AggregateInputMode::hash` · datafusion-physical-plan 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateInputMode", "path": "AggregateInputMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 45], "end": [242, 49], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/aggregates/mod.rs:242`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
