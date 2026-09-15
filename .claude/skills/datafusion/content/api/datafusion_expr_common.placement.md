# `datafusion_expr_common::placement`

Crate `datafusion-expr-common` · 1 public items · structured records in [`model/datafusion_expr_common.placement.json`](../model/datafusion_expr_common.placement.json)

## ExpressionPlacement

`enum` · `datafusion_expr_common::placement::ExpressionPlacement`

Also reachable as `datafusion::logical_expr::ExpressionPlacement`, `datafusion::logical_expr_common::ExpressionPlacement`, `datafusion_expr::ExpressionPlacement`, `datafusion_expr_common::ExpressionPlacement`

```rust
enum ExpressionPlacement
```

**Variants**: `Literal`, `Column`, `MoveTowardsLeafNodes`, `KeepInPlace`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn should_push_to_leaves(&self) -> bool
```

Describes where an expression should be placed in the query plan for
optimal execution. This is used by optimizers to make decisions about
expression placement, such as whether to push expressions down through
projections.

---
