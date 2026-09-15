# `datafusion_common::display`

Crate `datafusion-common` · 3 public items · structured records in [`model/datafusion_common.display.json`](../model/datafusion_common.display.json)

## PlanType

`enum` · `datafusion_common::display::PlanType`

Also reachable as `datafusion::logical_expr::PlanType`, `datafusion_expr::PlanType`, `datafusion_expr::logical_plan::PlanType`

```rust
enum PlanType
```

**Variants**: `InitialLogicalPlan`, `AnalyzedLogicalPlan`, `FinalAnalyzedLogicalPlan`, `OptimizedLogicalPlan`, `FinalLogicalPlan`, `InitialPhysicalPlan`, `InitialPhysicalPlanWithStats`, `InitialPhysicalPlanWithSchema`, `OptimizedPhysicalPlan`, `FinalPhysicalPlan`, `FinalPhysicalPlanWithStats`, `FinalPhysicalPlanWithSchema`, `PhysicalPlanError`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Represents which type of plan, when storing multiple
for use in EXPLAIN plans

---

## StringifiedPlan

`struct` · `datafusion_common::display::StringifiedPlan`

Also reachable as `datafusion::logical_expr::StringifiedPlan`, `datafusion_expr::StringifiedPlan`, `datafusion_expr::logical_plan::StringifiedPlan`

```rust
struct StringifiedPlan
```

**Fields**: `plan_type`, `plan`

**Implements**: `core::convert::From`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn new(plan_type: PlanType, plan: impl Into<String>) -> Self
fn should_display(&self, verbose_mode: bool) -> bool
```

Represents some sort of execution plan, in String form

---

## ToStringifiedPlan

`trait` · `datafusion_common::display::ToStringifiedPlan`

Also reachable as `datafusion::logical_expr::ToStringifiedPlan`, `datafusion_expr::ToStringifiedPlan`, `datafusion_expr::logical_plan::ToStringifiedPlan`

```rust
trait ToStringifiedPlan
```

**Implementors** (1)

- `datafusion_expr::logical_plan::plan::LogicalPlan`

**Methods** (1)

```rust
fn to_stringified(&self, plan_type: PlanType) -> StringifiedPlan
```

Trait for something that can be formatted as a stringified plan

---
