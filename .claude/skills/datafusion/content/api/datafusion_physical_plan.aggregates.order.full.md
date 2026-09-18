# `datafusion_physical_plan::aggregates::order::full`

Crate `datafusion-physical-plan` · 1 public items · structured records in [`model/datafusion_physical_plan.aggregates.order.full.json`](../model/datafusion_physical_plan.aggregates.order.full.json)

## GroupOrderingFull

`struct` · `datafusion_physical_plan::aggregates::order::full::GroupOrderingFull`

Also reachable as `datafusion_physical_plan::aggregates::order::GroupOrderingFull`

```rust
struct GroupOrderingFull
```

**Derives**: Debug, Default

**Methods** (6)

```rust
fn emit_to(&self) -> Option<EmitTo>
fn input_done(&mut self)
fn new() -> Self
fn new_groups(&mut self, total_num_groups: usize)
fn remove_groups(&mut self, n: usize)
fn reset(&mut self)
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.aggregates.order.full.GroupOrderingFull.md).


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

---
