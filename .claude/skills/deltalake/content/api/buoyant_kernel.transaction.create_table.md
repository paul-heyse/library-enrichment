# `buoyant_kernel::transaction::create_table`

Crate `buoyant_kernel` · 2 public items · structured records in [`model/buoyant_kernel.transaction.create_table.json`](../model/buoyant_kernel.transaction.create_table.json)

## create_table

`function` · `buoyant_kernel::transaction::create_table::create_table`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.transaction.create_table.create_table.md)

Also reachable as `delta_kernel::transaction::create_table::create_table`

```rust
fn create_table(path: impl AsRef<str>, schema: schema::SchemaRef, engine_info: impl Into<String>) -> CreateTableTransactionBuilder
```

Creates a builder for creating a new Delta table.

This function returns a [`CreateTableTransactionBuilder`] that can be configured with table
properties and other options before building a [`CreateTableTransaction`].

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

---

## CreateTableTransaction

`type_alias` · `buoyant_kernel::transaction::create_table::CreateTableTransaction`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.transaction.create_table.CreateTableTransaction.md)

Also reachable as `delta_kernel::transaction::create_table::CreateTableTransaction`

```rust
type CreateTableTransaction = transaction::Transaction<transaction::CreateTable>
```

A type alias for create-table transactions.

This provides a restricted API surface that only exposes operations valid during table
creation. Operations like removing files, removing domain metadata, updating deletion
vectors, and setting blind append are not available at compile time.

# Operations NOT available on create-table transactions

- **`with_domain_metadata_removed()`** — Cannot remove domain metadata from a table that doesn't
  exist yet.
- **`remove_files()`** — Cannot remove files from a table that has no files.
- **`with_blind_append()`** — Blind append semantics don't apply to table creation.
- **`update_deletion_vectors()`** — Deletion vectors require an existing table.
- **`with_transaction_id()`** — Transaction ID (app_id) tracking is for existing tables.
- **`with_operation()`** — The operation is fixed to `"CREATE TABLE"`.

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
    .build(engine, Box::new(FileSystemCommitter::new()))?
    .commit(engine)?;
# Ok(())
# }
```

---
