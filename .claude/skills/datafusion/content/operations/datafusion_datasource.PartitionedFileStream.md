# `datafusion_datasource::PartitionedFileStream`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.PartitionedFileStream.json).

<a id="op-a73d7b15ce6643074016b215"></a>
## PartitionedFileStream

`type_alias` · `datafusion_datasource::PartitionedFileStream` · datafusion-datasource 55.1.0

```rust
type PartitionedFileStream = std::pin::Pin<Box<dyn Stream<Item = datafusion_common::Result<PartitionedFile>> + Send + Sync + 'static>>
```

Source: `src/mod.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Stream of files get listed from object store
