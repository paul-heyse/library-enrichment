# `deltalake_core::operations::get_num_idx_cols_and_stats_columns`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.get_num_idx_cols_and_stats_columns.json).

<a id="op-edba6a1a89ad656fb072231d"></a>
## get_num_idx_cols_and_stats_columns

`function` · `deltalake_core::operations::get_num_idx_cols_and_stats_columns` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_num_idx_cols_and_stats_columns(config: Option<&delta_kernel::table_properties::TableProperties>, configuration: std::collections::HashMap<String, Option<String>>) -> (delta_kernel::table_properties::DataSkippingNumIndexedCols, Option<Vec<String>>)
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/mod.rs#L323).

Source: `crates/core/src/operations/mod.rs:323`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get the num_idx_columns and stats_columns from the table configuration in the state
If table_config does not exist (only can occur in the first write action) it takes
the configuration that was passed to the writerBuilder.
