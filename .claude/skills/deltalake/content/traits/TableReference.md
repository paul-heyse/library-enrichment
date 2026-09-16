# TableReference

`deltalake_core::kernel::transaction::TableReference`

```rust
trait TableReference: Send + Sync
```

Also reachable as `deltalake::kernel::transaction::TableReference`

Prose: [`api/deltalake_core.kernel.transaction.md`](../api/deltalake_core.kernel.transaction.md#tablereference) · records: [`model/deltalake_core.kernel.transaction.json`](../model/deltalake_core.kernel.transaction.json)

## Required

Every implementation must supply these.

```rust
fn config(&self) -> &TableProperties
fn eager_snapshot(&self) -> &EagerSnapshot
fn metadata(&self) -> &Metadata
fn protocol(&self) -> &Protocol
```

## Implementors (2)

Read one before writing your own.

- `deltalake_core::kernel::snapshot::EagerSnapshot`
- `deltalake_core::table::state::DeltaTableState`

## Documentation

Reference to some structure that contains mandatory attributes for performing a commit.
