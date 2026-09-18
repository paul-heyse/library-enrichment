# `deltalake_core::operations::optimize`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.optimize.json).

<a id="op-390ed54ef4f5f3b2f04cd570"></a>
## optimize

`module` · `deltalake_core::operations::optimize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod optimize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L1).

Source: `crates/core/src/operations/optimize.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Optimize a Delta Table

Perform bin-packing on a Delta Table which merges small files into a large
file. Bin-packing reduces the number of API calls required for read
operations.

Optimize will fail if a concurrent write operation removes files from the
table (such as in an overwrite). It will always succeed if concurrent writers
are only appending.

Optimize increments the table's version and creates remove actions for
optimized files. Optimize does not delete files from storage. To delete
files that were removed, call `vacuum` on [`DeltaTable`](../operations/deltalake_core.table.DeltaTable.md#op-2732e9061346704f1c6a0875).

See [`OptimizeBuilder`](../operations/deltalake_core.operations.optimize.OptimizeBuilder.md#op-b5ca9f882a7416ed60fd1d9e) for configuration.

# Example
```rust ignore
let table = open_table(Url::from_directory_path("/abs/path/to/table").unwrap())?;
let (table, metrics) = OptimizeBuilder::new(table.object_store(), table.state).await?;
````
