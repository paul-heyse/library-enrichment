# `datafusion_datasource::file_stream`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.file_stream.json).

<a id="op-85ae9045ce29632e70ea7f15"></a>
## file_stream

`module` · `datafusion_datasource::file_stream` · datafusion-datasource 55.1.0

```rust
mod file_stream
```

Source: `src/file_stream/mod.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

A generic stream over file format readers that can be used by
any file format that read its files from start to end.

Note: Most traits here need to be marked `Sync + Send` to be
compliant with the `SendableRecordBatchStream` trait.
