# `datafusion_physical_plan::aggregates::order::partial::GroupOrderingPartial`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.aggregates.order.partial.GroupOrderingPartial.json).

<a id="op-6177fe62e5dbb68d17d2e2f9"></a>
## GroupOrderingPartial

`struct` · `datafusion_physical_plan::aggregates::order::partial::GroupOrderingPartial` · datafusion-physical-plan 55.1.0

```rust
struct GroupOrderingPartial
```

Source: `src/aggregates/order/partial.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Tracks grouping state when the data is ordered by some subset of
the group keys.

Once the next *sort key* value is seen, never see groups with that
sort key again, so we can emit all groups with the previous sort
key and earlier.

For example, given `SUM(amt) GROUP BY id, state` if the input is
sorted by `state`, when a new value of `state` is seen, all groups
with prior values of `state` can be emitted.

The state is tracked like this:

```text
                                           ┏━━━━━━━━━━━━━━━━━┓ ┏━━━━━━━┓
    ┌─────┐    ┌───────────────────┐ ┌─────┃        9        ┃ ┃ "MD"  ┃
    │┌───┐│    │ ┌──────────────┐  │ │     ┗━━━━━━━━━━━━━━━━━┛ ┗━━━━━━━┛
    ││ 0 ││    │ │  123, "MA"   │  │ │        current_sort      sort_key
    │└───┘│    │ └──────────────┘  │ │
    │ ... │    │    ...            │ │      current_sort tracks the
    │┌───┐│    │ ┌──────────────┐  │ │      smallest group index that had
    ││ 8 ││    │ │  765, "MA"   │  │ │      the same sort_key as current
    │├───┤│    │ ├──────────────┤  │ │
    ││ 9 ││    │ │  923, "MD"   │◀─┼─┘
    │├───┤│    │ ├──────────────┤  │        ┏━━━━━━━━━━━━━━┓
    ││10 ││    │ │  345, "MD"   │  │  ┌─────┃      11      ┃
    │├───┤│    │ ├──────────────┤  │  │     ┗━━━━━━━━━━━━━━┛
    ││11 ││    │ │  124, "MD"   │◀─┼──┘         current
    │└───┘│    │ └──────────────┘  │
    └─────┘    └───────────────────┘

 group indices
(in group value  group_values               current tracks the most
     order)                                    recent group index
```

<a id="op-62c851c3d95fc38a59624ad8"></a>
## emit_to

`function` · `datafusion_physical_plan::aggregates::order::partial::GroupOrderingPartial::emit_to` · datafusion-physical-plan 55.1.0

```rust
fn emit_to(&self) -> Option<EmitTo>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::order::partial::GroupOrderingPartial", "path": "GroupOrderingPartial"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [274, 2], "filename": "src/aggregates/order/partial.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/order/partial.rs:142`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

How many groups be emitted, or None if no data can be emitted

<a id="op-6d1bff08e64abe8630f257a6"></a>
## fmt

`function` · `datafusion_physical_plan::aggregates::order::partial::GroupOrderingPartial::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::order::partial::GroupOrderingPartial", "path": "GroupOrderingPartial"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 10], "end": [65, 15], "filename": "src/aggregates/order/partial.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aggregates/order/partial.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff09456f0242599b24a1590b"></a>
## input_done

`function` · `datafusion_physical_plan::aggregates::order::partial::GroupOrderingPartial::input_done` · datafusion-physical-plan 55.1.0

```rust
fn input_done(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::order::partial::GroupOrderingPartial", "path": "GroupOrderingPartial"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [274, 2], "filename": "src/aggregates/order/partial.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/order/partial.rs:182`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Note that the input is complete so any outstanding groups are done as well

<a id="op-5bb8a5e2a9c6ef9250e885e1"></a>
## new_groups

`function` · `datafusion_physical_plan::aggregates::order::partial::GroupOrderingPartial::new_groups` · datafusion-physical-plan 55.1.0

```rust
fn new_groups(&mut self, batch_group_values: &[ArrayRef], group_indices: &[usize], total_num_groups: usize) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::order::partial::GroupOrderingPartial", "path": "GroupOrderingPartial"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [274, 2], "filename": "src/aggregates/order/partial.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/order/partial.rs:214`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Called when new groups are added in a batch. See documentation
on [`super::GroupOrdering::new_groups`](../operations/datafusion_physical_plan.aggregates.order.GroupOrdering.md#op-a4917d87812169d2f40a4322)

<a id="op-7a207d7e601e7202e0e0aaef"></a>
## remove_groups

`function` · `datafusion_physical_plan::aggregates::order::partial::GroupOrderingPartial::remove_groups` · datafusion-physical-plan 55.1.0

```rust
fn remove_groups(&mut self, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::order::partial::GroupOrderingPartial", "path": "GroupOrderingPartial"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [274, 2], "filename": "src/aggregates/order/partial.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/order/partial.rs:162`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

remove the first n groups from the internal state, shifting
all existing indexes down by `n`

<a id="op-a8e5015d8631bec2d3c7f61e"></a>
## reset

`function` · `datafusion_physical_plan::aggregates::order::partial::GroupOrderingPartial::reset` · datafusion-physical-plan 55.1.0

```rust
fn reset(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::order::partial::GroupOrderingPartial", "path": "GroupOrderingPartial"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [274, 2], "filename": "src/aggregates/order/partial.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/order/partial.rs:191`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Starts tracking a new ordered input segment with the same sort-key
columns.

<a id="op-b4d1b0bc7a180dc0a2960fd2"></a>
## try_new

`function` · `datafusion_physical_plan::aggregates::order::partial::GroupOrderingPartial::try_new` · datafusion-physical-plan 55.1.0

```rust
fn try_new(order_indices: Vec<usize>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::order::partial::GroupOrderingPartial", "path": "GroupOrderingPartial"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [274, 2], "filename": "src/aggregates/order/partial.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/order/partial.rs:120`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

TODO: Remove unnecessary `input_schema` parameter.
