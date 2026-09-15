# `datafusion_common::column`

Crate `datafusion-common` · 1 public items · structured records in [`model/datafusion_common.column.json`](../model/datafusion_common.column.json)

## Column

`struct` · `datafusion_common::column::Column`

Also reachable as `datafusion::common::Column`, `datafusion::prelude::Column`, `datafusion_common::Column`

```rust
struct Column
```

**Fields**: `relation`, `name`, `spans`

**Implements**: `core::convert::From`, `core::fmt::Display`, `core::str::traits::FromStr`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (13)

```rust
fn flat_name(&self) -> String
fn from_name(name: impl Into<String>) -> Self
fn from_qualified_name(flat_name: impl Into<String>) -> Self
fn from_qualified_name_ignore_case(flat_name: impl Into<String>) -> Self
fn name(&self) -> &str
fn new(relation: Option<impl Into<TableReference>>, name: impl Into<String>) -> Self
fn new_unqualified(name: impl Into<String>) -> Self
fn normalize_with_schemas_and_ambiguity_check(self, schemas: &[&[&DFSchema]], using_columns: &[HashSet<Column>]) -> Result<Self>
fn quoted_flat_name(&self) -> String
fn spans(&self) -> &Spans
fn spans_mut(&mut self) -> &mut Spans
fn with_relation(&self, relation: TableReference) -> Self
fn with_spans(self, spans: Spans) -> Self
```

**via `core::convert::From`**

```rust
fn from(c: String) -> Self
fn from((relation, field): (Option<&TableReference>, &Field)) -> Self
fn from((relation, field): (Option<&TableReference>, &FieldRef)) -> Self
fn from(c: &str) -> Self
fn from(c: &String) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

A named reference to a qualified field in a schema.

---
