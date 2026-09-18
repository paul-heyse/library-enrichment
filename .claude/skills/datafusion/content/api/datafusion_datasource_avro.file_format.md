# `datafusion_datasource_avro::file_format`

Crate `datafusion-datasource-avro` · 2 public items · structured records in [`model/datafusion_datasource_avro.file_format.json`](../model/datafusion_datasource_avro.file_format.json)

## AvroFormat

`struct` · `datafusion_datasource_avro::file_format::AvroFormat`

Also reachable as `datafusion::datasource::file_format::avro::AvroFormat`, `datafusion_datasource_avro::AvroFormat`

```rust
struct AvroFormat
```

**Implements**: `datafusion_datasource::file_format::FileFormat`

**Derives**: Debug, Default

**via `datafusion_datasource::file_format::FileFormat`**

```rust
fn compression_type(&self) -> Option<FileCompressionType>
async fn create_physical_plan(&self, _state: &dyn Session, conf: FileScanConfig) -> Result<Arc<dyn ExecutionPlan>>
fn file_source(&self, table_schema: datafusion_datasource::TableSchema) -> Arc<dyn FileSource>
fn get_ext(&self) -> String
fn get_ext_with_compression(&self, file_compression_type: &FileCompressionType) -> Result<String>
async fn infer_schema(&self, _state: &dyn Session, store: &Arc<dyn ObjectStore>, objects: &[ObjectMeta]) -> Result<SchemaRef>
async fn infer_stats(&self, _state: &dyn Session, _store: &Arc<dyn ObjectStore>, table_schema: SchemaRef, _object: &ObjectMeta) -> Result<Statistics>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource_avro.file_format.AvroFormat.md).


Avro [`FileFormat`] implementation.

---

## AvroFormatFactory

`struct` · `datafusion_datasource_avro::file_format::AvroFormatFactory`

Also reachable as `datafusion::datasource::file_format::avro::AvroFormatFactory`, `datafusion_datasource_avro::AvroFormatFactory`

```rust
struct AvroFormatFactory
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

[Full member, field, variant and typed contracts](../operations/datafusion_datasource_avro.file_format.AvroFormatFactory.md).


Factory struct used to create [`AvroFormat`]

---
