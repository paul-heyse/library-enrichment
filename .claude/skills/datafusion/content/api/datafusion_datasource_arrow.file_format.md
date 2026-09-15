# `datafusion_datasource_arrow::file_format`

Crate `datafusion-datasource-arrow` · 2 public items · structured records in [`model/datafusion_datasource_arrow.file_format.json`](../model/datafusion_datasource_arrow.file_format.json)

## ArrowFormat

`struct` · `datafusion_datasource_arrow::file_format::ArrowFormat`

Also reachable as `datafusion::datasource::file_format::arrow::ArrowFormat`, `datafusion_datasource_arrow::ArrowFormat`

```rust
struct ArrowFormat
```

**Implements**: `datafusion_datasource::file_format::FileFormat`

**Derives**: Debug, Default

**via `datafusion_datasource::file_format::FileFormat`**

```rust
fn compression_type(&self) -> Option<FileCompressionType>
async fn create_physical_plan(&self, state: &dyn Session, conf: FileScanConfig) -> Result<Arc<dyn ExecutionPlan>>
async fn create_writer_physical_plan(&self, input: Arc<dyn ExecutionPlan>, _state: &dyn Session, conf: FileSinkConfig, order_requirements: Option<LexRequirement>) -> Result<Arc<dyn ExecutionPlan>>
fn file_source(&self, table_schema: TableSchema) -> Arc<dyn FileSource>
fn get_ext(&self) -> String
fn get_ext_with_compression(&self, file_compression_type: &FileCompressionType) -> Result<String>
async fn infer_schema(&self, _state: &dyn Session, store: &Arc<dyn ObjectStore>, objects: &[ObjectMeta]) -> Result<SchemaRef>
async fn infer_stats(&self, _state: &dyn Session, _store: &Arc<dyn ObjectStore>, table_schema: SchemaRef, _object: &ObjectMeta) -> Result<Statistics>
```

Arrow [`FileFormat`] implementation.

---

## ArrowFormatFactory

`struct` · `datafusion_datasource_arrow::file_format::ArrowFormatFactory`

Also reachable as `datafusion::datasource::file_format::arrow::ArrowFormatFactory`, `datafusion_datasource_arrow::ArrowFormatFactory`

```rust
struct ArrowFormatFactory
```

**Implements**: `datafusion_common::file_options::file_type::GetExt`, `datafusion_datasource::file_format::FileFormatFactory`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_common::file_options::file_type::GetExt`**

```rust
fn get_ext(&self) -> String
```

**via `datafusion_datasource::file_format::FileFormatFactory`**

```rust
fn create(&self, _state: &dyn Session, _format_options: &HashMap<String, String>) -> Result<Arc<dyn FileFormat>>
fn default(&self) -> Arc<dyn FileFormat>
```

Factory struct used to create [`ArrowFormat`]

---
