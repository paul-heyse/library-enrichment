# `deltalake_core::data_catalog::storage`

Crate `deltalake-core` · 1 public items · structured records in [`model/deltalake_core.data_catalog.storage.json`](../model/deltalake_core.data_catalog.storage.json)

## ListingSchemaProvider

`struct` · `deltalake_core::data_catalog::storage::ListingSchemaProvider`
[Full member contracts, output types and access classification](../operations/deltalake_core.data_catalog.storage.ListingSchemaProvider.md)

Also reachable as `deltalake::data_catalog::storage::ListingSchemaProvider`

```rust
struct ListingSchemaProvider
```

**Implements**: `datafusion_session::schema::SchemaProvider`

**Derives**: Debug

**Methods** (2)

```rust
async fn refresh(&self) -> datafusion::common::Result<()>
fn try_new(root_uri: impl AsRef<str>, options: Option<HashMap<String, String>>) -> DeltaResult<Self>
```

**via `datafusion_session::schema::SchemaProvider`**

```rust
fn deregister_table(&self, _name: &str) -> datafusion::common::Result<Option<Arc<dyn TableProvider>>>
fn register_table(&self, _name: String, _table: Arc<dyn TableProvider>) -> datafusion::common::Result<Option<Arc<dyn TableProvider>>>
async fn table(&self, name: &str) -> datafusion::common::Result<Option<Arc<dyn TableProvider>>>
fn table_exist(&self, name: &str) -> bool
fn table_names(&self) -> Vec<String>
```

A `SchemaProvider` that scans an `ObjectStore` to automatically discover delta tables.

A subfolder relationship is assumed, i.e. given:
authority = s3://host.example.com:3000
path = /data/tpch

A table called "customer" will be registered for the folder:
s3://host.example.com:3000/data/tpch/customer

assuming it contains valid deltalake data, i.e a `_delta_log` folder:
s3://host.example.com:3000/data/tpch/customer/_delta_log/

---
