# `deltalake_core::delta_datafusion::planner`

Crate `deltalake-core` · 2 public items · structured records in [`model/deltalake_core.delta_datafusion.planner.json`](../model/deltalake_core.delta_datafusion.planner.json)

## DeltaExtensionPlanner

`struct` · `deltalake_core::delta_datafusion::planner::DeltaExtensionPlanner`

Also reachable as `deltalake::delta_datafusion::planner::DeltaExtensionPlanner`

```rust
struct DeltaExtensionPlanner
```

**Implements**: `datafusion_session::planner::ExtensionPlanner`

**Methods** (1)

```rust
fn new() -> Arc<Self>
```

**via `datafusion_session::planner::ExtensionPlanner`**

```rust
async fn plan_extension(&self, planner: &dyn PhysicalPlanner, node: &dyn UserDefinedLogicalNode, logical_inputs: &[&LogicalPlan], physical_inputs: &[Arc<dyn ExecutionPlan>], session_state: &dyn Session, planning_ctx: &PhysicalPlanningContext) -> DataFusionResult<Option<Arc<dyn ExecutionPlan>>>
```

Extension [`PhysicalPlanner`](datafusion::physical_planner::PhysicalPlanner) that knows
how to lower delta-rs custom logical nodes into executable physical plans.

---

## DeltaPlanner

`struct` · `deltalake_core::delta_datafusion::planner::DeltaPlanner`

Also reachable as `deltalake::delta_datafusion::planner::DeltaPlanner`

```rust
struct DeltaPlanner
```

**Implements**: `datafusion_session::planner::QueryPlanner`

**Derives**: Debug

**Methods** (1)

```rust
fn new() -> Arc<Self>
```

**via `datafusion_session::planner::QueryPlanner`**

```rust
async fn create_physical_plan(&self, logical_plan: &LogicalPlan, session: &dyn Session) -> DataFusionResult<Arc<dyn ExecutionPlan>>
```

Deltaplanner

---
