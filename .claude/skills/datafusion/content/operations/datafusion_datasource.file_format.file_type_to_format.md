# `datafusion_datasource::file_format::file_type_to_format`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.file_format.file_type_to_format.json).

<a id="op-33425a42a255df9937d0bc57"></a>
## file_type_to_format

`function` · `datafusion_datasource::file_format::file_type_to_format` · datafusion-datasource 55.1.0

```rust
fn file_type_to_format(file_type: &std::sync::Arc<dyn FileType>) -> datafusion_common::Result<std::sync::Arc<dyn FileFormatFactory>>
```

Source: `src/file_format.rs:281`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Converts a [FileType](../operations/datafusion_common.file_options.file_type.FileType.md#op-a3f73eb11aa44031af77da32) to a [FileFormatFactory](../operations/datafusion_datasource.file_format.FileFormatFactory.md#op-f967009d6b2c0c52e0070973).
Returns an error if the [FileType](../operations/datafusion_common.file_options.file_type.FileType.md#op-a3f73eb11aa44031af77da32) cannot be
downcasted to a [DefaultFileType](../operations/datafusion_datasource.file_format.DefaultFileType.md#op-266cf80407fe6bc321faa948).
