# MetadataExt

`deltalake_core::kernel::models::actions::MetadataExt`

```rust
trait MetadataExt
```

Also reachable as `deltalake::kernel::MetadataExt`, `deltalake::kernel::models::MetadataExt`, `deltalake_core::kernel::MetadataExt`, `deltalake_core::kernel::models::MetadataExt`

Prose: [`api/deltalake_core.kernel.models.actions.md`](../api/deltalake_core.kernel.models.actions.md#metadataext) · records: [`model/deltalake_core.kernel.models.actions.json`](../model/deltalake_core.kernel.models.actions.json)

## Required

Every implementation must supply these.

```rust
fn add_config_key(self, key: String, value: String) -> DeltaResult<Metadata>
fn remove_config_key(self, key: &str) -> DeltaResult<Metadata>
fn with_description(self, description: String) -> DeltaResult<Metadata>
fn with_name(self, name: String) -> DeltaResult<Metadata>
fn with_schema(self, schema: &StructType) -> DeltaResult<Metadata>
fn with_table_id(self, table_id: String) -> DeltaResult<Metadata>
```

## Implementors (1)

Read one before writing your own.

- `buoyant_kernel::actions::Metadata`

## Documentation

Extension trait for Metadata action

This trait is a stop-gap to adopt the Metadata action from delta-kernel-rs
while the update / mutation APIs are being implemented. It allows us to implement
additional APIs on the Metadata action and hide specifics of how we do the updates.
