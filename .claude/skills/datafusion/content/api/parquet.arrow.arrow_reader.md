# `parquet::arrow::arrow_reader`

Crate `parquet` · 6 public items · structured records in [`model/parquet.arrow.arrow_reader.json`](../model/parquet.arrow.arrow_reader.json)

## DEFAULT_BATCH_SIZE

`constant` · `parquet::arrow::arrow_reader::DEFAULT_BATCH_SIZE`

```rust
const DEFAULT_BATCH_SIZE: usize = 1024
```

[Full member, field, variant and typed contracts](../operations/parquet.arrow.arrow_reader.DEFAULT_BATCH_SIZE.md).


Default batch size for reading parquet files

---

## ArrowReaderBuilder

`struct` · `parquet::arrow::arrow_reader::ArrowReaderBuilder`

```rust
struct ArrowReaderBuilder<T>
```

**Derives**: Debug

**Methods** (28)

```rust
fn build(self) -> Result<ParquetPushDecoder, ParquetError>
fn build(self) -> Result<ParquetRecordBatchStream<T>>
fn build(self) -> Result<ParquetRecordBatchReader>
async fn get_row_group_column_bloom_filter(&mut self, row_group_idx: usize, column_idx: usize) -> Result<Option<Sbbf>>
fn get_row_group_column_bloom_filter(&self, row_group_idx: usize, column_idx: usize) -> Result<Option<Sbbf>>
fn metadata(&self) -> &Arc<ParquetMetaData>
async fn new(input: T) -> Result<Self>
fn new_with_metadata(arrow_reader_metadata: ArrowReaderMetadata) -> Self
fn new_with_metadata(input: T, metadata: ArrowReaderMetadata) -> Self
fn new_with_metadata(input: T, metadata: ArrowReaderMetadata) -> Self
async fn new_with_options(input: T, options: ArrowReaderOptions) -> Result<Self>
fn parquet_schema(&self) -> &SchemaDescriptor
fn schema(&self) -> &SchemaRef
fn try_new(reader: T) -> Result<Self>
fn try_new_decoder(parquet_metadata: Arc<ParquetMetaData>) -> Result<Self, ParquetError>
fn try_new_decoder_with_options(parquet_metadata: Arc<ParquetMetaData>, arrow_reader_options: ArrowReaderOptions) -> Result<Self, ParquetError>
fn try_new_with_options(reader: T, options: ArrowReaderOptions) -> Result<Self>
fn with_batch_size(self, batch_size: usize) -> Self
fn with_buffers(self, buffers: PushBuffers) -> Self
fn with_limit(self, limit: usize) -> Self
fn with_max_predicate_cache_size(self, max_predicate_cache_size: usize) -> Self
fn with_metrics(self, metrics: ArrowReaderMetrics) -> Self
fn with_offset(self, offset: usize) -> Self
fn with_projection(self, mask: ProjectionMask) -> Self
fn with_row_filter(self, filter: RowFilter) -> Self
fn with_row_groups(self, row_groups: Vec<usize>) -> Self
fn with_row_selection(self, selection: RowSelection) -> Self
fn with_row_selection_policy(self, policy: RowSelectionPolicy) -> Self
```

[Full member, field, variant and typed contracts](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md).


Builder for constructing Parquet readers that decode into [Apache Arrow]
arrays.

Most users should use one of the following specializations:

* synchronous API: [`ParquetRecordBatchReaderBuilder`]
* `async` API: [`ParquetRecordBatchStreamBuilder`]
* decoder API: [`ParquetPushDecoderBuilder`]

# Features
* Projection pushdown: [`Self::with_projection`]
* Cached metadata: [`ArrowReaderMetadata::load`]
* Offset skipping: [`Self::with_offset`] and [`Self::with_limit`]
* Row group filtering: [`Self::with_row_groups`]
* Range filtering: [`Self::with_row_selection`]
* Row level filtering: [`Self::with_row_filter`]

# Implementing Predicate Pushdown

[`Self::with_row_filter`] permits filter evaluation *during* the decoding
process, which is efficient and allows the most low level optimizations.

However, most Parquet based systems will apply filters at many steps prior
to decoding such as pruning files, row groups and data pages. This crate
provides the low level APIs needed to implement such filtering, but does not
include any logic to actually evaluate predicates. For example:

* [`Self::with_row_groups`] for Row Group pruning
* [`Self::with_row_selection`] for data page pruning
* [`StatisticsConverter`] to convert Parquet statistics to Arrow arrays

The rationale for this design is that implementing predicate pushdown is a
complex topic and varies significantly from system to system. For example

1. Predicates supported (do you support predicates like prefix matching, user defined functions, etc)
2. Evaluating predicates on multiple files (with potentially different but compatible schemas)
3. Evaluating predicates using information from an external metadata catalog (e.g. Apache Iceberg or similar)
4. Interleaving fetching metadata, evaluating predicates, and decoding files

You can read more about this design in the [Querying Parquet with
Millisecond Latency] Arrow blog post.

[`ParquetRecordBatchStreamBuilder`]: crate::arrow::async_reader::ParquetRecordBatchStreamBuilder
[`ParquetPushDecoderBuilder`]: crate::arrow::push_decoder::ParquetPushDecoderBuilder
[Apache Arrow]: https://arrow.apache.org/
[`StatisticsConverter`]: statistics::StatisticsConverter
[Querying Parquet with Millisecond Latency]: https://arrow.apache.org/blog/2022/12/26/querying-parquet-with-millisecond-latency/

---

## ArrowReaderMetadata

`struct` · `parquet::arrow::arrow_reader::ArrowReaderMetadata`

