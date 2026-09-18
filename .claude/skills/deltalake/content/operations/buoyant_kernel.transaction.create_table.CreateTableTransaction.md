# `buoyant_kernel::transaction::create_table::CreateTableTransaction`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transaction.create_table.CreateTableTransaction.json).

<a id="op-32c02d6f39623e6417b55daf"></a>
## CreateTableTransaction

`type_alias` · `buoyant_kernel::transaction::create_table::CreateTableTransaction` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type CreateTableTransaction = transaction::Transaction<transaction::CreateTable>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/create_table.rs#L87).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/create_table.rs:87`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

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
