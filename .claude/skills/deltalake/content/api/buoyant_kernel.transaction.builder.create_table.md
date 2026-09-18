# `buoyant_kernel::transaction::builder::create_table`

Crate `buoyant_kernel` · 1 public items · structured records in [`model/buoyant_kernel.transaction.builder.create_table.json`](../model/buoyant_kernel.transaction.builder.create_table.json)

## CreateTableTransactionBuilder

`struct` · `buoyant_kernel::transaction::builder::create_table::CreateTableTransactionBuilder`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.transaction.builder.create_table.CreateTableTransactionBuilder.md)

Also reachable as `buoyant_kernel::transaction::create_table::CreateTableTransactionBuilder`, `delta_kernel::transaction::builder::create_table::CreateTableTransactionBuilder`

```rust
struct CreateTableTransactionBuilder
```

**Methods** (5)

```rust
fn build(self, engine: &dyn Engine, committer: Box<dyn Committer>) -> DeltaResult<CreateTableTransaction>
fn new(path: impl AsRef<str>, schema: SchemaRef, engine_info: impl Into<String>) -> Self
fn with_correlation_id(self, correlation_id: impl Into<Arc<str>>) -> Self
fn with_data_layout(self, layout: DataLayout) -> Self
fn with_table_properties<I, K, V>(self, properties: I) -> Self where I: IntoIterator<Item = (K, V)>, K: Into<String>, V: Into<String>
```

Builder for configuring a new Delta table.

Use this to configure table properties before building a [`CreateTableTransaction`].
If the table build fails, no transaction will be created.

Created via [`create_table()`](super::super::create_table::create_table).

---
