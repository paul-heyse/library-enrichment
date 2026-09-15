# `datafusion_physical_plan::aggregates::order::partial`

Crate `datafusion-physical-plan` · 1 public items · structured records in [`model/datafusion_physical_plan.aggregates.order.partial.json`](../model/datafusion_physical_plan.aggregates.order.partial.json)

## GroupOrderingPartial

`struct` · `datafusion_physical_plan::aggregates::order::partial::GroupOrderingPartial`

Also reachable as `datafusion_physical_plan::aggregates::order::GroupOrderingPartial`

```rust
struct GroupOrderingPartial
```

**Derives**: Debug

**Methods** (6)

```rust
fn emit_to(&self) -> Option<EmitTo>
fn input_done(&mut self)
fn new_groups(&mut self, batch_group_values: &[ArrayRef], group_indices: &[usize], total_num_groups: usize) -> Result<()>
fn remove_groups(&mut self, n: usize)
fn reset(&mut self)
fn try_new(order_indices: Vec<usize>) -> Result<Self>
```

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

---
