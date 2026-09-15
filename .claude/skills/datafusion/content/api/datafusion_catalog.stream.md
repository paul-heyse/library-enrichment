# `datafusion_catalog::stream`

Crate `datafusion-catalog` · 6 public items · structured records in [`model/datafusion_catalog.stream.json`](../model/datafusion_catalog.stream.json)

## StreamEncoding

`enum` · `datafusion_catalog::stream::StreamEncoding`

Also reachable as `datafusion::datasource::stream::StreamEncoding`

```rust
enum StreamEncoding
```

**Variants**: `Csv`, `Json`

**Implements**: `core::str::traits::FromStr`

**Derives**: Clone, Debug

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> std::result::Result<Self, Self::Err>
```

The data encoding for [`StreamTable`]

---

## FileStreamProvider

`struct` · `datafusion_catalog::stream::FileStreamProvider`

Also reachable as `datafusion::datasource::stream::FileStreamProvider`

```rust
struct FileStreamProvider
```

**Fields**: `schema`

**Implements**: `datafusion_catalog::stream::StreamProvider`

**Derives**: Debug

**Methods** (4)

```rust
fn new_file(schema: SchemaRef, location: PathBuf) -> Self
fn with_batch_size(self, batch_size: usize) -> Self
fn with_encoding(self, encoding: StreamEncoding) -> Self
fn with_header(self, header: bool) -> Self
```

**via `datafusion_catalog::stream::StreamProvider`**

```rust
fn reader(&self) -> Result<Box<dyn RecordBatchReader>>
fn schema(&self) -> &SchemaRef
fn stream_write_display(&self, _t: DisplayFormatType, f: &mut Formatter<'_>) -> std::fmt::Result
fn writer(&self) -> Result<Box<dyn RecordBatchWriter>>
```

Stream data from the file at `location`

* Data will be read sequentially from the provided `location`
* New data will be appended to the end of the file

The encoding can be configured with [`Self::with_encoding`] and
defaults to [`StreamEncoding::Csv`]

---

## StreamConfig

`struct` · `datafusion_catalog::stream::StreamConfig`

Also reachable as `datafusion::datasource::stream::StreamConfig`

```rust
struct StreamConfig
```

**Derives**: Debug

**Methods** (3)

```rust
fn new(source: Arc<dyn StreamProvider>) -> Self
fn with_constraints(self, constraints: Constraints) -> Self
fn with_order(self, order: Vec<Vec<SortExpr>>) -> Self
```

The configuration for a [`StreamTable`]

---

## StreamTable

`struct` · `datafusion_catalog::stream::StreamTable`

Also reachable as `datafusion::datasource::stream::StreamTable`

```rust
struct StreamTable
```

**Implements**: `datafusion_session::table::TableProvider`

**Derives**: Debug

**Methods** (1)

```rust
fn new(config: Arc<StreamConfig>) -> Self
```

**via `datafusion_session::table::TableProvider`**

```rust
fn constraints(&self) -> Option<&Constraints>
async fn insert_into(&self, _state: &dyn Session, input: Arc<dyn ExecutionPlan>, _insert_op: InsertOp) -> Result<Arc<dyn ExecutionPlan>>
async fn scan(&self, state: &dyn Session, projection: Option<&Vec<usize>>, _filters: &[Expr], limit: Option<usize>) -> Result<Arc<dyn ExecutionPlan>>
fn schema(&self) -> SchemaRef
fn table_type(&self) -> TableType
```

A [`TableProvider`] for an unbounded stream source

Currently only reading from / appending to a single file in-place is supported, but
other stream sources and sinks may be added in future.

Applications looking to read/write datasets comprising multiple files, e.g. [Hadoop]-style
data stored in object storage, should instead consider [`ListingTable`].

[Hadoop]: https://hadoop.apache.org/
[`ListingTable`]: https://docs.rs/datafusion/latest/datafusion/datasource/listing/struct.ListingTable.html

---

## StreamTableFactory

`struct` · `datafusion_catalog::stream::StreamTableFactory`

Also reachable as `datafusion::datasource::stream::StreamTableFactory`

```rust
struct StreamTableFactory
```

**Implements**: `datafusion_session::table::TableProviderFactory`

**Derives**: Debug, Default

**via `datafusion_session::table::TableProviderFactory`**

```rust
async fn create(&self, state: &dyn Session, cmd: &CreateExternalTable) -> Result<Arc<dyn TableProvider>>
```

A [`TableProviderFactory`] for [`StreamTable`]

---

## StreamProvider

`trait` · `datafusion_catalog::stream::StreamProvider`

Also reachable as `datafusion::datasource::stream::StreamProvider`

```rust
trait StreamProvider: std::fmt::Debug + Send + Sync
```

**Implementors** (1)

- `datafusion_catalog::stream::FileStreamProvider`

**Methods** (4)

```rust
fn reader(&self) -> Result<Box<dyn RecordBatchReader>>
fn schema(&self) -> &SchemaRef
fn stream_write_display(&self, t: DisplayFormatType, f: &mut Formatter<'_>) -> std::fmt::Result
fn writer(&self) -> Result<Box<dyn RecordBatchWriter>>
```

The StreamProvider trait is used as a generic interface for reading and writing from streaming
data sources (such as FIFO, Websocket, Kafka, etc.).  Implementations of the provider are
responsible for providing a `RecordBatchReader` and optionally a `RecordBatchWriter`.

---
