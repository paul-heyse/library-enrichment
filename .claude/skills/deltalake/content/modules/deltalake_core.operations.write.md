# `deltalake_core::operations::write`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.write.json).

<a id="op-efa0272433847fd2a898b0c6"></a>
## write

`module` · `deltalake_core::operations::write` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod write
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L1).

Source: `crates/core/src/operations/write/mod.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).


New Table Semantics
 - The schema of the [Plan] is used to initialize the table.
 - The partition columns will be used to partition the table.

Existing Table Semantics
 - The save mode will control how existing data is handled (i.e. overwrite, append, etc)
 - Conflicting columns (i.e. a INT, and a STRING)
   will result in an exception.
   Partition columns, if present, are validated against the existing metadata.
   When omitted, the table partitioning is respected.
   Full table overwrite with `SchemaMode::Overwrite` and no replaceWhere predicate may
   replace the partition columns.

In combination with `Overwrite`, a `replaceWhere` option can be used to transactionally
replace data that matches a predicate.

# Example
```rust ignore
let id_field = arrow::datatypes::Field::new("id", arrow::datatypes::DataType::Int32, false);
let schema = Arc::new(arrow::datatypes::Schema::new(vec![id_field]));
let ids = arrow::array::Int32Array::from(vec![1, 2, 3, 4, 5]);
let batch = RecordBatch::try_new(schema, vec![Arc::new(ids)])?;
let table = DeltaTableBuilder::from_url("../path/to/empty/dir").unwrap().build()?;
let table = table.write(vec![batch]).await?;
````
