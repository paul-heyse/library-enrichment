# `datafusion_datasource::write::get_writer_schema`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.write.get_writer_schema.json).

<a id="op-2e79253c6780256c24d1eb66"></a>
## get_writer_schema

`function` · `datafusion_datasource::write::get_writer_schema` · datafusion-datasource 55.1.0

```rust
fn get_writer_schema(config: &file_sink_config::FileSinkConfig) -> std::sync::Arc<arrow::datatypes::Schema>
```

Source: `src/write/mod.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Converts table schema to writer schema, which may differ in the case
of hive style partitioning where some columns are removed from the
underlying files.
