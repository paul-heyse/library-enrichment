# `deltalake_core::table::columns`

Crate `deltalake-core` · 2 public items · structured records in [`model/deltalake_core.table.columns.json`](../model/deltalake_core.table.columns.json)

## Constraint

`struct` · `deltalake_core::table::columns::Constraint`
[Full member contracts, output types and access classification](../operations/deltalake_core.table.columns.Constraint.md)

Also reachable as `deltalake::table::Constraint`, `deltalake_core::table::Constraint`

```rust
struct Constraint
```

**Fields**: `name`, `expr`

**Implements**: `deltalake_core::kernel::schema::DataCheck`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

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

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A constraint in a check constraint

---

## GeneratedColumn

`struct` · `deltalake_core::table::columns::GeneratedColumn`
[Full member contracts, output types and access classification](../operations/deltalake_core.table.columns.GeneratedColumn.md)

Also reachable as `deltalake::table::GeneratedColumn`, `deltalake_core::table::GeneratedColumn`

```rust
struct GeneratedColumn
```

**Fields**: `name`, `generation_expr`, `validation_expr`, `data_type`

**Implements**: `deltalake_core::kernel::schema::DataCheck`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn get_generation_expression(&self) -> &str
fn new(field_name: &str, sql_generation: &str, data_type: &DataType) -> Self
```

**via `deltalake_core::kernel::schema::DataCheck`**

```rust
fn as_any(&self) -> &dyn Any
fn get_expression(&self) -> &str
fn get_name(&self) -> &str
```

A generated column

---
