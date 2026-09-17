# `datafusion_tracing::planner`

Crate `datafusion-tracing` · 2 public items · structured records in [`model/datafusion_tracing.planner.json`](../model/datafusion_tracing.planner.json)

## span_at_level

`macro` · `datafusion_tracing::planner::span_at_level`

```rust
macro_rules! span_at_level
```

Creates a span at the specified tracing level with the given name and fields.

This macro eliminates the need for repetitive match blocks when creating spans
at different levels. The level must be a `tracing::Level` value.

---

## TracingQueryPlanner

`struct` · `datafusion_tracing::planner::TracingQueryPlanner`

```rust
struct TracingQueryPlanner
```

**Implements**: `datafusion_session::planner::QueryPlanner`

**Derives**: Debug

**Methods** (2)

```rust
fn instrument_state_with_level(state: SessionState, level: Level) -> SessionState
fn new_with_level(inner: Arc<dyn QueryPlanner + Send + Sync>, level: Level) -> Self
```

**via `datafusion_session::planner::QueryPlanner`**

```rust
async fn create_physical_plan(&self, logical_plan: &LogicalPlan, session_state: &dyn Session) -> Result<Arc<dyn ExecutionPlan>>
```

A `QueryPlanner` that instruments the creation of the physical plan.

This is automatically applied when instrumenting a `SessionState` with physical
optimizer instrumentation enabled (PhaseOnly or Full).

---
