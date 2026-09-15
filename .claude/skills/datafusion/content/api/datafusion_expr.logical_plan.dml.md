# `datafusion_expr::logical_plan::dml`

Crate `datafusion-expr` · 8 public items · structured records in [`model/datafusion_expr.logical_plan.dml.json`](../model/datafusion_expr.logical_plan.dml.json)

## InsertOp

`enum` · `datafusion_expr::logical_plan::dml::InsertOp`

Also reachable as `datafusion_expr::dml::InsertOp`

```rust
enum InsertOp
```

**Variants**: `Append`, `Overwrite`, `Replace`

**Implements**: `core::convert::From`, `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn name(&self) -> &str
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

---

## MergeIntoAction

`enum` · `datafusion_expr::logical_plan::dml::MergeIntoAction`

Also reachable as `datafusion::logical_expr::MergeIntoAction`, `datafusion_expr::MergeIntoAction`, `datafusion_expr::dml::MergeIntoAction`, `datafusion_expr::logical_plan::MergeIntoAction`

```rust
enum MergeIntoAction
```

**Variants**: `Update`, `Insert`, `Delete`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

The action for a single WHEN clause.

---

## MergeIntoClauseKind

`enum` · `datafusion_expr::logical_plan::dml::MergeIntoClauseKind`

Also reachable as `datafusion::logical_expr::MergeIntoClauseKind`, `datafusion_expr::MergeIntoClauseKind`, `datafusion_expr::dml::MergeIntoClauseKind`, `datafusion_expr::logical_plan::MergeIntoClauseKind`

```rust
enum MergeIntoClauseKind
```

**Variants**: `Matched`, `NotMatched`, `NotMatchedByTarget`, `NotMatchedBySource`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn canonical(self) -> Self
fn is_not_matched_by_target(&self) -> bool
```

**via `core::convert::From`**

```rust
fn from(kind: protobuf::merge_into_clause_node::Kind) -> Self
```

Which rows a MERGE WHEN clause applies to.

Mirrors `sqlparser::ast::MergeClauseKind` so that the SQL spelling is
preserved through the logical plan.

**Note on `NotMatched` vs `NotMatchedByTarget`:** these two variants are
semantically identical — both describe a source row that has no matching
target row. `NotMatched` is the SQL standard short form (used by
Snowflake, Postgres, SQL Server); `NotMatchedByTarget` is BigQuery's
explicit form added for symmetry with `NotMatchedBySource`. Downstream
consumers (planners, table providers, optimizers) MUST treat the two
variants identically.

---

## WriteOp

`enum` · `datafusion_expr::logical_plan::dml::WriteOp`

Also reachable as `datafusion::logical_expr::WriteOp`, `datafusion_expr::WriteOp`, `datafusion_expr::dml::WriteOp`, `datafusion_expr::logical_plan::WriteOp`

```rust
enum WriteOp
```

**Variants**: `Insert`, `Delete`, `Update`, `Ctas`, `Truncate`, `MergeInto`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn name(&self) -> &str
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

The type of DML operation to perform.

See [`DmlStatement`] for more details.

Marked `#[non_exhaustive]` so adding new variants in future releases is
not a SemVer break for downstream matchers.

---

## CopyTo

`struct` · `datafusion_expr::logical_plan::dml::CopyTo`

Also reachable as `datafusion_expr::dml::CopyTo`

```rust
struct CopyTo
```

**Fields**: `input`, `output_url`, `partition_by`, `file_type`, `options`, `output_schema`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd

**Methods** (1)

```rust
fn new(input: Arc<LogicalPlan>, output_url: String, partition_by: Vec<String>, file_type: Arc<dyn FileType>, options: HashMap<String, String>) -> Self
```

Operator that copies the contents of a database to file(s)

---

## DmlStatement

`struct` · `datafusion_expr::logical_plan::dml::DmlStatement`

Also reachable as `datafusion::logical_expr::DmlStatement`, `datafusion_expr::DmlStatement`, `datafusion_expr::dml::DmlStatement`, `datafusion_expr::logical_plan::DmlStatement`

```rust
struct DmlStatement
```

**Fields**: `table_name`, `target`, `op`, `input`, `output_schema`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd

**Methods** (2)

```rust
fn name(&self) -> &str
fn new(table_name: TableReference, target: Arc<dyn TableSource>, op: WriteOp, input: Arc<LogicalPlan>) -> Self
```

Modifies the content of a database

This operator is used to perform DML operations such as INSERT, DELETE,
UPDATE, and CTAS (CREATE TABLE AS SELECT).

* `INSERT` - Appends new rows to the existing table. Calls
  [`TableProvider::insert_into`]

* `DELETE` - Removes rows from the table. Calls [`TableProvider::delete_from`]

* `UPDATE` - Modifies existing rows in the table. Calls [`TableProvider::update`]

* `CREATE TABLE AS SELECT` - Creates a new table and populates it with data
  from a query. This is similar to the `INSERT` operation, but it creates a new
  table instead of modifying an existing one.

Note that the structure is adapted from substrait WriteRel)

[`TableProvider`]: https://docs.rs/datafusion/latest/datafusion/datasource/trait.TableProvider.html
[`TableProvider::insert_into`]: https://docs.rs/datafusion/latest/datafusion/datasource/trait.TableProvider.html#method.insert_into
[`TableProvider::delete_from`]: https://docs.rs/datafusion/latest/datafusion/datasource/trait.TableProvider.html#method.delete_from
[`TableProvider::update`]: https://docs.rs/datafusion/latest/datafusion/datasource/trait.TableProvider.html#method.update

---

## MergeIntoClause

`struct` · `datafusion_expr::logical_plan::dml::MergeIntoClause`

Also reachable as `datafusion::logical_expr::MergeIntoClause`, `datafusion_expr::MergeIntoClause`, `datafusion_expr::dml::MergeIntoClause`, `datafusion_expr::logical_plan::MergeIntoClause`

```rust
struct MergeIntoClause
```

**Fields**: `kind`, `predicate`, `action`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

A single WHEN clause within a MERGE INTO statement.

---

## MergeIntoOp

`struct` · `datafusion_expr::logical_plan::dml::MergeIntoOp`

Also reachable as `datafusion::logical_expr::MergeIntoOp`, `datafusion_expr::MergeIntoOp`, `datafusion_expr::dml::MergeIntoOp`, `datafusion_expr::logical_plan::MergeIntoOp`

```rust
struct MergeIntoOp
```

**Fields**: `on`, `clauses`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn exprs(&self) -> Vec<&Expr>
fn with_new_exprs(&self, exprs: Vec<Expr>) -> Result<Self>
```

Describes a MERGE INTO operation's parameters.

---
