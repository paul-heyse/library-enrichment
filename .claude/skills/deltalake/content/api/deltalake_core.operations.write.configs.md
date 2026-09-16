# `deltalake_core::operations::write::configs`

Crate `deltalake-core` · 1 public items · structured records in [`model/deltalake_core.operations.write.configs.json`](../model/deltalake_core.operations.write.configs.json)

## WriterStatsConfig

`struct` · `deltalake_core::operations::write::configs::WriterStatsConfig`

Also reachable as `deltalake::operations::write::WriterStatsConfig`, `deltalake::operations::write::configs::WriterStatsConfig`, `deltalake_core::operations::write::WriterStatsConfig`

```rust
struct WriterStatsConfig
```

**Fields**: `num_indexed_cols`, `stats_columns`

**Derives**: Clone

**Methods** (2)

```rust
fn from_config(config: &TableConfiguration) -> Self
fn new(num_indexed_cols: DataSkippingNumIndexedCols, stats_columns: Option<Vec<String>>) -> Self
```

Configuration for the writer on how to collect stats

---
