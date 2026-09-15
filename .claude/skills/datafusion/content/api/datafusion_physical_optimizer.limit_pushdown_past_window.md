# `datafusion_physical_optimizer::limit_pushdown_past_window`

Crate `datafusion-physical-optimizer` · 1 public items · structured records in [`model/datafusion_physical_optimizer.limit_pushdown_past_window.json`](../model/datafusion_physical_optimizer.limit_pushdown_past_window.json)

## LimitPushPastWindows

`struct` · `datafusion_physical_optimizer::limit_pushdown_past_window::LimitPushPastWindows`

```rust
struct LimitPushPastWindows
```

**Implements**: `datafusion_session::physical_optimizer::PhysicalOptimizerRule`

**Derives**: Clone, Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_session::physical_optimizer::PhysicalOptimizerRule`**

```rust
fn name(&self) -> &str
fn optimize(&self, original: Arc<dyn ExecutionPlan>, config: &ConfigOptions) -> datafusion_common::Result<Arc<dyn ExecutionPlan>>
fn schema_check(&self) -> bool
```

This rule inspects [`ExecutionPlan`]'s attempting to find fetch limits that were not pushed
down by `LimitPushdown` because [BoundedWindowAggExec]s were "in the way". If the window is
bounded by [WindowFrameUnits::Rows] then we calculate the adjustment needed to grow the limit
and continue pushdown.

---
