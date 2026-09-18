# `datafusion_datasource_arrow::source`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_arrow.source.json).

<a id="op-e63d3277fc5699c5e7063c9a"></a>
## source

`module` · `datafusion_datasource_arrow::source` · datafusion-datasource-arrow 55.1.0

```rust
mod source
```

Source: `src/source.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

Execution plan for reading Arrow IPC files

# Naming Note

The naming in this module can be confusing:
- `ArrowFileOpener` handles the Arrow IPC **file format**
  (with footer, supports parallel reading)
- `ArrowStreamFileOpener` handles the Arrow IPC **stream format**
  (without footer, sequential only)
- `ArrowSource` is the unified `FileSource` implementation that uses either opener
  depending on the format specified at construction

Despite the name "ArrowStreamFileOpener", it still reads from files - the "Stream"
refers to the Arrow IPC stream format, not streaming I/O. Both formats can be stored
in files on disk or object storage.