```rust
struct ArrowReaderMetadata
```

**Derives**: Clone, Debug

**Methods** (6)

```rust
fn load<T: ChunkReader>(reader: &T, options: ArrowReaderOptions) -> Result<Self>
async fn load_async<T: AsyncFileReader>(input: &mut T, options: ArrowReaderOptions) -> Result<Self>
fn metadata(&self) -> &Arc<ParquetMetaData>
fn parquet_schema(&self) -> &SchemaDescriptor
fn schema(&self) -> &SchemaRef
fn try_new(metadata: Arc<ParquetMetaData>, options: ArrowReaderOptions) -> Result<Self>
```

[Full member, field, variant and typed contracts](../operations/parquet.arrow.arrow_reader.ArrowReaderMetadata.md).


The metadata necessary to construct a [`ArrowReaderBuilder`]

Note this structure is cheaply clone-able as it consists of several arcs.

This structure allows

1. Loading metadata for a file once and then using that same metadata to
   construct multiple separate readers, for example, to distribute readers
   across multiple threads

2. Using a cached copy of the [`ParquetMetadata`] rather than reading it
   from the file each time a reader is constructed.

[`ParquetMetadata`]: crate::file::metadata::ParquetMetaData

---

## ArrowReaderOptions

`struct` · `parquet::arrow::arrow_reader::ArrowReaderOptions`

```rust
struct ArrowReaderOptions
```

**Derives**: Clone, Debug, Default

**Methods** (19)

```rust
fn column_index_policy(&self) -> PageIndexPolicy
fn file_decryption_properties(&self) -> Option<&Arc<FileDecryptionProperties>>
fn metadata_options(&self) -> &ParquetMetaDataOptions
fn new() -> Self
fn offset_index_policy(&self) -> PageIndexPolicy
fn page_index(&self) -> bool
fn with_column_index_policy(self, policy: PageIndexPolicy) -> Self
fn with_column_stats_policy(self, policy: ParquetStatisticsPolicy) -> Self
fn with_encoding_stats_as_mask(self, val: bool) -> Self
fn with_encoding_stats_policy(self, policy: ParquetStatisticsPolicy) -> Self
fn with_file_decryption_properties(self, file_decryption_properties: Arc<FileDecryptionProperties>) -> Self
fn with_offset_index_policy(self, policy: PageIndexPolicy) -> Self
fn with_page_index(self, page_index: bool) -> Self
fn with_page_index_policy(self, policy: PageIndexPolicy) -> Self
fn with_parquet_schema(self, schema: Arc<SchemaDescriptor>) -> Self
fn with_schema(self, schema: SchemaRef) -> Self
fn with_size_stats_policy(self, policy: ParquetStatisticsPolicy) -> Self
fn with_skip_arrow_metadata(self, skip_arrow_metadata: bool) -> Self
fn with_virtual_columns(self, virtual_columns: Vec<FieldRef>) -> Result<Self>
```

[Full member, field, variant and typed contracts](../operations/parquet.arrow.arrow_reader.ArrowReaderOptions.md).


Options that control how [`ParquetMetaData`] is read when constructing
an Arrow reader.

To use these options, pass them to one of the following methods:
* [`ParquetRecordBatchReaderBuilder::try_new_with_options`]
* [`ParquetRecordBatchStreamBuilder::new_with_options`]

For fine-grained control over metadata loading, use
[`ArrowReaderMetadata::load`] to load metadata with these options,

See [`ArrowReaderBuilder`] for how to configure how the column data
is then read from the file, including projection and filter pushdown

[`ParquetRecordBatchStreamBuilder::new_with_options`]: crate::arrow::async_reader::ParquetRecordBatchStreamBuilder::new_with_options

---

## ParquetRecordBatchReader

`struct` · `parquet::arrow::arrow_reader::ParquetRecordBatchReader`

```rust
struct ParquetRecordBatchReader
```

**Implements**: `arrow_array::record_batch::RecordBatchReader`, `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**Methods** (2)

```rust
fn try_new<T: ChunkReader + 'static>(reader: T, batch_size: usize) -> Result<Self>
fn try_new_with_row_groups(levels: &FieldLevels, row_groups: &dyn RowGroups, batch_size: usize, selection: Option<RowSelection>) -> Result<Self>
```

**via `arrow_array::record_batch::RecordBatchReader`**

```rust
fn schema(&self) -> SchemaRef
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

[Full member, field, variant and typed contracts](../operations/parquet.arrow.arrow_reader.ParquetRecordBatchReader.md).


Reads Parquet data as Arrow [`RecordBatch`]es

This struct implements the [`RecordBatchReader`] trait and is an
`Iterator<Item = ArrowResult<RecordBatch>>` that yields [`RecordBatch`]es.

Typically, either reads from a file or an in memory buffer [`Bytes`]

Created by [`ParquetRecordBatchReaderBuilder`]

[`Bytes`]: bytes::Bytes

---

## ParquetRecordBatchReaderBuilder

`type_alias` · `parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder`

```rust
type ParquetRecordBatchReaderBuilder<T> = ArrowReaderBuilder<SyncReader<T>>
```

[Full member, field, variant and typed contracts](../operations/parquet.arrow.arrow_reader.ParquetRecordBatchReaderBuilder.md).


Creates [`ParquetRecordBatchReader`] for reading Parquet files into Arrow [`RecordBatch`]es

# See Also
* [`crate::arrow::async_reader::ParquetRecordBatchStreamBuilder`] for an async API
* [`crate::arrow::push_decoder::ParquetPushDecoderBuilder`] for a SansIO decoder API
* [`ArrowReaderBuilder`] for additional member functions

---
