# `datafusion_sql::unparser::extension_unparser`

Crate `datafusion-sql` · 3 public items · structured records in [`model/datafusion_sql.unparser.extension_unparser.json`](../model/datafusion_sql.unparser.extension_unparser.json)

## UnparseToStatementResult

`enum` · `datafusion_sql::unparser::extension_unparser::UnparseToStatementResult`

```rust
enum UnparseToStatementResult
```

**Variants**: `Modified`, `Unmodified`

The result of unparsing a custom logical node to a statement.

---

## UnparseWithinStatementResult

`enum` · `datafusion_sql::unparser::extension_unparser::UnparseWithinStatementResult`

```rust
enum UnparseWithinStatementResult
```

**Variants**: `Modified`, `Unmodified`

The result of unparsing a custom logical node within a statement.

---

## UserDefinedLogicalNodeUnparser

`trait` · `datafusion_sql::unparser::extension_unparser::UserDefinedLogicalNodeUnparser`

```rust
trait UserDefinedLogicalNodeUnparser
```

**Methods** (2)

```rust
fn unparse(&self, _node: &dyn UserDefinedLogicalNode, _unparser: &Unparser<'_>, _query: &mut Option<&mut QueryBuilder>, _select: &mut Option<&mut SelectBuilder>, _relation: &mut Option<&mut RelationBuilder>) -> datafusion_common::Result<UnparseWithinStatementResult>
fn unparse_to_statement(&self, _node: &dyn UserDefinedLogicalNode, _unparser: &Unparser<'_>) -> datafusion_common::Result<UnparseToStatementResult>
```

This trait allows users to define custom unparser logic for their custom logical nodes.

---
