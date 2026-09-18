# `datafusion_datasource::memory::PartitionData`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.memory.PartitionData.json).

<a id="op-e7ecc20b3b576da6525fa6a9"></a>
## PartitionData

`type_alias` · `datafusion_datasource::memory::PartitionData` · datafusion-datasource 55.1.0

```rust
type PartitionData = std::sync::Arc<tokio::sync::RwLock<Vec<arrow::array::RecordBatch>>>
```

Source: `src/memory.rs:857`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Type alias for partition data
