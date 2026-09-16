# `deltalake_core::writer::json`

Crate `deltalake-core` · 1 public items · structured records in [`model/deltalake_core.writer.json.json`](../model/deltalake_core.writer.json.json)

## JsonWriter

`struct` · `deltalake_core::writer::json::JsonWriter`

Also reachable as `deltalake::writer::JsonWriter`, `deltalake::writer::json::JsonWriter`, `deltalake_core::writer::JsonWriter`

```rust
struct JsonWriter
```

**Implements**: `deltalake_core::writer::DeltaWriter`

**Derives**: Debug

**Methods** (7)

```rust
fn arrow_schema(&self) -> ArrowSchemaRef
fn buffer_len(&self) -> usize
fn buffered_record_batch_count(&self) -> usize
fn for_table(table: &DeltaTable) -> Result<JsonWriter, DeltaTableError>
fn reset(&mut self)
async fn try_new(table_url: Url, schema_ref: ArrowSchemaRef, partition_columns: Option<Vec<String>>, storage_options: Option<HashMap<String, String>>) -> Result<Self, DeltaTableError>
fn with_target_file_size(self, target_file_size: u64) -> Self
```

**via `deltalake_core::writer::DeltaWriter`**

```rust
async fn flush(&mut self) -> Result<Vec<Add>, DeltaTableError>
async fn write(&mut self, values: Vec<Value>) -> Result<(), DeltaTableError>
async fn write_with_mode(&mut self, values: Vec<Value>, mode: WriteMode) -> Result<(), DeltaTableError>
```

Writes messages to a delta lake table.

Batches are streamed to storage as they are written, and a flush window
commits all-or-nothing: if any write returns an error — including a
transient IO error from the object store — every batch buffered since the
last flush is discarded along with the failing one, and the caller must
re-write all of them. (Validation errors caught before the data reaches
storage fail only that call and leave the window untouched. A write with
malformed JSON records rejects the whole call and writes nothing.)

---
