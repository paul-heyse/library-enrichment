# `deltalake_core::operations::update`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.update.json).

<a id="op-d0e931e671e3aa1a3674d515"></a>
## update

`module` · `deltalake_core::operations::update` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod update
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update.rs#L1).

Source: `crates/core/src/operations/update.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Update records from a Delta Table for records satisfy a predicate

When a predicate is not provided then all records are updated from the Delta
Table. Otherwise a scan of the Delta table is performed to mark any files
that contain records that satisfy the predicate. Once they are determined
then column values are updated with new values provided by the user


Predicates MUST be deterministic otherwise undefined behaviour may occur during the
scanning and rewriting phase.

# Example
```rust ignore
let table = open_table(Url::from_directory_path("/abs/path/to/table").unwrap())?;
let (table, metrics) = UpdateBuilder::new(table.object_store(), table.state)
    .with_predicate(col("col1").eq(lit(1)))
    .with_update("value", col("value") + lit(20))
    .await?;
````
