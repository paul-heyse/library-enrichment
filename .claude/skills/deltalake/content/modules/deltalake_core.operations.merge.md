# `deltalake_core::operations::merge`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.merge.json).

<a id="op-42b70c67b0cc9771588e5e77"></a>
## merge

`module` · `deltalake_core::operations::merge` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod merge
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L1).

Source: `crates/core/src/operations/merge/mod.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Merge data from a source dataset with the target Delta Table based on a join
predicate.  A full outer join is performed which results in source and
target records that match, source records that do not match, or target
records that do not match.

Users can specify update, delete, and insert operations for these categories
and specify additional predicates for finer control. The order of operations
specified matter.  See [`MergeBuilder`](../operations/deltalake_core.operations.merge.MergeBuilder.md#op-178a56da87bb85e396423d0b) for more information

# Example
```rust ignore
let table = open_table(Url::from_directory_path("/abs/path/to/table").unwrap())?;
let (table, metrics) = table
    .merge(source, col("target.id").eq(col("source.id")))
    .with_source_alias("source")
    .with_target_alias("target")
    .when_matched_update(|update| {
        update
            .update("value", col("source.value") + lit(1))
            .update("modified", col("source.modified"))
    })?
    .when_not_matched_insert(|insert| {
        insert
            .set("id", col("source.id"))
            .set("value", col("source.value"))
            .set("modified", col("source.modified"))
    })?
    .await?
````
