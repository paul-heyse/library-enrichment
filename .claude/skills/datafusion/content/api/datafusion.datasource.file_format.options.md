# `datafusion::datasource::file_format::options`

Crate `datafusion` · 7 public items · structured records in [`model/datafusion.datasource.file_format.options.json`](../model/datafusion.datasource.file_format.options.json)

## ArrowReadOptions

`struct` · `datafusion::datasource::file_format::options::ArrowReadOptions`

Also reachable as `datafusion::execution::options::ArrowReadOptions`

```rust
struct ArrowReadOptions<'a>
```

**Fields**: `schema`, `file_extension`, `table_partition_cols`

**Implements**: `datafusion::datasource::file_format::options::ReadOptions`

**Derives**: Clone, Default

**Methods** (2)

```rust
fn schema(self, schema: &'a Schema) -> Self
fn table_partition_cols(self, table_partition_cols: Vec<(String, DataType)>) -> Self
```

**via `datafusion::datasource::file_format::options::ReadOptions`**

```rust
async fn get_resolved_schema(&self, config: &SessionConfig, state: SessionState, table_path: ListingTableUrl) -> Result<SchemaRef>
fn schema_source(&self) -> SchemaSource
fn to_listing_options(&self, _config: &SessionConfig, _table_options: TableOptions) -> ListingOptions
```

[Full member, field, variant and typed contracts](../operations/datafusion.datasource.file_format.options.ArrowReadOptions.md).


Options that control the reading of ARROW files.

Note this structure is supplied when a datasource is created and
can not not vary from statement to statement. For settings that
can vary statement to statement see
[`ConfigOptions`](crate::config::ConfigOptions).

---

## AvroReadOptions

`struct` · `datafusion::datasource::file_format::options::AvroReadOptions`

Also reachable as `datafusion::execution::options::AvroReadOptions`, `datafusion::prelude::AvroReadOptions`

```rust
struct AvroReadOptions<'a>
```

**Fields**: `schema`, `file_extension`, `table_partition_cols`

**Implements**: `datafusion::datasource::file_format::options::ReadOptions`

**Derives**: Clone, Default

**Methods** (2)

```rust
fn schema(self, schema: &'a Schema) -> Self
fn table_partition_cols(self, table_partition_cols: Vec<(String, DataType)>) -> Self
```

**via `datafusion::datasource::file_format::options::ReadOptions`**

```rust
async fn get_resolved_schema(&self, config: &SessionConfig, state: SessionState, table_path: ListingTableUrl) -> Result<SchemaRef>
fn schema_source(&self) -> SchemaSource
fn to_listing_options(&self, _config: &SessionConfig, _table_options: TableOptions) -> ListingOptions
```

[Full member, field, variant and typed contracts](../operations/datafusion.datasource.file_format.options.AvroReadOptions.md).


Options that control the reading of AVRO files.

Note this structure is supplied when a datasource is created and
can not not vary from statement to statement. For settings that
can vary statement to statement see
[`ConfigOptions`](crate::config::ConfigOptions).

---

## CsvReadOptions

`struct` · `datafusion::datasource::file_format::options::CsvReadOptions`

Also reachable as `datafusion::execution::options::CsvReadOptions`, `datafusion::prelude::CsvReadOptions`

```rust
struct CsvReadOptions<'a>
```

**Fields**: `has_header`, `delimiter`, `quote`, `terminator`, `escape`, `comment`, `newlines_in_values`, `schema`, `schema_infer_max_records`, `file_extension`, `table_partition_cols`, `file_compression_type`, `file_sort_order`, `null_regex`, `truncated_rows`

**Implements**: `datafusion::datasource::file_format::options::ReadOptions`

**Derives**: Clone, Default

**Methods** (17)

```rust
fn comment(self, comment: u8) -> Self
fn delimiter(self, delimiter: u8) -> Self
fn delimiter_option(self, delimiter: Option<u8>) -> Self
fn escape(self, escape: u8) -> Self
fn file_compression_type(self, file_compression_type: FileCompressionType) -> Self
fn file_extension(self, file_extension: &'a str) -> Self
fn file_sort_order(self, file_sort_order: Vec<Vec<SortExpr>>) -> Self
fn has_header(self, has_header: bool) -> Self
fn new() -> Self
fn newlines_in_values(self, newlines_in_values: bool) -> Self
fn null_regex(self, null_regex: Option<String>) -> Self
fn quote(self, quote: u8) -> Self
fn schema(self, schema: &'a Schema) -> Self
fn schema_infer_max_records(self, max_records: usize) -> Self
fn table_partition_cols(self, table_partition_cols: Vec<(String, DataType)>) -> Self
fn terminator(self, terminator: Option<u8>) -> Self
fn truncated_rows(self, truncated_rows: bool) -> Self
```

**via `datafusion::datasource::file_format::options::ReadOptions`**

```rust
async fn get_resolved_schema(&self, config: &SessionConfig, state: SessionState, table_path: ListingTableUrl) -> Result<SchemaRef>
fn schema_source(&self) -> SchemaSource
fn to_listing_options(&self, _config: &SessionConfig, table_options: TableOptions) -> ListingOptions
```

[Full member, field, variant and typed contracts](../operations/datafusion.datasource.file_format.options.CsvReadOptions.md).


Options that control the reading of CSV files.

Note this structure is supplied when a datasource is created and
can not not vary from statement to statement. For settings that
can vary statement to statement see
[`ConfigOptions`](crate::config::ConfigOptions).

---

## JsonReadOptions

`struct` · `datafusion::datasource::file_format::options::JsonReadOptions`

Also reachable as `datafusion::execution::options::JsonReadOptions`, `datafusion::prelude::JsonReadOptions`

