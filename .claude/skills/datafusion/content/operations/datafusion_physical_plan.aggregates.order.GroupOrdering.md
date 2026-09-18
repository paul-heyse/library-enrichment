# `datafusion_physical_plan::aggregates::order::GroupOrdering`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.aggregates.order.GroupOrdering.json).

<a id="op-cd5ed13ddf4351feaa849fc7"></a>
## GroupOrdering

`enum` · `datafusion_physical_plan::aggregates::order::GroupOrdering` · datafusion-physical-plan 55.1.0

```rust
enum GroupOrdering
```

Source: `src/aggregates/order/mod.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Ordering information for each group in the hash table

<a id="op-3ac0e0167087343a4641076d"></a>
## Full

`variant` · `datafusion_physical_plan::aggregates::order::GroupOrdering::Full` · datafusion-physical-plan 55.1.0

```rust
Full
```

Source: `src/aggregates/order/mod.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Groups are entirely contiguous,

<a id="op-276eaab503333ec7f13380da"></a>
## None

`variant` · `datafusion_physical_plan::aggregates::order::GroupOrdering::None` · datafusion-physical-plan 55.1.0

```rust
None
```

Source: `src/aggregates/order/mod.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Groups are not ordered

<a id="op-89da4153eccfbeae86b70d65"></a>
## Partial

`variant` · `datafusion_physical_plan::aggregates::order::GroupOrdering::Partial` · datafusion-physical-plan 55.1.0

```rust
Partial
```

Source: `src/aggregates/order/mod.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Groups are ordered by some pre-set of the group keys

<a id="op-35ce66679b92d67b0ef033dd"></a>
## emit_to

`function` · `datafusion_physical_plan::aggregates::order::GroupOrdering::emit_to` · datafusion-physical-plan 55.1.0

```rust
fn emit_to(&self) -> Option<EmitTo>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::order::GroupOrdering", "path": "GroupOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [159, 2], "filename": "src/aggregates/order/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/order/mod.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns how many groups can be emitted while respecting the current
ordering guarantees, or `None` if no data can be emitted.

<a id="op-283eec337b28a79b2c4b9d33"></a>
## fmt

`function` · `datafusion_physical_plan::aggregates::order::GroupOrdering::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::order::GroupOrdering", "path": "GroupOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 10], "end": [32, 15], "filename": "src/aggregates/order/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aggregates/order/mod.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bdd66af1dd9c125f87a1ff28"></a>
## input_done

`function` · `datafusion_physical_plan::aggregates::order::GroupOrdering::input_done` · datafusion-physical-plan 55.1.0

```rust
fn input_done(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::order::GroupOrdering", "path": "GroupOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [159, 2], "filename": "src/aggregates/order/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/order/mod.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Updates the state to indicate that the input is complete.

<a id="op-a4917d87812169d2f40a4322"></a>
## new_groups

`function` · `datafusion_physical_plan::aggregates::order::GroupOrdering::new_groups` · datafusion-physical-plan 55.1.0

```rust
fn new_groups(&mut self, batch_group_values: &[ArrayRef], group_indices: &[usize], total_num_groups: usize) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::order::GroupOrdering", "path": "GroupOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [159, 2], "filename": "src/aggregates/order/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/order/mod.rs:128`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Called when new groups are added in a batch.

* `batch_group_values`: group key values for each row in the batch

* `group_indices`: indices for each row in the batch

* `total_num_groups`: total number of groups (so max
  group_index is total_num_groups - 1).

<a id="op-20eb4588fbca3926226c2237"></a>
## oom_emit_to

`function` · `datafusion_physical_plan::aggregates::order::GroupOrdering::oom_emit_to` · datafusion-physical-plan 55.1.0

```rust
fn oom_emit_to(&self, n: usize) -> Option<EmitTo>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::order::GroupOrdering", "path": "GroupOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [159, 2], "filename": "src/aggregates/order/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/order/mod.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the emit strategy to use under memory pressure (OOM).

Returns the strategy that must be used when emitting up to `n` groups
while respecting the current ordering guarantees.

Returns `None` if no data can be emitted.

<a id="op-4eeee382d9be4112493ef393"></a>
## remove_groups

`function` · `datafusion_physical_plan::aggregates::order::GroupOrdering::remove_groups` · datafusion-physical-plan 55.1.0

```rust
fn remove_groups(&mut self, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::order::GroupOrdering", "path": "GroupOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [159, 2], "filename": "src/aggregates/order/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/order/mod.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Removes the first `n` groups from the internal state, shifting all
existing indexes down by `n`.

<a id="op-cad118a1ede1c820edef1929"></a>
## reset

`function` · `datafusion_physical_plan::aggregates::order::GroupOrdering::reset` · datafusion-physical-plan 55.1.0

```rust
fn reset(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::order::GroupOrdering", "path": "GroupOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [159, 2], "filename": "src/aggregates/order/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/order/mod.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Resets the ordering state while preserving the configured ordering mode.

Ordered partial aggregation uses this after passing intermediate states
downstream, and ordered final aggregation uses it after spilling a run.
In both cases the hash table is empty and can start tracking the next
input batch from a fresh ordering state.

<a id="op-b17fb8175125d5b8571a47ee"></a>
## size

`function` · `datafusion_physical_plan::aggregates::order::GroupOrdering::size` · datafusion-physical-plan 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::order::GroupOrdering", "path": "GroupOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [159, 2], "filename": "src/aggregates/order/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/order/mod.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the size of memory used by the ordering state, in bytes.

<a id="op-a28d86ed9d1d1a0ea63d1100"></a>
## try_new

`function` · `datafusion_physical_plan::aggregates::order::GroupOrdering::try_new` · datafusion-physical-plan 55.1.0

```rust
fn try_new(mode: &InputOrderMode) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::order::GroupOrdering", "path": "GroupOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [159, 2], "filename": "src/aggregates/order/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/order/mod.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a `GroupOrdering` for the specified ordering
