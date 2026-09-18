# `deltalake_core::table::blind`

Crate `deltalake-core` · 1 public items · structured records in [`model/deltalake_core.table.blind.json`](../model/deltalake_core.table.blind.json)

## BlindDeltaTable

`struct` · `deltalake_core::table::blind::BlindDeltaTable`
[Full member contracts, output types and access classification](../operations/deltalake_core.table.blind.BlindDeltaTable.md)

Also reachable as `deltalake::BlindDeltaTable`, `deltalake::table::BlindDeltaTable`, `deltalake_core::BlindDeltaTable`, `deltalake_core::table::BlindDeltaTable`

```rust
struct BlindDeltaTable
```

**Derives**: Clone, Debug

**Methods** (15)

```rust
fn arrow_schema(&self) -> DeltaResult<ArrowSchemaRef>
async fn commit(&mut self, adds: Vec<Add>) -> DeltaResult<u64>
fn is_append_only(&self) -> bool
fn log_store(&self) -> LogStoreRef
fn metadata(&self) -> &Metadata
fn object_store(&self) -> ObjectStoreRef
fn protocol(&self) -> &Protocol
fn schema(&self) -> KernelSchemaRef
fn snapshot(&self) -> &Snapshot
fn table_properties(&self) -> &TableProperties
fn table_url(&self) -> &Url
async fn try_new(table_uri: impl AsRef<str>) -> DeltaResult<Self>
async fn try_new_with_log_store(log_store: LogStoreRef) -> DeltaResult<Self>
async fn try_new_with_options(table_uri: impl AsRef<str>, storage_options: HashMap<String, String>) -> DeltaResult<Self>
fn version(&self) -> u64
```

A Delta table optimized for blind append-only write operations.

`BlindDeltaTable` loads only the table metadata (protocol, schema, properties)
without scanning file statistics. This makes it ideal for:

- Large tables with many files where stats parsing is expensive
- Append-only workloads that don't need to read existing data
- High-throughput write scenarios

# Type Safety

This type intentionally does not expose methods like `files()` or `log_data()`
that would require loading file statistics. This prevents accidental use of
operations like merge or delete that are incompatible with append-only tables.

# Kernel Transaction API

This implementation uses the Kernel Transaction API directly for commits,
bypassing the `CommitBuilder` used by the standard `DeltaTable`. This provides
a more lightweight commit path optimized for append-only workloads.

---
