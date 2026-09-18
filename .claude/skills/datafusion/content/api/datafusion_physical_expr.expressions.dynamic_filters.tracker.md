# `datafusion_physical_expr::expressions::dynamic_filters::tracker`

Crate `datafusion-physical-expr` · 2 public items · structured records in [`model/datafusion_physical_expr.expressions.dynamic_filters.tracker.json`](../model/datafusion_physical_expr.expressions.dynamic_filters.tracker.json)

## DynamicFilterTracking

`enum` · `datafusion_physical_expr::expressions::dynamic_filters::tracker::DynamicFilterTracking`

Also reachable as `datafusion::physical_expr::DynamicFilterTracking`, `datafusion_physical_expr::DynamicFilterTracking`, `datafusion_physical_expr::expressions::DynamicFilterTracking`, `datafusion_physical_plan::execution_plan::expressions::DynamicFilterTracking`, `datafusion_physical_plan::expressions::DynamicFilterTracking`

```rust
enum DynamicFilterTracking
```

**Variants**: `Static`, `AllComplete`, `Watching`

**Derives**: Debug

**Methods** (3)

```rust
fn classify(predicate: &Arc<dyn PhysicalExpr>) -> Self
fn contains_dynamic_filter(&self) -> bool
fn watcher(&mut self) -> Option<&mut DynamicFilterTracker>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.expressions.dynamic_filters.tracker.DynamicFilterTracking.md).


Classification of a predicate according to the dynamic filters it contains.

Produced by [`DynamicFilterTracking::classify`] with a single tree walk so
callers can answer both "is it worth pruning at all?" and "do I need to keep
watching?" without traversing the predicate twice.

---

## DynamicFilterTracker

`struct` · `datafusion_physical_expr::expressions::dynamic_filters::tracker::DynamicFilterTracker`

Also reachable as `datafusion::physical_expr::DynamicFilterTracker`, `datafusion_physical_expr::DynamicFilterTracker`, `datafusion_physical_expr::expressions::DynamicFilterTracker`, `datafusion_physical_plan::execution_plan::expressions::DynamicFilterTracker`, `datafusion_physical_plan::expressions::DynamicFilterTracker`

```rust
struct DynamicFilterTracker
```

**Derives**: Debug

**Methods** (1)

```rust
fn changed(&mut self) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.expressions.dynamic_filters.tracker.DynamicFilterTracker.md).


Watches every still-incomplete [`DynamicFilterPhysicalExpr`] reachable from a
predicate and reports, cheaply, whether any of them has been updated since
the last check.

Obtain one from [`DynamicFilterTracking::classify`] via
[`DynamicFilterTracking::watcher`]; the `Watching` variant carries it only
when there is at least one dynamic filter that can still change.

---
