# StructTypeExt

`deltalake_core::kernel::schema::schema::StructTypeExt`

```rust
trait StructTypeExt
```

Also reachable as `deltalake::StructTypeExt`, `deltalake::kernel::StructTypeExt`, `deltalake::kernel::schema::StructTypeExt`, `deltalake::schema::StructTypeExt`, `deltalake_core::StructTypeExt`, `deltalake_core::kernel::StructTypeExt`, `deltalake_core::kernel::schema::StructTypeExt`, `deltalake_core::schema::StructTypeExt`

Prose: [`api/deltalake_core.kernel.schema.schema.md`](../api/deltalake_core.kernel.schema.schema.md#structtypeext) · records: [`model/deltalake_core.kernel.schema.schema.json`](../model/deltalake_core.kernel.schema.schema.json)

## Required

Every implementation must supply these.

```rust
fn get_generated_columns(&self) -> Result<Vec<GeneratedColumn>, Error>
fn get_invariants(&self) -> Result<Vec<Invariant>, Error>
```

## Implementors (1)

Read one before writing your own.

- `buoyant_kernel::schema::StructType`

## Documentation

Trait to add convenience functions to struct type
