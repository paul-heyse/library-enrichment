# `datafusion_catalog::listing_schema`

Crate `datafusion-catalog` · 1 public items · structured records in [`model/datafusion_catalog.listing_schema.json`](../model/datafusion_catalog.listing_schema.json)

## ListingSchemaProvider

`struct` · `datafusion_catalog::listing_schema::ListingSchemaProvider`

```rust
struct ListingSchemaProvider
```

**Implements**: `datafusion_session::schema::SchemaProvider`

**Derives**: Debug

**Methods** (2)

```rust
fn new(authority: String, path: object_store::path::Path, factory: Arc<dyn TableProviderFactory>, store: Arc<dyn ObjectStore>, format: String) -> Self
async fn refresh(&self, state: &dyn Session) -> datafusion_common::Result<()>
```

**via `datafusion_session::schema::SchemaProvider`**

```rust
fn deregister_table(&self, name: &str) -> datafusion_common::Result<Option<Arc<dyn TableProvider>>>
fn register_table(&self, name: String, table: Arc<dyn TableProvider>) -> datafusion_common::Result<Option<Arc<dyn TableProvider>>>
async fn table(&self, name: &str) -> Result<Option<Arc<dyn TableProvider>>, DataFusionError>
fn table_exist(&self, name: &str) -> bool
fn table_names(&self) -> Vec<String>
```

[Full member, field, variant and typed contracts](../operations/datafusion_catalog.listing_schema.ListingSchemaProvider.md).


A [`SchemaProvider`] that scans an [`ObjectStore`] to automatically discover tables

A subfolder relationship is assumed, i.e. given:
- authority = `s3://host.example.com:3000`
- path = `/data/tpch`
- factory = `DeltaTableFactory`

A table called "customer" will be registered for the folder:
`s3://host.example.com:3000/data/tpch/customer`

assuming it contains valid deltalake data, i.e:
- `s3://host.example.com:3000/data/tpch/customer/part-00000-xxxx.snappy.parquet`
- `s3://host.example.com:3000/data/tpch/customer/_delta_log/`

[`ObjectStore`]: object_store::ObjectStore

---
