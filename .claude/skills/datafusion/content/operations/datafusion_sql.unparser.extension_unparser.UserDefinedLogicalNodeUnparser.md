# `datafusion_sql::unparser::extension_unparser::UserDefinedLogicalNodeUnparser`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.unparser.extension_unparser.UserDefinedLogicalNodeUnparser.json).

<a id="op-176cebcf1171735c57540ca8"></a>
## UserDefinedLogicalNodeUnparser

`trait` · `datafusion_sql::unparser::extension_unparser::UserDefinedLogicalNodeUnparser` · datafusion-sql 55.1.0

```rust
trait UserDefinedLogicalNodeUnparser
```

Source: `src/unparser/extension_unparser.rs:24`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

This trait allows users to define custom unparser logic for their custom logical nodes.

<a id="op-479a21e955148f0f5343e9e9"></a>
## unparse

`function` · `datafusion_sql::unparser::extension_unparser::UserDefinedLogicalNodeUnparser::unparse` · datafusion-sql 55.1.0

```rust
fn unparse(&self, _node: &dyn UserDefinedLogicalNode, _unparser: &Unparser<'_>, _query: &mut Option<&mut QueryBuilder>, _select: &mut Option<&mut SelectBuilder>, _relation: &mut Option<&mut RelationBuilder>) -> datafusion_common::Result<UnparseWithinStatementResult>
```

Source: `src/unparser/extension_unparser.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Unparse the custom logical node to SQL within a statement.

This method is called when the custom logical node is part of a statement.
e.g. `SELECT * FROM custom_logical_node`

The return value should be [UnparseWithinStatementResult::Modified](../operations/datafusion_sql.unparser.extension_unparser.UnparseWithinStatementResult.md#op-15c5cfbc862b34b2acc8d55f) if the custom logical node was successfully unparsed.
Otherwise, return [UnparseWithinStatementResult::Unmodified](../operations/datafusion_sql.unparser.extension_unparser.UnparseWithinStatementResult.md#op-13f14243cf65157c6984a576).

<a id="op-e6d1a4ad3301f73f28b501cf"></a>
## unparse_to_statement

`function` · `datafusion_sql::unparser::extension_unparser::UserDefinedLogicalNodeUnparser::unparse_to_statement` · datafusion-sql 55.1.0

```rust
fn unparse_to_statement(&self, _node: &dyn UserDefinedLogicalNode, _unparser: &Unparser<'_>) -> datafusion_common::Result<UnparseToStatementResult>
```

Source: `src/unparser/extension_unparser.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Unparse the custom logical node to a statement.

This method is called when the custom logical node is a custom statement.

The return value should be [UnparseToStatementResult::Modified](../operations/datafusion_sql.unparser.extension_unparser.UnparseToStatementResult.md#op-d2226a1fdd2c98032b9d7f1e) if the custom logical node was successfully unparsed.
Otherwise, return [UnparseToStatementResult::Unmodified](../operations/datafusion_sql.unparser.extension_unparser.UnparseToStatementResult.md#op-068945e97d4853a78688deb3).
