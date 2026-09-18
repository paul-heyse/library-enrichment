# `datafusion_physical_plan::aggregates::AggregateOutputMode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.aggregates.AggregateOutputMode.json).

<a id="op-6147b1a705ac6e29745affac"></a>
## AggregateOutputMode

`enum` · `datafusion_physical_plan::aggregates::AggregateOutputMode` · datafusion-physical-plan 55.1.0

```rust
enum AggregateOutputMode
```

Source: `src/aggregates/mod.rs:258`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Whether an aggregate stage produces intermediate accumulator state
or final output values.

See the [table on `AggregateMode`](AggregateMode#variants-and-their-inputoutput-modes)
for how this relates to aggregate modes.

<a id="op-4e3781bbd0d983d6d548ce6a"></a>
## Final

`variant` · `datafusion_physical_plan::aggregates::AggregateOutputMode::Final` · datafusion-physical-plan 55.1.0

```rust
Final
```

Source: `src/aggregates/mod.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The stage produces final output values via
[`Accumulator::evaluate`](../operations/datafusion_expr_common.accumulator.Accumulator.md#op-d45f0e13fa55e22a4afa980e).

<a id="op-683af6ecfb92c7c82275db8e"></a>
## Partial

`variant` · `datafusion_physical_plan::aggregates::AggregateOutputMode::Partial` · datafusion-physical-plan 55.1.0

```rust
Partial
```

Source: `src/aggregates/mod.rs:261`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The stage produces intermediate accumulator state, serialized via
[`Accumulator::state`](../operations/datafusion_expr_common.accumulator.Accumulator.md#op-e13afcecd1da09f0b485e6dc).

<a id="op-dbfc240bf835e3ab8757bca5"></a>
## clone

`function` · `datafusion_physical_plan::aggregates::AggregateOutputMode::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> AggregateOutputMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateOutputMode", "path": "AggregateOutputMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [257, 23], "end": [257, 28], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/aggregates/mod.rs:257`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1265e0d0bcee839f01b13b13"></a>
## eq

`function` · `datafusion_physical_plan::aggregates::AggregateOutputMode::eq` · datafusion-physical-plan 55.1.0

```rust
fn eq(&self, other: &AggregateOutputMode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateOutputMode", "path": "AggregateOutputMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [257, 30], "end": [257, 39], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/aggregates/mod.rs:257`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85deaae94a3aba83f919c1f7"></a>
## fmt

`function` · `datafusion_physical_plan::aggregates::AggregateOutputMode::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateOutputMode", "path": "AggregateOutputMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [257, 10], "end": [257, 15], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aggregates/mod.rs:257`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a0e6952d4e069397c7af3f9"></a>
## hash

`function` · `datafusion_physical_plan::aggregates::AggregateOutputMode::hash` · datafusion-physical-plan 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::AggregateOutputMode", "path": "AggregateOutputMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [257, 45], "end": [257, 49], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/aggregates/mod.rs:257`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
