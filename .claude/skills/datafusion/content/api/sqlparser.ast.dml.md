# `sqlparser::ast::dml`

Crate `sqlparser` · 16 public items · structured records in [`model/sqlparser.ast.dml.json`](../model/sqlparser.ast.dml.json)

## MergeAction

`enum` · `sqlparser::ast::dml::MergeAction`

Also reachable as `sqlparser::ast::MergeAction`

```rust
enum MergeAction
```

**Variants**: `Insert`, `Update`, `Delete`

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

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.dml.MergeAction.md).


Underlying statement of a `WHEN` clause within a `MERGE` Statement

Example
```sql
INSERT (product, quantity) VALUES(product, quantity)
```

[Snowflake](https://docs.snowflake.com/en/sql-reference/sql/merge)
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/dml-syntax#merge_statement)
[Oracle](https://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/MERGE.html)

---

## MergeClauseKind

`enum` · `sqlparser::ast::dml::MergeClauseKind`

Also reachable as `sqlparser::ast::MergeClauseKind`

```rust
enum MergeClauseKind
```

**Variants**: `Matched`, `NotMatched`, `NotMatchedByTarget`, `NotMatchedBySource`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

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

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.dml.MergeClauseKind.md).


Variant of `WHEN` clause used within a `MERGE` Statement.

Example:
```sql
MERGE INTO T USING U ON FALSE WHEN MATCHED THEN DELETE
```
[Snowflake](https://docs.snowflake.com/en/sql-reference/sql/merge)
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/dml-syntax#merge_statement)

---

## MergeInsertKind

`enum` · `sqlparser::ast::dml::MergeInsertKind`

Also reachable as `sqlparser::ast::MergeInsertKind`

```rust
enum MergeInsertKind
```

**Variants**: `Values`, `Row`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

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

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.dml.MergeInsertKind.md).


The type of expression used to insert rows within a `MERGE` statement.

[Snowflake](https://docs.snowflake.com/en/sql-reference/sql/merge)
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/dml-syntax#merge_statement)

---

## MultiTableInsertType

`enum` · `sqlparser::ast::dml::MultiTableInsertType`

Also reachable as `sqlparser::ast::MultiTableInsertType`

```rust
enum MultiTableInsertType
```

**Variants**: `All`, `First`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

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

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.dml.MultiTableInsertType.md).


The type of multi-table INSERT statement(Snowflake).

See: <https://docs.snowflake.com/en/sql-reference/sql/insert-multi-table>

---

## MultiTableInsertValue

`enum` · `sqlparser::ast::dml::MultiTableInsertValue`

Also reachable as `sqlparser::ast::MultiTableInsertValue`

```rust
enum MultiTableInsertValue
```

**Variants**: `Expr`, `Default`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

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

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.dml.MultiTableInsertValue.md).


A value in a multi-table INSERT VALUES clause.

---

## OutputClause

`enum` · `sqlparser::ast::dml::OutputClause`

Also reachable as `sqlparser::ast::OutputClause`

```rust
enum OutputClause
```

**Variants**: `Output`, `Returning`

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

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.dml.OutputClause.md).


An `OUTPUT` clause on `MERGE`, `INSERT`, `UPDATE`, or `DELETE` (MSSQL).

Example:
OUTPUT $action, deleted.* INTO dbo.temp_products;
<https://learn.microsoft.com/en-us/sql/t-sql/queries/output-clause-transact-sql>

---

## Delete

`struct` · `sqlparser::ast::dml::Delete`

Also reachable as `sqlparser::ast::Delete`

```rust
struct Delete
```

**Fields**: `delete_token`, `optimizer_hints`, `tables`, `from`, `using`, `selection`, `returning`, `output`, `order_by`, `limit`

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

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.dml.Delete.md).


DELETE statement.

---

## Insert

`struct` · `sqlparser::ast::dml::Insert`

Also reachable as `sqlparser::ast::Insert`

```rust
struct Insert
```

**Fields**: `insert_token`, `optimizer_hints`, `or`, `ignore`, `into`, `table`, `table_alias`, `columns`, `overwrite`, `source`, `assignments`, `partitioned`, `after_columns`, `has_table_keyword`, `on`, `returning`, `output`, `replace_into`, `priority`, `insert_alias`, `settings`, `format_clause`, `multi_table_insert_type`, `multi_table_into_clauses`, `multi_table_when_clauses`, `multi_table_else_clause`

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

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.dml.Insert.md).


