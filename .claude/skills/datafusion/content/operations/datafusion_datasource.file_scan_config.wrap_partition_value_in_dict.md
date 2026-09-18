# `datafusion_datasource::file_scan_config::wrap_partition_value_in_dict`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.file_scan_config.wrap_partition_value_in_dict.json).

<a id="op-7415a922f3047bcec48460d7"></a>
## wrap_partition_value_in_dict

`function` · `datafusion_datasource::file_scan_config::wrap_partition_value_in_dict` · datafusion-datasource 55.1.0

```rust
fn wrap_partition_value_in_dict(val: datafusion_common::ScalarValue) -> datafusion_common::ScalarValue
```

Source: `src/file_scan_config/mod.rs:1579`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Convert a [`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b) of partition columns to a type, as
described in the documentation of [`wrap_partition_type_in_dict`](../operations/datafusion_datasource.file_scan_config.wrap_partition_type_in_dict.md#op-c038d86df29ae6b5ad984102),
which can wrap the types.
