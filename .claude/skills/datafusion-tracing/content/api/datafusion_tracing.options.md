# `datafusion_tracing::options`

Crate `datafusion-tracing` · 2 public items · structured records in [`model/datafusion_tracing.options.json`](../model/datafusion_tracing.options.json)

## InstrumentationOptions

`struct` · `datafusion_tracing::options::InstrumentationOptions`

Also reachable as `datafusion_tracing::InstrumentationOptions`

```rust
struct InstrumentationOptions
```

**Fields**: `custom_fields`, `preview_fn`, `preview_limit`, `record_metrics`

**Derives**: Clone, Debug, Default

**Methods** (1)

```rust
fn builder() -> InstrumentationOptionsBuilder
```

Configuration options for instrumented execution plans.

---

## InstrumentationOptionsBuilder

`struct` · `datafusion_tracing::options::InstrumentationOptionsBuilder`

```rust
struct InstrumentationOptionsBuilder
```

**Derives**: Default

**Methods** (6)

```rust
fn add_custom_field<K: Into<String>, V: Into<String>>(self, key: K, value: V) -> Self
fn build(self) -> InstrumentationOptions
fn custom_fields(self, fields: HashMap<String, String>) -> Self
fn preview_fn(self, func: Arc<dyn Fn(&datafusion::arrow::record_batch::RecordBatch) -> Result<String, datafusion::arrow::error::ArrowError> + Send + Sync>) -> Self
fn preview_limit(self, limit: usize) -> Self
fn record_metrics(self, record: bool) -> Self
```

The builder for `InstrumentationOptions`.

---
