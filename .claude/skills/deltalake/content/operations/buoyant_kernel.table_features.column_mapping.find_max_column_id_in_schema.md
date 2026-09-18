# `buoyant_kernel::table_features::column_mapping::find_max_column_id_in_schema`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.table_features.column_mapping.find_max_column_id_in_schema.json).

<a id="op-ca539fe0c0bd760ec501c0bf"></a>
## find_max_column_id_in_schema

`function` · `buoyant_kernel::table_features::column_mapping::find_max_column_id_in_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn find_max_column_id_in_schema(schema: &schema::StructType) -> Option<i64>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_features/column_mapping.rs#L664).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_features/column_mapping.rs:664`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the largest column mapping id found anywhere in `schema`. This includes both
per-field `delta.columnMapping.id` annotations and the nested ids in
`delta.columnMapping.nested.ids` metadata.
