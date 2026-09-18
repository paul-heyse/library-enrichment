# `buoyant_kernel::metrics::metered_engine`

Crate `buoyant_kernel` · 1 public items · structured records in [`model/buoyant_kernel.metrics.metered_engine.json`](../model/buoyant_kernel.metrics.metered_engine.json)

## MeteredDeltaEngine

`struct` · `buoyant_kernel::metrics::metered_engine::MeteredDeltaEngine`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.metrics.metered_engine.MeteredDeltaEngine.md)

Also reachable as `buoyant_kernel::metrics::MeteredDeltaEngine`, `delta_kernel::metrics::metered_engine::MeteredDeltaEngine`

```rust
struct MeteredDeltaEngine
```

**Implements**: `buoyant_kernel::Engine`

**Derives**: Debug

**Methods** (1)

```rust
fn new(inner: Arc<dyn Engine>) -> Self
```

**via `buoyant_kernel::Engine`**

```rust
fn evaluation_handler(&self) -> Arc<dyn EvaluationHandler>
fn json_handler(&self) -> Arc<dyn JsonHandler>
fn parquet_handler(&self) -> Arc<dyn ParquetHandler>
fn storage_handler(&self) -> Arc<dyn StorageHandler>
```

Decorator over any [`Engine`] that meters its storage, JSON, and Parquet handlers.
See module docs.

---
