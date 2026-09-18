# `arrow_flight::sql::gen::command_statement_ingest::table_definition_options`

Crate `arrow-flight` · 2 public items · structured records in [`model/arrow_flight.sql.gen.command_statement_ingest.table_definition_options.json`](../model/arrow_flight.sql.gen.command_statement_ingest.table_definition_options.json)

## TableExistsOption

`enum` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableExistsOption`

Also reachable as `arrow_flight::sql::TableExistsOption`

```rust
enum TableExistsOption
```

**Variants**: `Unspecified`, `Fail`, `Append`, `Replace`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<TableExistsOption>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<TableExistsOption, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.command_statement_ingest.table_definition_options.TableExistsOption.md).


The action to take if the target table already exists

---

## TableNotExistOption

`enum` · `arrow_flight::sql::gen::command_statement_ingest::table_definition_options::TableNotExistOption`

Also reachable as `arrow_flight::sql::TableNotExistOption`

```rust
enum TableNotExistOption
```

**Variants**: `Unspecified`, `Create`, `Fail`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<TableNotExistOption>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<TableNotExistOption, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.command_statement_ingest.table_definition_options.TableNotExistOption.md).


The action to take if the target table does not exist

---
