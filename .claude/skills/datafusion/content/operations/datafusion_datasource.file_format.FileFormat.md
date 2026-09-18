# `datafusion_datasource::file_format::FileFormat`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.file_format.FileFormat.json).

<a id="op-81da8109cfb5e6919dc4a7ee"></a>
## FileFormat

`trait` · `datafusion_datasource::file_format::FileFormat` · datafusion-datasource 55.1.0

```rust
trait FileFormat: Any + Send + Sync + fmt::Debug
```

Source: `src/file_format.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

This trait abstracts all the file format specific implementations
from the [`TableProvider`]. This helps code re-utilization across
providers that support the same file formats.

[`TableProvider`]: https://docs.rs/datafusion/latest/datafusion/catalog/trait.TableProvider.html

<a id="op-826a4915eab04fbfe418e6a0"></a>
## compression_type

`function` · `datafusion_datasource::file_format::FileFormat::compression_type` · datafusion-datasource 55.1.0

```rust
fn compression_type(&self) -> Option<FileCompressionType>
```

Source: `src/file_format.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Returns whether this instance uses compression if applicable

<a id="op-a219ad5e5530232068322923"></a>
## create_physical_plan

`function` · `datafusion_datasource::file_format::FileFormat::create_physical_plan` · datafusion-datasource 55.1.0

```rust
async fn create_physical_plan(&self, state: &dyn Session, conf: FileScanConfig) -> Result<Arc<dyn ExecutionPlan>>
```

Source: `src/file_format.rs:167`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Take a list of files and convert it to the appropriate executor
according to this file format.

<a id="op-328d8200c0d1b146d37f4ddd"></a>
## create_writer_physical_plan

`function` · `datafusion_datasource::file_format::FileFormat::create_writer_physical_plan` · datafusion-datasource 55.1.0

```rust
async fn create_writer_physical_plan(&self, _input: Arc<dyn ExecutionPlan>, _state: &dyn Session, _conf: FileSinkConfig, _order_requirements: Option<LexRequirement>) -> Result<Arc<dyn ExecutionPlan>>
```

Source: `src/file_format.rs:175`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Take a list of files and the configuration to convert it to the
appropriate writer executor according to this file format.

<a id="op-e7e77e03b9f5c576460ca3f0"></a>
## file_source

`function` · `datafusion_datasource::file_format::FileFormat::file_source` · datafusion-datasource 55.1.0

```rust
fn file_source(&self, table_schema: TableSchema) -> Arc<dyn FileSource>
```

Source: `src/file_format.rs:189`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Return the related FileSource such as `CsvSource`, `JsonSource`, etc.

# Arguments
* `table_schema` - The table schema to use for the FileSource (includes partition columns)

<a id="op-d0fc862c4d95ef9cf879ab1b"></a>
## get_ext

`function` · `datafusion_datasource::file_format::FileFormat::get_ext` · datafusion-datasource 55.1.0

```rust
fn get_ext(&self) -> String
```

Source: `src/file_format.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Returns the extension for this FileFormat, e.g. "file.csv" -> csv

<a id="op-c6dbed1d3c0394db3e80d807"></a>
## get_ext_with_compression

`function` · `datafusion_datasource::file_format::FileFormat::get_ext_with_compression` · datafusion-datasource 55.1.0

```rust
fn get_ext_with_compression(&self, _file_compression_type: &FileCompressionType) -> Result<String>
```

Source: `src/file_format.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Returns the extension for this FileFormat when compressed, e.g. "file.csv.gz" -> csv

<a id="op-33af6ca1dc82f2663fbc56b4"></a>
## infer_ordering

`function` · `datafusion_datasource::file_format::FileFormat::infer_ordering` · datafusion-datasource 55.1.0

```rust
async fn infer_ordering(&self, _state: &dyn Session, _store: &Arc<dyn ObjectStore>, _table_schema: SchemaRef, _object: &ObjectMeta) -> Result<Option<LexOrdering>>
```

Source: `src/file_format.rs:128`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Infer the ordering (sort order) for the provided object from file metadata.

Returns `Ok(None)` if the file format does not support ordering inference
or if the file does not have ordering information.

`table_schema` is the (combined) schema of the overall table
and may be a superset of the schema contained in this file.

The default implementation returns `Ok(None)`.

<a id="op-17e4cc8199e4c667431a735a"></a>
## infer_schema

`function` · `datafusion_datasource::file_format::FileFormat::infer_schema` · datafusion-datasource 55.1.0

```rust
async fn infer_schema(&self, state: &dyn Session, store: &Arc<dyn ObjectStore>, objects: &[ObjectMeta]) -> Result<SchemaRef>
```

Source: `src/file_format.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Infer the common schema of the provided objects. The objects will usually
be analysed up to a given number of records or files (as specified in the
format config) then give the estimated common schema. This might fail if
the files have schemas that cannot be merged.

<a id="op-aa3ccc6347245cad0ae8bf11"></a>
## infer_stats

`function` · `datafusion_datasource::file_format::FileFormat::infer_stats` · datafusion-datasource 55.1.0

```rust
async fn infer_stats(&self, state: &dyn Session, store: &Arc<dyn ObjectStore>, table_schema: SchemaRef, object: &ObjectMeta) -> Result<Statistics>
```

Source: `src/file_format.rs:111`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Infer the statistics for the provided object. The cost and accuracy of the
estimated statistics might vary greatly between file formats.

`table_schema` is the (combined) schema of the overall table
and may be a superset of the schema contained in this file.

TODO: should the file source return statistics for only columns referred to in the table schema?

<a id="op-a37e5aef3886096f673ca012"></a>
## infer_stats_and_ordering

`function` · `datafusion_datasource::file_format::FileFormat::infer_stats_and_ordering` · datafusion-datasource 55.1.0

```rust
async fn infer_stats_and_ordering(&self, state: &dyn Session, store: &Arc<dyn ObjectStore>, table_schema: SchemaRef, object: &ObjectMeta) -> Result<FileMeta>
```

Source: `src/file_format.rs:146`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Infer both statistics and ordering from a single metadata read.

This is more efficient than calling [`Self::infer_stats`](../operations/datafusion_datasource.file_format.FileFormat.md#op-aa3ccc6347245cad0ae8bf11) and
[`Self::infer_ordering`](../operations/datafusion_datasource.file_format.FileFormat.md#op-33af6ca1dc82f2663fbc56b4) separately when both are needed, as it avoids
reading file metadata twice.

The default implementation calls both methods separately. File formats
that can extract both from a single read should override this method.
