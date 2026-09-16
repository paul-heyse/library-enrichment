# `deltalake_core::kernel::schema::schema`

Crate `deltalake-core` · 4 public items · structured records in [`model/deltalake_core.kernel.schema.schema.json`](../model/deltalake_core.kernel.schema.schema.json)

## Invariant

`struct` · `deltalake_core::kernel::schema::schema::Invariant`

Also reachable as `deltalake::Invariant`, `deltalake::kernel::Invariant`, `deltalake::kernel::schema::Invariant`, `deltalake::schema::Invariant`, `deltalake_core::Invariant`, `deltalake_core::kernel::Invariant`, `deltalake_core::kernel::schema::Invariant`, `deltalake_core::schema::Invariant`

```rust
struct Invariant
```

**Fields**: `field_name`, `invariant_sql`

**Implements**: `deltalake_core::kernel::schema::DataCheck`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(field_name: &str, invariant_sql: &str) -> Self
```

**via `deltalake_core::kernel::schema::DataCheck`**

```rust
fn as_any(&self) -> &dyn Any
fn get_expression(&self) -> &str
fn get_name(&self) -> &str
```

An invariant for a column that is enforced on all writes to a Delta table.

---

## StructTypeExt

`trait` · `deltalake_core::kernel::schema::schema::StructTypeExt`

Also reachable as `deltalake::StructTypeExt`, `deltalake::kernel::StructTypeExt`, `deltalake::kernel::schema::StructTypeExt`, `deltalake::schema::StructTypeExt`, `deltalake_core::StructTypeExt`, `deltalake_core::kernel::StructTypeExt`, `deltalake_core::kernel::schema::StructTypeExt`, `deltalake_core::schema::StructTypeExt`

```rust
trait StructTypeExt
```

**Implementors** (1)

- `buoyant_kernel::schema::StructType`

**Methods** (2)

```rust
fn get_generated_columns(&self) -> Result<Vec<GeneratedColumn>, Error>
fn get_invariants(&self) -> Result<Vec<Invariant>, Error>
```

Trait to add convenience functions to struct type

---

## Schema

`type_alias` · `deltalake_core::kernel::schema::schema::Schema`

Also reachable as `deltalake::Schema`, `deltalake::kernel::Schema`, `deltalake::kernel::schema::Schema`, `deltalake::schema::Schema`, `deltalake_core::Schema`, `deltalake_core::kernel::Schema`, `deltalake_core::kernel::schema::Schema`, `deltalake_core::schema::Schema`

```rust
type Schema = StructType
```

Type alias for a top level schema

---

## SchemaRef

`type_alias` · `deltalake_core::kernel::schema::schema::SchemaRef`

Also reachable as `deltalake::SchemaRef`, `deltalake::kernel::SchemaRef`, `deltalake::kernel::schema::SchemaRef`, `deltalake::schema::SchemaRef`, `deltalake_core::SchemaRef`, `deltalake_core::kernel::SchemaRef`, `deltalake_core::kernel::schema::SchemaRef`, `deltalake_core::schema::SchemaRef`

```rust
type SchemaRef = std::sync::Arc<StructType>
```

Schema reference type

---
