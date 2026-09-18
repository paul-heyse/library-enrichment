# `datafusion_datasource::file_scan_config::wrap_partition_type_in_dict`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.file_scan_config.wrap_partition_type_in_dict.json).

<a id="op-c038d86df29ae6b5ad984102"></a>
## wrap_partition_type_in_dict

`function` · `datafusion_datasource::file_scan_config::wrap_partition_type_in_dict` · datafusion-datasource 55.1.0

```rust
fn wrap_partition_type_in_dict(val_type: arrow::datatypes::DataType) -> arrow::datatypes::DataType
```

Source: `src/file_scan_config/mod.rs:1572`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Convert type to a type suitable for use as a `ListingTable`
partition column. Returns `Dictionary(UInt16, val_type)`, which is
a reasonable trade off between a reasonable number of partition
values and space efficiency.

This use this to specify types for partition columns. However
you MAY also choose not to dictionary-encode the data or to use a
different dictionary type.

Use [`wrap_partition_value_in_dict`](../operations/datafusion_datasource.file_scan_config.wrap_partition_value_in_dict.md#op-7415a922f3047bcec48460d7) to wrap a [`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b) in the same say.
