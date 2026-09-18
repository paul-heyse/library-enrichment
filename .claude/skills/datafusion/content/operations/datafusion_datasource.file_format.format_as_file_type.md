# `datafusion_datasource::file_format::format_as_file_type`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.file_format.format_as_file_type.json).

<a id="op-c794024940b3236615a5dd5e"></a>
## format_as_file_type

`function` · `datafusion_datasource::file_format::format_as_file_type` · datafusion-datasource 55.1.0

```rust
fn format_as_file_type(file_format_factory: std::sync::Arc<dyn FileFormatFactory>) -> std::sync::Arc<dyn FileType>
```

Source: `src/file_format.rs:270`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Converts a [FileFormatFactory](../operations/datafusion_datasource.file_format.FileFormatFactory.md#op-f967009d6b2c0c52e0070973) to a [FileType](../operations/datafusion_common.file_options.file_type.FileType.md#op-a3f73eb11aa44031af77da32)
