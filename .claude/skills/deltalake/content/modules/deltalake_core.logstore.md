# `deltalake_core::logstore`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.json).

<a id="op-fafbdacc7fa1a9134feeca7a"></a>
## logstore

`module` · `deltalake_core::logstore` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod logstore
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L1).

Source: `crates/core/src/logstore/mod.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

# DeltaLake storage system

Interacting with storage systems is a crucial part of any table format.
On one had the storage abstractions need to provide certain guarantees
(e.g. atomic rename, ...) and meet certain assumptions (e.g. sorted list results)
on the other hand can we exploit our knowledge about the general file layout
and access patterns to optimize our operations in terms of cost and performance.

Two distinct phases are involved in querying a Delta table:
- **Metadata**: Fetching metadata about the table, such as schema, partitioning, and statistics.
- **Data**: Reading and processing data files based on the metadata.

When writing to a table, we see the same phases, just in inverse order:
- **Data**: Writing data files that should become part of the table.
- **Metadata**: Updating table metadata to incorporate updates.

Two main abstractions govern the file operations [`LogStore`](../operations/deltalake_core.logstore.LogStore.md#op-05065cf369d14ac5f8039639) and [`ObjectStore`].

[`LogStore`](../operations/deltalake_core.logstore.LogStore.md#op-05065cf369d14ac5f8039639)s are scoped to individual tables and are responsible for maintaining proper
behaviours and ensuring consistency during the metadata phase. The correctness is predicated
on the atomicity and durability guarantees of the implementation of this interface.

- Atomic visibility: Partial writes must not be visible to readers.
- Mutual exclusion: Only one writer must be able to write to a specific log file.
- Consistent listing: Once a file has been written, any future list files operation must return
  the underlying file system entry must immediately.

<div class="warning">

While most object stores today provide the required guarantees, the specific
locking mechanics are a table level responsibility. Specific implementations may
decide to refer to a central catalog or other mechanisms for coordination.

</div>

[`ObjectStore`]s are responsible for direct interactions with storage systems. Either
during the data phase, where additional requirements are imposed on the storage system,
or by specific LogStore implementations for their internal object store interactions.

## Managing LogStores and ObjectStores.

Aside from very basic implementations (i.e. in-memory and local file system) we rely
on external integrations to provide [`ObjectStore`] and/or [`LogStore`](../operations/deltalake_core.logstore.LogStore.md#op-05065cf369d14ac5f8039639) implementations.

At runtime, deltalake needs to produce appropriate [`ObjectStore`]s to access the files
discovered in a table. This is done via

## Configuration


Unresolved upstream links (retained, not inferred): ``ObjectStore``.
