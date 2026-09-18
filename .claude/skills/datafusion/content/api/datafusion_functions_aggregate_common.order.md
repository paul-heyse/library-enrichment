# `datafusion_functions_aggregate_common::order`

Crate `datafusion-functions-aggregate-common` · 1 public items · structured records in [`model/datafusion_functions_aggregate_common.order.json`](../model/datafusion_functions_aggregate_common.order.json)

## AggregateOrderSensitivity

`enum` · `datafusion_functions_aggregate_common::order::AggregateOrderSensitivity`

Also reachable as `datafusion_expr::utils::AggregateOrderSensitivity`

```rust
enum AggregateOrderSensitivity
```

**Variants**: `Insensitive`, `HardRequirement`, `SoftRequirement`, `Beneficial`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn hard_requires(&self) -> bool
fn is_beneficial(&self) -> bool
fn is_insensitive(&self) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.order.AggregateOrderSensitivity.md).


Represents the sensitivity of an aggregate expression to ordering.

---
