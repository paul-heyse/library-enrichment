# `datafusion_physical_plan::aggregates::group_values::single_group_by::primitive`

Crate `datafusion-physical-plan` · 1 public items · structured records in [`model/datafusion_physical_plan.aggregates.group_values.single_group_by.primitive.json`](../model/datafusion_physical_plan.aggregates.group_values.single_group_by.primitive.json)

## HashValue

`trait` · `datafusion_physical_plan::aggregates::group_values::single_group_by::primitive::HashValue`

```rust
trait HashValue
```

**Methods** (2)

```rust
fn canonicalize(self) -> Self where Self: Sized
fn hash(&self, state: &RandomState) -> u64
```

A trait to allow hashing of floating point numbers

---
