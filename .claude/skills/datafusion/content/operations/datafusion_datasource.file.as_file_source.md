# `datafusion_datasource::file::as_file_source`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.file.as_file_source.json).

<a id="op-9ea29946de7425848fd99f9a"></a>
## as_file_source

`function` · `datafusion_datasource::file::as_file_source` · datafusion-datasource 55.1.0

```rust
fn as_file_source<T: FileSource + 'static>(source: T) -> std::sync::Arc<dyn FileSource>
```

Source: `src/file.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Helper function to convert any type implementing [`FileSource`](../operations/datafusion_datasource.file.FileSource.md#op-8c1b19f80ea0c73466216a63) to `Arc<dyn FileSource>`
