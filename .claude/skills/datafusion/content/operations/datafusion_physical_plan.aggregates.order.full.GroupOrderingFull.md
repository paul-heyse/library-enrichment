# `datafusion_physical_plan::aggregates::order::full::GroupOrderingFull`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.aggregates.order.full.GroupOrderingFull.json).

<a id="op-6f9c0417ba94ce674dcd1cf1"></a>
## GroupOrderingFull

`struct` · `datafusion_physical_plan::aggregates::order::full::GroupOrderingFull` · datafusion-physical-plan 55.1.0

```rust
struct GroupOrderingFull
```

Source: `src/aggregates/order/full.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Tracks grouping state when the data is ordered entirely by its
group keys

When the group values are sorted, as soon as we see group `n+1` we
know we will never see any rows for group `n` again and thus they
can be emitted.

For example, given `SUM(amt) GROUP BY id` if the input is sorted
by `id` as soon as a new `id` value is seen all previous values
can be emitted.

The state is tracked like this:

```text
     ┌─────┐   ┌──────────────────┐
     │┌───┐│   │ ┌──────────────┐ │         ┏━━━━━━━━━━━━━━┓
     ││ 0 ││   │ │     123      │ │   ┌─────┃      13      ┃
     │└───┘│   │ └──────────────┘ │   │     ┗━━━━━━━━━━━━━━┛
     │ ... │   │    ...           │   │
     │┌───┐│   │ ┌──────────────┐ │   │         current
     ││12 ││   │ │     234      │ │   │
     │├───┤│   │ ├──────────────┤ │   │
     ││12 ││   │ │     234      │ │   │
     │├───┤│   │ ├──────────────┤ │   │
     ││13 ││   │ │     456      │◀┼───┘
     │└───┘│   │ └──────────────┘ │
     └─────┘   └──────────────────┘

 group indices    group_values        current tracks the most
(in group value                          recent group index
     order)
```

In this diagram, the current group is `13`, and thus groups
`0..12` can be emitted. Note that `13` can not yet be emitted as
there may be more values in the next batch with the same group_id.

<a id="op-a23928a8282ad67d51612ffa"></a>
## default

`function` · `datafusion_physical_plan::aggregates::order::full::GroupOrderingFull::default` · datafusion-physical-plan 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::order::full::GroupOrderingFull", "path": "GroupOrderingFull"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 1], "end": [156, 2], "filename": "src/aggregates/order/full.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/aggregates/order/full.rs:153`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f06ade416e84fc1e93e6013"></a>
## emit_to

`function` · `datafusion_physical_plan::aggregates::order::full::GroupOrderingFull::emit_to` · datafusion-physical-plan 55.1.0

```rust
fn emit_to(&self) -> Option<EmitTo>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::order::full::GroupOrderingFull", "path": "GroupOrderingFull"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 1], "end": [150, 2], "filename": "src/aggregates/order/full.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/order/full.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb756183946250a7c14b3d21"></a>
## fmt

`function` · `datafusion_physical_plan::aggregates::order::full::GroupOrderingFull::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::order::full::GroupOrderingFull", "path": "GroupOrderingFull"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 10], "end": [57, 15], "filename": "src/aggregates/order/full.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aggregates/order/full.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8fd63e7ae1e7eb70c5446067"></a>
## input_done

`function` · `datafusion_physical_plan::aggregates::order::full::GroupOrderingFull::input_done` · datafusion-physical-plan 55.1.0

```rust
fn input_done(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::order::full::GroupOrderingFull", "path": "GroupOrderingFull"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 1], "end": [150, 2], "filename": "src/aggregates/order/full.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/order/full.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Note that the input is complete so any outstanding groups are done as well

<a id="op-13134672bd9cd519511f5c93"></a>
## new

`function` · `datafusion_physical_plan::aggregates::order::full::GroupOrderingFull::new` · datafusion-physical-plan 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::order::full::GroupOrderingFull", "path": "GroupOrderingFull"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 1], "end": [150, 2], "filename": "src/aggregates/order/full.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/order/full.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-744d2a49728451df9ae16db4"></a>
## new_groups

`function` · `datafusion_physical_plan::aggregates::order::full::GroupOrderingFull::new_groups` · datafusion-physical-plan 55.1.0

```rust
fn new_groups(&mut self, total_num_groups: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::order::full::GroupOrderingFull", "path": "GroupOrderingFull"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 1], "end": [150, 2], "filename": "src/aggregates/order/full.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/order/full.rs:125`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Called when new groups are added in a batch. See documentation
on [`super::GroupOrdering::new_groups`](../operations/datafusion_physical_plan.aggregates.order.GroupOrdering.md#op-a4917d87812169d2f40a4322)

<a id="op-5ef5db4ccb49c35ab4502a82"></a>
## remove_groups

`function` · `datafusion_physical_plan::aggregates::order::full::GroupOrderingFull::remove_groups` · datafusion-physical-plan 55.1.0

```rust
fn remove_groups(&mut self, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::order::full::GroupOrderingFull", "path": "GroupOrderingFull"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 1], "end": [150, 2], "filename": "src/aggregates/order/full.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/order/full.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

remove the first n groups from the internal state, shifting
all existing indexes down by `n`

<a id="op-52b205a820a320b3bac728df"></a>
## reset

`function` · `datafusion_physical_plan::aggregates::order::full::GroupOrderingFull::reset` · datafusion-physical-plan 55.1.0

```rust
fn reset(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::order::full::GroupOrderingFull", "path": "GroupOrderingFull"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 1], "end": [150, 2], "filename": "src/aggregates/order/full.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/order/full.rs:119`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Starts tracking a new fully ordered input segment.