INSERT statement.

---

## Merge

`struct` · `sqlparser::ast::dml::Merge`

Also reachable as `sqlparser::ast::Merge`

```rust
struct Merge
```

**Fields**: `merge_token`, `optimizer_hints`, `into`, `table`, `source`, `on`, `clauses`, `output`

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

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.dml.Merge.md).


A `MERGE` statement.

---

## MergeClause

`struct` · `sqlparser::ast::dml::MergeClause`

Also reachable as `sqlparser::ast::MergeClause`

```rust
struct MergeClause
```

**Fields**: `when_token`, `clause_kind`, `predicate`, `action`

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

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.dml.MergeClause.md).


A `WHEN` clause within a `MERGE` Statement

Example:
```sql
WHEN NOT MATCHED BY SOURCE AND product LIKE '%washer%' THEN DELETE
```
[Snowflake](https://docs.snowflake.com/en/sql-reference/sql/merge)
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/dml-syntax#merge_statement)

---

## MergeInsertExpr

`struct` · `sqlparser::ast::dml::MergeInsertExpr`

Also reachable as `sqlparser::ast::MergeInsertExpr`

```rust
struct MergeInsertExpr
```

**Fields**: `insert_token`, `columns`, `kind_token`, `kind`, `insert_predicate`

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

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.dml.MergeInsertExpr.md).


The expression used to insert rows within a `MERGE` statement.

Examples
```sql
INSERT (product, quantity) VALUES(product, quantity)
INSERT ROW
```

[Snowflake](https://docs.snowflake.com/en/sql-reference/sql/merge)
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/dml-syntax#merge_statement)
[Oracle](https://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/MERGE.html)

---

## MergeUpdateExpr

`struct` · `sqlparser::ast::dml::MergeUpdateExpr`

Also reachable as `sqlparser::ast::MergeUpdateExpr`

```rust
struct MergeUpdateExpr
```

**Fields**: `update_token`, `assignments`, `update_predicate`, `delete_predicate`

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

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.dml.MergeUpdateExpr.md).


The expression used to update rows within a `MERGE` statement.

Examples
```sql
UPDATE SET quantity = T.quantity + S.quantity
```

[Snowflake](https://docs.snowflake.com/en/sql-reference/sql/merge)
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/dml-syntax#merge_statement)
[Oracle](https://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/MERGE.html)

---

## MultiTableInsertIntoClause

`struct` · `sqlparser::ast::dml::MultiTableInsertIntoClause`

Also reachable as `sqlparser::ast::MultiTableInsertIntoClause`

```rust
struct MultiTableInsertIntoClause
```

**Fields**: `table_name`, `columns`, `values`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

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

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.dml.MultiTableInsertIntoClause.md).


An INTO clause in a multi-table INSERT.

Syntax:
```sql
INTO <target_table> [ ( <target_col_name> [ , ... ] ) ] [ VALUES ( { <source_col_name> | DEFAULT | NULL } [ , ... ] ) ]
```

---

## MultiTableInsertValues

`struct` · `sqlparser::ast::dml::MultiTableInsertValues`

Also reachable as `sqlparser::ast::MultiTableInsertValues`

```rust
struct MultiTableInsertValues
```

**Fields**: `values`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.dml.MultiTableInsertValues.md).


The VALUES clause in a multi-table INSERT INTO clause.

---

## MultiTableInsertWhenClause

`struct` · `sqlparser::ast::dml::MultiTableInsertWhenClause`

Also reachable as `sqlparser::ast::MultiTableInsertWhenClause`

```rust
struct MultiTableInsertWhenClause
```

**Fields**: `condition`, `into_clauses`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

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

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.dml.MultiTableInsertWhenClause.md).


A WHEN clause in a conditional multi-table INSERT.

Syntax:
```sql
WHEN n1 > 100 THEN
  INTO t1
  INTO t2 (c1, c2) VALUES (n1, n2)
```

---

## Update

`struct` · `sqlparser::ast::dml::Update`

Also reachable as `sqlparser::ast::Update`

```rust
struct Update
```

**Fields**: `update_token`, `optimizer_hints`, `table`, `assignments`, `from`, `selection`, `returning`, `output`, `or`, `order_by`, `limit`

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

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.dml.Update.md).


UPDATE statement.

---
