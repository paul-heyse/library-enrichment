# `buoyant_kernel::table_changes`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.table_changes.json).

<a id="op-24388d36d2a9d44b7be7c70a"></a>
## table_changes

`module` · `buoyant_kernel::table_changes` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod table_changes
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/mod.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/mod.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Provides an API to read the table's change data feed between two versions.

# Example
```rust
# use std::sync::Arc;
# use buoyant_kernel as delta_kernel;
# use test_utils::delta_kernel_default_engine::{DefaultEngine, DefaultEngineBuilder};
# use delta_kernel::expressions::{column_expr, Scalar};
# use delta_kernel::{Predicate, Snapshot, SnapshotRef, Error, Engine};
# use delta_kernel::table_changes::TableChanges;
# let path = "./tests/data/table-with-cdf";
let url = delta_kernel::try_parse_uri(path)?;
# use test_utils::delta_kernel_default_engine::storage::store_from_url;
# let engine = std::sync::Arc::new(DefaultEngineBuilder::new(store_from_url(&url)?).build());
// Get the table changes (change data feed) between version 0 and 1
let table_changes = TableChanges::try_new(url, engine.as_ref(), 0, Some(1))?;

// Optionally specify a schema and predicate to apply to the table changes scan
let schema = table_changes
    .schema()
    .project(&["id", "_commit_version"])?;
let predicate = Arc::new(Predicate::gt(column_expr!("id"), Scalar::from(10)));

// Construct the table changes scan
let table_changes_scan = table_changes
    .into_scan_builder()
    .with_schema(schema)
    .with_predicate(predicate.clone())
    .build()?;

// Execute the table changes scan to get a fallible iterator of `Box<dyn EngineData>`s
let table_change_batches = table_changes_scan.execute(engine.clone())?;
# Ok::<(), Error>(())
```
