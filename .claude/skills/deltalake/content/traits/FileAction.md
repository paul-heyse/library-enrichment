# FileAction

`deltalake_core::delta_datafusion::cdf::FileAction`

```rust
trait FileAction
```

Also reachable as `deltalake::delta_datafusion::cdf::FileAction`

Prose: [`api/deltalake_core.delta_datafusion.cdf.md`](../api/deltalake_core.delta_datafusion.cdf.md#fileaction) · records: [`model/deltalake_core.delta_datafusion.cdf.json`](../model/deltalake_core.delta_datafusion.cdf.json)

## Required

Every implementation must supply these.

```rust
fn deletion_vector(&self) -> Option<DeletionVectorDescriptor>
fn partition_values(&self) -> DeltaResult<&HashMap<String, Option<String>>>
fn path(&self) -> String
fn size(&self) -> DeltaResult<usize>
```

## Provided

Defaulted, and this is where the capability hides. The default is the conservative answer -- no pushdown, no statistics, no specialization -- so an implementation that overrides none of these works correctly and performs badly.

```rust
fn has_deletion_vector(&self) -> bool
```

## Implementors (3)

Read one before writing your own.

- `deltalake_core::kernel::models::actions::Add`
- `deltalake_core::kernel::models::actions::AddCDCFile`
- `deltalake_core::kernel::models::actions::Remove`

## Documentation

This trait defines a generic set of operations used by CDF Reader
