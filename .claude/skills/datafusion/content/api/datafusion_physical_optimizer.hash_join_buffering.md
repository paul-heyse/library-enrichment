# `datafusion_physical_optimizer::hash_join_buffering`

Crate `datafusion-physical-optimizer` · 1 public items · structured records in [`model/datafusion_physical_optimizer.hash_join_buffering.json`](../model/datafusion_physical_optimizer.hash_join_buffering.json)

## HashJoinBuffering

`struct` · `datafusion_physical_optimizer::hash_join_buffering::HashJoinBuffering`

```rust
struct HashJoinBuffering
```

**Implements**: `datafusion_session::physical_optimizer::PhysicalOptimizerRule`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_session::physical_optimizer::PhysicalOptimizerRule`**

```rust
fn name(&self) -> &str
fn optimize(&self, plan: Arc<dyn ExecutionPlan>, config: &ConfigOptions) -> datafusion_common::Result<Arc<dyn ExecutionPlan>>
fn schema_check(&self) -> bool
```

Looks for all the [HashJoinExec]s in the plan and places a [BufferExec] node with the
configured capacity in the probe side:

```text
           ┌───────────────────┐
           │   HashJoinExec    │
           └─────▲────────▲────┘
         ┌───────┘        └─────────┐
         │                          │
┌────────────────┐         ┌─────────────────┐
│   Build side   │       + │   BufferExec    │
└────────────────┘         └────────▲────────┘
                                    │
                           ┌────────┴────────┐
                           │   Probe side    │
                           └─────────────────┘
```

Which allows eagerly pulling it even before the build side has completely finished.

---
