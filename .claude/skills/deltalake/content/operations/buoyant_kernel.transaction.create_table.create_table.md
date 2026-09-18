# `buoyant_kernel::transaction::create_table::create_table`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transaction.create_table.create_table.json).

<a id="op-4616c01665602127d3c49d64"></a>
## create_table

`function` · `buoyant_kernel::transaction::create_table::create_table` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn create_table(path: impl AsRef<str>, schema: schema::SchemaRef, engine_info: impl Into<String>) -> CreateTableTransactionBuilder
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/create_table.rs#L128).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/create_table.rs:128`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a builder for creating a new Delta table.

This function returns a [`CreateTableTransactionBuilder`](../operations/buoyant_kernel.transaction.builder.create_table.CreateTableTransactionBuilder.md#op-05656d6a5734e8cc8804084f) that can be configured with table
properties and other options before building a [`CreateTableTransaction`](../operations/buoyant_kernel.transaction.create_table.CreateTableTransaction.md#op-32c02d6f39623e6417b55daf).

# Arguments

* `path` - The file system path where the Delta table will be created
* `schema` - The schema for the new table
* `engine_info` - Information about the engine creating the table (e.g., "MyApp/1.0")

# Example

```no_run
# use buoyant_kernel as delta_kernel;
use std::sync::Arc;
use delta_kernel::transaction::create_table::create_table;
use delta_kernel::schema::{DataType, StructField, StructType};
use delta_kernel::committer::FileSystemCommitter;
use test_utils::delta_kernel_default_engine::DefaultEngineBuilder;
use test_utils::delta_kernel_default_engine::storage::store_from_url;

# fn main() -> delta_kernel::DeltaResult<()> {
let schema = Arc::new(StructType::new_unchecked(vec![
    StructField::new("id", DataType::INTEGER, true),
    StructField::new("name", DataType::STRING, true),
]));

let url = url::Url::parse("file:///tmp/my_table")?;
let engine = DefaultEngineBuilder::new(store_from_url(&url)?).build();

let transaction = create_table("/tmp/my_table", schema, "MyApp/1.0")
    .build(&engine, Box::new(FileSystemCommitter::new()))?;

// Commit the transaction to create the table
transaction.commit(&engine)?;
# Ok(())
# }
```
