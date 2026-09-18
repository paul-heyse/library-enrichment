# `datafusion_physical_plan::common::build_checked_file_list`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.common.build_checked_file_list.json).

<a id="op-f2e610d203f72bd97d584cd2"></a>
## build_checked_file_list

`function` · `datafusion_physical_plan::common::build_checked_file_list` · datafusion-physical-plan 55.1.0

```rust
fn build_checked_file_list(dir: &str, ext: &str) -> datafusion_common::Result<Vec<String>>
```

Source: `src/common.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Recursively builds a list of files in a directory with a given extension
