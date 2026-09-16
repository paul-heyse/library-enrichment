# ToSchema

`buoyant_kernel::schema::ToSchema`

```rust
trait ToSchema
```

Also reachable as `delta_kernel::schema::ToSchema`

Prose: [`api/buoyant_kernel.schema.md`](../api/buoyant_kernel.schema.md#toschema) · records: [`model/buoyant_kernel.schema.json`](../model/buoyant_kernel.schema.json)

## Required

Every implementation must supply these.

```rust
fn to_schema() -> StructType
```

## Implementors (12)

Read one before writing your own.

- `buoyant_kernel::actions::Add`
- `buoyant_kernel::actions::Cdc`
- `buoyant_kernel::actions::CheckpointMetadata`
- `buoyant_kernel::actions::CommitInfo`
- `buoyant_kernel::actions::DomainMetadata`
- `buoyant_kernel::actions::Format`
- `buoyant_kernel::actions::Metadata`
- `buoyant_kernel::actions::Protocol`
- `buoyant_kernel::actions::Remove`
- `buoyant_kernel::actions::SetTransaction`
- `buoyant_kernel::actions::Sidecar`
- `buoyant_kernel::actions::deletion_vector::DeletionVectorDescriptor`

## Documentation

Converts a type to a [`Schema`] that represents that type. Derivable for struct types using the
[`delta_kernel_derive::ToSchema`] derive macro.
