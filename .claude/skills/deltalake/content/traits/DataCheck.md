# DataCheck

`deltalake_core::kernel::schema::DataCheck`

```rust
trait DataCheck
```

Also reachable as `deltalake::DataCheck`, `deltalake::kernel::DataCheck`, `deltalake::kernel::schema::DataCheck`, `deltalake::schema::DataCheck`, `deltalake_core::DataCheck`, `deltalake_core::kernel::DataCheck`, `deltalake_core::schema::DataCheck`

Prose: [`api/deltalake_core.kernel.schema.md`](../api/deltalake_core.kernel.schema.md#datacheck) · records: [`model/deltalake_core.kernel.schema.json`](../model/deltalake_core.kernel.schema.json)

## Required

Every implementation must supply these.

```rust
fn as_any(&self) -> &dyn Any
fn get_expression(&self) -> &str
fn get_name(&self) -> &str
```

## Implementors (3)

Read one before writing your own.

- `deltalake_core::kernel::schema::schema::Invariant`
- `deltalake_core::table::columns::Constraint`
- `deltalake_core::table::columns::GeneratedColumn`

## Documentation

A trait for all kernel types that are used as part of data checking
