# `deltalake_core::delta_datafusion::engine`

Crate `deltalake-core` · 1 public items · structured records in [`model/deltalake_core.delta_datafusion.engine.json`](../model/deltalake_core.delta_datafusion.engine.json)

## DataFusionEngine

`struct` · `deltalake_core::delta_datafusion::engine::DataFusionEngine`
[Full member contracts, output types and access classification](../operations/deltalake_core.delta_datafusion.engine.DataFusionEngine.md)

Also reachable as `deltalake::delta_datafusion::engine::DataFusionEngine`

```rust
struct DataFusionEngine
```

**Implements**: `buoyant_kernel::Engine`

**Derives**: Clone

**Methods** (3)

```rust
fn new(ctx: Arc<TaskContext>, handle: Handle) -> Self
fn new_from_context(ctx: Arc<TaskContext>) -> Arc<Self>
fn new_from_session(session: &dyn Session) -> Arc<Self>
```

**via `buoyant_kernel::Engine`**

```rust
fn evaluation_handler(&self) -> Arc<dyn EvaluationHandler>
fn json_handler(&self) -> Arc<dyn JsonHandler>
fn parquet_handler(&self) -> Arc<dyn ParquetHandler>
fn storage_handler(&self) -> Arc<dyn StorageHandler>
```

A Datafusion based Kernel Engine

---
