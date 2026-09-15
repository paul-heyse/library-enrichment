# `datafusion_physical_plan::aggregates::order`

Crate `datafusion-physical-plan` · 1 public items · structured records in [`model/datafusion_physical_plan.aggregates.order.json`](../model/datafusion_physical_plan.aggregates.order.json)

## GroupOrdering

`enum` · `datafusion_physical_plan::aggregates::order::GroupOrdering`

```rust
enum GroupOrdering
```

**Variants**: `None`, `Partial`, `Full`

**Derives**: Debug

**Methods** (8)

```rust
fn emit_to(&self) -> Option<EmitTo>
fn input_done(&mut self)
fn new_groups(&mut self, batch_group_values: &[ArrayRef], group_indices: &[usize], total_num_groups: usize) -> Result<()>
fn oom_emit_to(&self, n: usize) -> Option<EmitTo>
fn remove_groups(&mut self, n: usize)
fn reset(&mut self)
fn size(&self) -> usize
fn try_new(mode: &InputOrderMode) -> Result<Self>
```

Ordering information for each group in the hash table

---
