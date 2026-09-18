# `deltalake_core::writer::utils`

Crate `deltalake-core` · 2 public items · structured records in [`model/deltalake_core.writer.utils.json`](../model/deltalake_core.writer.utils.json)

## record_batch_from_message

`function` · `deltalake_core::writer::utils::record_batch_from_message`
[Full member contracts, output types and access classification](../operations/deltalake_core.writer.utils.record_batch_from_message.md)

Also reachable as `deltalake::writer::utils::record_batch_from_message`

```rust
fn record_batch_from_message(arrow_schema: std::sync::Arc<arrow_schema::Schema>, json: &[serde_json::Value]) -> errors::DeltaResult<arrow_array::RecordBatch>
```

Convert a vector of json values to a RecordBatch

---

## ShareableBuffer

`struct` · `deltalake_core::writer::utils::ShareableBuffer`
[Full member contracts, output types and access classification](../operations/deltalake_core.writer.utils.ShareableBuffer.md)

Also reachable as `deltalake::writer::utils::ShareableBuffer`

```rust
struct ShareableBuffer
```

**Implements**: `core::io::write::Write`

**Derives**: Clone, Debug, Default

**Methods** (5)

```rust
fn from_bytes(bytes: &[u8]) -> Self
fn into_inner(self) -> Option<Vec<u8>>
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn to_vec(&self) -> Vec<u8>
```

**via `core::io::write::Write`**

```rust
fn flush(&mut self) -> std::io::Result<()>
fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>
```

An in memory buffer that allows for shared ownership and interior mutability.
The underlying buffer is wrapped in an `Arc` and `RwLock`, so cloning the instance
allows multiple owners to have access to the same underlying buffer.

---
