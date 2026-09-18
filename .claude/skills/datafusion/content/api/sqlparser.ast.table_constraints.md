# `sqlparser::ast::table_constraints`

Crate `sqlparser` · 8 public items · structured records in [`model/sqlparser.ast.table_constraints.json`](../model/sqlparser.ast.table_constraints.json)

## TableConstraint

`enum` · `sqlparser::ast::table_constraints::TableConstraint`

Also reachable as `sqlparser::ast::TableConstraint`

```rust
enum TableConstraint
```

**Variants**: `Unique`, `PrimaryKey`, `ForeignKey`, `Check`, `Index`, `FulltextOrSpatial`, `PrimaryKeyUsingIndex`, `UniqueUsingIndex`

**Implements**: `core::convert::From`, `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::spans::Spanned`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(constraint: PrimaryKeyConstraint) -> Self
fn from(constraint: IndexConstraint) -> Self
fn from(constraint: UniqueConstraint) -> Self
fn from(constraint: CheckConstraint) -> Self
fn from(constraint: ForeignKeyConstraint) -> Self
fn from(constraint: FullTextOrSpatialConstraint) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::spans::Spanned`**

```rust
fn span(&self) -> Span
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.table_constraints.TableConstraint.md).


A table-level constraint, specified in a `CREATE TABLE` or an
`ALTER TABLE ADD <constraint>` statement.

---

## CheckConstraint

`struct` · `sqlparser::ast::table_constraints::CheckConstraint`

Also reachable as `sqlparser::ast::CheckConstraint`

```rust
struct CheckConstraint
```

**Fields**: `name`, `expr`, `enforced`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::spans::Spanned`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::spans::Spanned`**

```rust
fn span(&self) -> Span
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.table_constraints.CheckConstraint.md).


A `CHECK` constraint (`[ CONSTRAINT <name> ] CHECK (<expr>) [[NOT] ENFORCED]`).

---

## ConstraintUsingIndex

`struct` · `sqlparser::ast::table_constraints::ConstraintUsingIndex`

Also reachable as `sqlparser::ast::ConstraintUsingIndex`

```rust
struct ConstraintUsingIndex
```

**Fields**: `name`, `index_name`, `characteristics`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::spans::Spanned`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn fmt_with_keyword(&self, f: &mut fmt::Formatter<'_>, keyword: &str) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::spans::Spanned`**

```rust
fn span(&self) -> Span
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.table_constraints.ConstraintUsingIndex.md).


PostgreSQL constraint that promotes an existing unique index to a table constraint.

`[ CONSTRAINT constraint_name ] { UNIQUE | PRIMARY KEY } USING INDEX index_name
  [ DEFERRABLE | NOT DEFERRABLE ] [ INITIALLY DEFERRED | INITIALLY IMMEDIATE ]`

See <https://www.postgresql.org/docs/current/sql-altertable.html>

---

## ForeignKeyConstraint

`struct` · `sqlparser::ast::table_constraints::ForeignKeyConstraint`

Also reachable as `sqlparser::ast::ForeignKeyConstraint`

```rust
struct ForeignKeyConstraint
```

**Fields**: `name`, `index_name`, `columns`, `foreign_table`, `referred_columns`, `on_delete`, `on_update`, `match_kind`, `characteristics`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::spans::Spanned`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::spans::Spanned`**

```rust
fn span(&self) -> Span
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.table_constraints.ForeignKeyConstraint.md).


A referential integrity constraint (`[ CONSTRAINT <name> ] FOREIGN KEY (<columns>)
REFERENCES <foreign_table> (<referred_columns>) [ MATCH { FULL | PARTIAL | SIMPLE } ]
{ [ON DELETE <referential_action>] [ON UPDATE <referential_action>] |
  [ON UPDATE <referential_action>] [ON DELETE <referential_action>]
}`).

---

## FullTextOrSpatialConstraint

`struct` · `sqlparser::ast::table_constraints::FullTextOrSpatialConstraint`

Also reachable as `sqlparser::ast::FullTextOrSpatialConstraint`

```rust
struct FullTextOrSpatialConstraint
```

**Fields**: `fulltext`, `index_type_display`, `opt_index_name`, `columns`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::spans::Spanned`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::spans::Spanned`**

```rust
fn span(&self) -> Span
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.table_constraints.FullTextOrSpatialConstraint.md).


MySQLs [fulltext][1] definition. Since the [`SPATIAL`][2] definition is exactly the same,
and MySQL displays both the same way, it is part of this definition as well.

Supported syntax:

```markdown
{FULLTEXT | SPATIAL} [INDEX | KEY] [index_name] (key_part,...)

key_part: col_name
```

[1]: https://dev.mysql.com/doc/refman/8.0/en/fulltext-natural-language.html
[2]: https://dev.mysql.com/doc/refman/8.0/en/spatial-types.html

---

## IndexConstraint

`struct` · `sqlparser::ast::table_constraints::IndexConstraint`

Also reachable as `sqlparser::ast::IndexConstraint`

```rust
struct IndexConstraint
```

**Fields**: `display_as_key`, `name`, `index_type`, `columns`, `index_options`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::spans::Spanned`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::spans::Spanned`**

```rust
fn span(&self) -> Span
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.table_constraints.IndexConstraint.md).


MySQLs [index definition][1] for index creation. Not present on ANSI so, for now, the usage
is restricted to MySQL, as no other dialects that support this syntax were found.

`{INDEX | KEY} [index_name] [index_type] (key_part,...) [index_option]...`

[1]: https://dev.mysql.com/doc/refman/8.0/en/create-table.html

---

## PrimaryKeyConstraint

`struct` · `sqlparser::ast::table_constraints::PrimaryKeyConstraint`

Also reachable as `sqlparser::ast::PrimaryKeyConstraint`

```rust
struct PrimaryKeyConstraint
```

**Fields**: `name`, `index_name`, `index_type`, `columns`, `index_options`, `characteristics`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::spans::Spanned`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::spans::Spanned`**

```rust
fn span(&self) -> Span
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.table_constraints.PrimaryKeyConstraint.md).


MySQL [definition][1] for `PRIMARY KEY` constraints statements:
* `[CONSTRAINT [<name>]] PRIMARY KEY [index_name] [index_type] (<columns>) <index_options>`

Actually the specification have no `[index_name]` but the next query will complete successfully:
```sql
CREATE TABLE unspec_table (
  xid INT NOT NULL,
  CONSTRAINT p_name PRIMARY KEY index_name USING BTREE (xid)
);
```

where:
* [index_type][2] is `USING {BTREE | HASH}`
* [index_options][3] is `{index_type | COMMENT 'string' | ... %currently unsupported stmts% } ...`

[1]: https://dev.mysql.com/doc/refman/8.3/en/create-table.html
[2]: IndexType
[3]: IndexOption

---

## UniqueConstraint

`struct` · `sqlparser::ast::table_constraints::UniqueConstraint`

Also reachable as `sqlparser::ast::UniqueConstraint`

```rust
struct UniqueConstraint
```

**Fields**: `name`, `index_name`, `index_type_display`, `index_type`, `columns`, `index_options`, `characteristics`, `nulls_distinct`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::spans::Spanned`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::spans::Spanned`**

```rust
fn span(&self) -> Span
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.table_constraints.UniqueConstraint.md).


Unique constraint definition.

---