```rust
struct JsonReadOptions<'a>
```

**Fields**: `schema`, `schema_infer_max_records`, `file_extension`, `table_partition_cols`, `file_compression_type`, `infinite`, `file_sort_order`, `newline_delimited`

**Implements**: `datafusion::datasource::file_format::options::ReadOptions`

**Derives**: Clone, Default

**Methods** (8)

```rust
fn file_compression_type(self, file_compression_type: FileCompressionType) -> Self
fn file_extension(self, file_extension: &'a str) -> Self
fn file_sort_order(self, file_sort_order: Vec<Vec<SortExpr>>) -> Self
fn mark_infinite(self, infinite: bool) -> Self
fn newline_delimited(self, newline_delimited: bool) -> Self
fn schema(self, schema: &'a Schema) -> Self
fn schema_infer_max_records(self, schema_infer_max_records: usize) -> Self
fn table_partition_cols(self, table_partition_cols: Vec<(String, DataType)>) -> Self
```

**via `datafusion::datasource::file_format::options::ReadOptions`**

```rust
async fn get_resolved_schema(&self, config: &SessionConfig, state: SessionState, table_path: ListingTableUrl) -> Result<SchemaRef>
fn schema_source(&self) -> SchemaSource
fn to_listing_options(&self, _config: &SessionConfig, table_options: TableOptions) -> ListingOptions
```

[Full member, field, variant and typed contracts](../operations/datafusion.datasource.file_format.options.JsonReadOptions.md).


Options that control the reading of JSON files.

Supports both newline-delimited JSON (NDJSON) and JSON array formats.

Note this structure is supplied when a datasource is created and
can not vary from statement to statement. For settings that
can vary statement to statement see
[`ConfigOptions`](crate::config::ConfigOptions).

---

## ParquetReadOptions

`struct` · `datafusion::datasource::file_format::options::ParquetReadOptions`

Also reachable as `datafusion::execution::options::ParquetReadOptions`, `datafusion::prelude::ParquetReadOptions`

```rust
struct ParquetReadOptions<'a>
```

**Fields**: `file_extension`, `table_partition_cols`, `parquet_pruning`, `skip_metadata`, `schema`, `file_sort_order`, `file_decryption_properties`, `metadata_size_hint`

**Implements**: `datafusion::datasource::file_format::options::ReadOptions`

**Derives**: Clone, Default

**Methods** (9)

```rust
fn file_decryption_properties(self, file_decryption_properties: ConfigFileDecryptionProperties) -> Self
fn file_extension(self, file_extension: &'a str) -> Self
fn file_sort_order(self, file_sort_order: Vec<Vec<SortExpr>>) -> Self
fn metadata_size_hint(self, size_hint: Option<usize>) -> Self
fn new() -> Self
fn parquet_pruning(self, parquet_pruning: bool) -> Self
fn schema(self, schema: &'a Schema) -> Self
fn skip_metadata(self, skip_metadata: bool) -> Self
fn table_partition_cols(self, table_partition_cols: Vec<(String, DataType)>) -> Self
```

**via `datafusion::datasource::file_format::options::ReadOptions`**

```rust
async fn get_resolved_schema(&self, config: &SessionConfig, state: SessionState, table_path: ListingTableUrl) -> Result<SchemaRef>
fn schema_source(&self) -> SchemaSource
fn to_listing_options(&self, _config: &SessionConfig, table_options: TableOptions) -> ListingOptions
```

[Full member, field, variant and typed contracts](../operations/datafusion.datasource.file_format.options.ParquetReadOptions.md).


Options that control the reading of Parquet files.

Note this structure is supplied when a datasource is created and
can not not vary from statement to statement. For settings that
can vary statement to statement see
[`ConfigOptions`](crate::config::ConfigOptions).

---

## ReadOptions

`trait` · `datafusion::datasource::file_format::options::ReadOptions`

Also reachable as `datafusion::execution::options::ReadOptions`

```rust
trait ReadOptions<'a>
```

**Implementors** (5)

- `datafusion::datasource::file_format::options::ArrowReadOptions`
- `datafusion::datasource::file_format::options::AvroReadOptions`
- `datafusion::datasource::file_format::options::CsvReadOptions`
- `datafusion::datasource::file_format::options::JsonReadOptions`
- `datafusion::datasource::file_format::options::ParquetReadOptions`

**Methods** (4)

```rust
async fn _get_resolved_schema(&'a self, config: &SessionConfig, state: SessionState, table_path: ListingTableUrl, schema: Option<&'a Schema>) -> Result<SchemaRef>
async fn get_resolved_schema(&self, config: &SessionConfig, state: SessionState, table_path: ListingTableUrl) -> Result<SchemaRef>
fn schema_source(&self) -> SchemaSource
fn to_listing_options(&self, config: &SessionConfig, table_options: TableOptions) -> ListingOptions
```

[Full member, field, variant and typed contracts](../operations/datafusion.datasource.file_format.options.ReadOptions.md).


['ReadOptions'] is implemented by Options like ['CsvReadOptions'] that control the reading of respective files/sources.

---

## NdJsonReadOptions

`type_alias` · `datafusion::datasource::file_format::options::NdJsonReadOptions`

> **Deprecated** — since 53.0.0: Use `JsonReadOptions` instead. This alias will be removed in a future version.

Also reachable as `datafusion::execution::options::NdJsonReadOptions`

```rust
type NdJsonReadOptions<'a> = JsonReadOptions<'a>
```

[Full member, field, variant and typed contracts](../operations/datafusion.datasource.file_format.options.NdJsonReadOptions.md).


Deprecated: Use [`JsonReadOptions`] instead.

---
