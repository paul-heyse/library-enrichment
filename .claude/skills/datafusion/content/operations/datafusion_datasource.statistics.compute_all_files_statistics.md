# `datafusion_datasource::statistics::compute_all_files_statistics`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.statistics.compute_all_files_statistics.json).

<a id="op-ae7d9c08572c005acb159b26"></a>
## compute_all_files_statistics

`function` · `datafusion_datasource::statistics::compute_all_files_statistics` · datafusion-datasource 55.1.0

```rust
fn compute_all_files_statistics(file_groups: Vec<file_groups::FileGroup>, table_schema: arrow::datatypes::SchemaRef, collect_stats: bool, inexact_stats: bool) -> datafusion_common::Result<(Vec<file_groups::FileGroup>, datafusion_physical_plan::Statistics)>
```

Source: `src/statistics.rs:517`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Computes statistics for all files across multiple file groups.

This function:
1. Computes statistics for each individual file group
2. Summary statistics across all file groups
3. Optionally marks statistics as inexact

# Parameters
* `file_groups` - Vector of file groups to process
* `table_schema` - Schema of the table
* `collect_stats` - Whether to collect statistics
* `inexact_stats` - Whether to mark the resulting statistics as inexact

# Returns
A tuple containing:
* The processed file groups with their individual statistics attached
* The summary statistics across all file groups, aka all files summary statistics
