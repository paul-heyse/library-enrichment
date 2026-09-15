# `datafusion_datasource::file_format`

Crate `datafusion-datasource` · 7 public items · structured records in [`model/datafusion_datasource.file_format.json`](../model/datafusion_datasource.file_format.json)

## DEFAULT_SCHEMA_INFER_MAX_RECORD

`constant` · `datafusion_datasource::file_format::DEFAULT_SCHEMA_INFER_MAX_RECORD`

Also reachable as `datafusion::datasource::file_format::DEFAULT_SCHEMA_INFER_MAX_RECORD`

```rust
const DEFAULT_SCHEMA_INFER_MAX_RECORD: usize = 1000
```

Default max records to scan to infer the schema

---

## file_type_to_format

`function` · `datafusion_datasource::file_format::file_type_to_format`

Also reachable as `datafusion::datasource::file_format::file_type_to_format`

```rust
fn file_type_to_format(file_type: &std::sync::Arc<dyn FileType>) -> datafusion_common::Result<std::sync::Arc<dyn FileFormatFactory>>
```

Converts a [FileType] to a [FileFormatFactory].
Returns an error if the [FileType] cannot be
downcasted to a [DefaultFileType].

---

## format_as_file_type

`function` · `datafusion_datasource::file_format::format_as_file_type`

Also reachable as `datafusion::datasource::file_format::format_as_file_type`

```rust
fn format_as_file_type(file_format_factory: std::sync::Arc<dyn FileFormatFactory>) -> std::sync::Arc<dyn FileType>
```

Converts a [FileFormatFactory] to a [FileType]

---

## DefaultFileType

`struct` · `datafusion_datasource::file_format::DefaultFileType`

Also reachable as `datafusion::datasource::file_format::DefaultFileType`

```rust
struct DefaultFileType
```

**Implements**: `core::fmt::Display`, `datafusion_common::file_options::file_type::FileType`, `datafusion_common::file_options::file_type::GetExt`

**Derives**: Debug

**Methods** (2)

```rust
fn as_format_factory(&self) -> &Arc<dyn FileFormatFactory>
fn new(file_format_factory: Arc<dyn FileFormatFactory>) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `datafusion_common::file_options::file_type::FileType`**

```rust
fn as_any(&self) -> &dyn Any
```

**via `datafusion_common::file_options::file_type::GetExt`**

```rust
fn get_ext(&self) -> String
```

A container of [FileFormatFactory] which also implements [FileType].
This enables converting a dyn FileFormat to a dyn FileType.
The former trait is a superset of the latter trait, which includes execution time
relevant methods. [FileType] is only used in logical planning and only implements
the subset of methods required during logical planning.

---

## FileMeta

`struct` · `datafusion_datasource::file_format::FileMeta`

Also reachable as `datafusion::datasource::file_format::FileMeta`

```rust
struct FileMeta
```

**Fields**: `statistics`, `ordering`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn new(statistics: Statistics) -> Self
fn with_ordering(self, ordering: Option<LexOrdering>) -> Self
```

Metadata fetched from a file, including statistics and ordering.

This struct is returned by [`FileFormat::infer_stats_and_ordering`] to
provide all metadata in a single read, avoiding duplicate I/O operations.

---

## FileFormat

`trait` · `datafusion_datasource::file_format::FileFormat`

Also reachable as `datafusion::datasource::file_format::FileFormat`

```rust
trait FileFormat: Any + Send + Sync + fmt::Debug
```

**Implementors** (5)

- `datafusion_datasource_arrow::file_format::ArrowFormat`
- `datafusion_datasource_avro::file_format::AvroFormat`
- `datafusion_datasource_csv::file_format::CsvFormat`
- `datafusion_datasource_json::file_format::JsonFormat`
- `datafusion_datasource_parquet::file_format::ParquetFormat`

**Methods** (10)

```rust
fn compression_type(&self) -> Option<FileCompressionType>
async fn create_physical_plan(&self, state: &dyn Session, conf: FileScanConfig) -> Result<Arc<dyn ExecutionPlan>>
async fn create_writer_physical_plan(&self, _input: Arc<dyn ExecutionPlan>, _state: &dyn Session, _conf: FileSinkConfig, _order_requirements: Option<LexRequirement>) -> Result<Arc<dyn ExecutionPlan>>
fn file_source(&self, table_schema: TableSchema) -> Arc<dyn FileSource>
fn get_ext(&self) -> String
fn get_ext_with_compression(&self, _file_compression_type: &FileCompressionType) -> Result<String>
async fn infer_ordering(&self, _state: &dyn Session, _store: &Arc<dyn ObjectStore>, _table_schema: SchemaRef, _object: &ObjectMeta) -> Result<Option<LexOrdering>>
async fn infer_schema(&self, state: &dyn Session, store: &Arc<dyn ObjectStore>, objects: &[ObjectMeta]) -> Result<SchemaRef>
async fn infer_stats(&self, state: &dyn Session, store: &Arc<dyn ObjectStore>, table_schema: SchemaRef, object: &ObjectMeta) -> Result<Statistics>
async fn infer_stats_and_ordering(&self, state: &dyn Session, store: &Arc<dyn ObjectStore>, table_schema: SchemaRef, object: &ObjectMeta) -> Result<FileMeta>
```

This trait abstracts all the file format specific implementations
from the [`TableProvider`]. This helps code re-utilization across
providers that support the same file formats.

[`TableProvider`]: https://docs.rs/datafusion/latest/datafusion/catalog/trait.TableProvider.html

---

## FileFormatFactory

`trait` · `datafusion_datasource::file_format::FileFormatFactory`

Also reachable as `datafusion::datasource::file_format::FileFormatFactory`

```rust
trait FileFormatFactory: Any + Sync + Send + GetExt + fmt::Debug
```

**Implementors** (5)

- `datafusion_datasource_arrow::file_format::ArrowFormatFactory`
- `datafusion_datasource_avro::file_format::AvroFormatFactory`
- `datafusion_datasource_csv::file_format::CsvFormatFactory`
- `datafusion_datasource_json::file_format::JsonFormatFactory`
- `datafusion_datasource_parquet::file_format::ParquetFormatFactory`

**Methods** (2)

```rust
fn create(&self, state: &dyn Session, format_options: &HashMap<String, String>) -> Result<Arc<dyn FileFormat>>
fn default(&self) -> Arc<dyn FileFormat>
```

Factory for creating [`FileFormat`] instances based on session and command level options

Users can provide their own `FileFormatFactory` to support arbitrary file formats

---
