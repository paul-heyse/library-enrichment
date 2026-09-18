# `datafusion_catalog::memory::schema`

Crate `datafusion-catalog` · 1 public items · structured records in [`model/datafusion_catalog.memory.schema.json`](../model/datafusion_catalog.memory.schema.json)

## MemorySchemaProvider

`struct` · `datafusion_catalog::memory::schema::MemorySchemaProvider`

Also reachable as `datafusion::catalog::MemorySchemaProvider`, `datafusion_catalog::MemorySchemaProvider`

```rust
struct MemorySchemaProvider
```

**Implements**: `datafusion_session::schema::SchemaProvider`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_session::schema::SchemaProvider`**

```rust
fn deregister_table(&self, name: &str) -> datafusion_common::Result<Option<Arc<dyn TableProvider>>>
fn register_table(&self, name: String, table: Arc<dyn TableProvider>) -> datafusion_common::Result<Option<Arc<dyn TableProvider>>>
async fn table(&self, name: &str) -> datafusion_common::Result<Option<Arc<dyn TableProvider>>, DataFusionError>
fn table_exist(&self, name: &str) -> bool
fn table_names(&self) -> Vec<String>
```

[Full member, field, variant and typed contracts](../operations/datafusion_catalog.memory.schema.MemorySchemaProvider.md).


Simple in-memory implementation of a schema.

---
