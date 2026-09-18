# `deltalake_core::operations::delete`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.delete.json).

<a id="op-726912fcdfbbab0eaaac252b"></a>
## delete

`module` · `deltalake_core::operations::delete` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod delete
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L1).

Source: `crates/core/src/operations/delete.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Delete records from a Delta Table that satisfy a predicate

When a predicate is not provided then all records are deleted from the Delta
Table. Otherwise a scan of the Delta table is performed to mark any files
that contain records that satisfy the predicate. Once files are determined
they are rewritten without the records.

`DeleteMetrics::num_deleted_rows` is optional. Row rewrite deletes derive the
count from execution metrics, while metadata only full file deletes return
`None` when this library cannot derive the count from file metadata.

Predicates MUST be deterministic otherwise undefined behaviour may occur during the
scanning and rewriting phase.

# Example
```
# use datafusion::logical_expr::{col, lit};
# use deltalake_core::{DeltaTable, kernel::{DataType, PrimitiveType, StructType, StructField}};
# use deltalake_core::operations::delete::DeleteBuilder;
# tokio_test::block_on(async {
#  let schema = StructType::try_new(vec![
#      StructField::new(
#          "id".to_string(),
#          DataType::Primitive(PrimitiveType::String),
#          true,
#      )]).expect("Failed to generate schema for test");
# let table = DeltaTable::try_from_url(url::Url::parse("memory://").unwrap())
#               .await.expect("Failed to construct DeltaTable instance for test")
#        .create()
#        .with_columns(schema.fields().cloned())
#        .await
#        .expect("Failed to create test table");
let (table, metrics) = table.delete()
    .with_predicate(col("id").eq(lit(102)))
    .await
    .expect("Failed to delete");
# })
````
