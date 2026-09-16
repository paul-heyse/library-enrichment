# `deltalake_core::kernel::schema`

Crate `deltalake-core` · 1 public items · structured records in [`model/deltalake_core.kernel.schema.json`](../model/deltalake_core.kernel.schema.json)

## DataCheck

`trait` · `deltalake_core::kernel::schema::DataCheck`

Also reachable as `deltalake::DataCheck`, `deltalake::kernel::DataCheck`, `deltalake::kernel::schema::DataCheck`, `deltalake::schema::DataCheck`, `deltalake_core::DataCheck`, `deltalake_core::kernel::DataCheck`, `deltalake_core::schema::DataCheck`

```rust
trait DataCheck
```

**Implementors** (3)

- `deltalake_core::kernel::schema::schema::Invariant`
- `deltalake_core::table::columns::Constraint`
- `deltalake_core::table::columns::GeneratedColumn`

**Methods** (3)

```rust
fn as_any(&self) -> &dyn Any
fn get_expression(&self) -> &str
fn get_name(&self) -> &str
```

A trait for all kernel types that are used as part of data checking

---
