# `buoyant_kernel::transaction::create_table`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transaction.create_table.json).

<a id="op-ad42421d0c1cbd4f29a80bee"></a>
## create_table

`module` · `buoyant_kernel::transaction::create_table` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod create_table
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/create_table.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/create_table.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create table transaction types and entry point (internal API).

This module defines the [`CreateTableTransaction`](../operations/buoyant_kernel.transaction.create_table.CreateTableTransaction.md#op-32c02d6f39623e6417b55daf) type alias and the [`create_table`](../operations/buoyant_kernel.transaction.create_table.create_table.md#op-4616c01665602127d3c49d64)
entry point function. The builder logic lives in
[`builder::create_table`](super::builder::create_table).

# Example

```rust,no_run
# use buoyant_kernel as delta_kernel;
use delta_kernel::transaction::create_table::create_table;
use delta_kernel::schema::{StructType, StructField, DataType};
use delta_kernel::committer::FileSystemCommitter;
use std::sync::Arc;
# use delta_kernel::Engine;
# fn example(engine: &dyn Engine) -> delta_kernel::DeltaResult<()> {

let schema = Arc::new(StructType::try_new(vec![
    StructField::new("id", DataType::INTEGER, true),
])?);

let result = create_table("/path/to/table", schema, "MyApp/1.0")
    .with_table_properties([("myapp.version", "1.0")])
    .build(engine, Box::new(FileSystemCommitter::new()))?
    .commit(engine)?;
# Ok(())
# }
```
