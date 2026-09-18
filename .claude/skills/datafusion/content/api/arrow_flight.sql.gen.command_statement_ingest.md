# `arrow_flight::sql::gen::command_statement_ingest`

Crate `arrow-flight` · 1 public items · structured records in [`model/arrow_flight.sql.gen.command_statement_ingest.json`](../model/arrow_flight.sql.gen.command_statement_ingest.json)

## TableDefinitionOptions

`struct` · `arrow_flight::sql::gen::command_statement_ingest::TableDefinitionOptions`

Also reachable as `arrow_flight::sql::TableDefinitionOptions`

```rust
struct TableDefinitionOptions
```

**Fields**: `if_not_exist`, `if_exists`

**Implements**: `prost::message::Message`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn if_exists(&self) -> table_definition_options::TableExistsOption
fn if_not_exist(&self) -> table_definition_options::TableNotExistOption
fn set_if_exists(&mut self, value: table_definition_options::TableExistsOption)
fn set_if_not_exist(&mut self, value: table_definition_options::TableNotExistOption)
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.command_statement_ingest.TableDefinitionOptions.md).


Options for table definition behavior

---
