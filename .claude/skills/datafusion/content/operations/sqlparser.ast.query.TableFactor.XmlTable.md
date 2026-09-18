# `sqlparser::ast::query::TableFactor::XmlTable`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TableFactor.XmlTable.json).

<a id="op-72cd44d423fe4fabfe880545"></a>
## alias

`struct_field` · `sqlparser::ast::query::TableFactor::XmlTable::alias` · sqlparser 0.62.0

```rust
alias: Option<TableAlias>
```

Source: `src/ast/query.rs:1709`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The alias for the table.

<a id="op-ed9f2d4583c36f2f51e64179"></a>
## columns

`struct_field` · `sqlparser::ast::query::TableFactor::XmlTable::columns` · sqlparser 0.62.0

```rust
columns: Vec<XmlTableColumn>
```

Source: `src/ast/query.rs:1707`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The columns to be extracted from each generated row.

<a id="op-da018070dab42de9e353c86f"></a>
## namespaces

`struct_field` · `sqlparser::ast::query::TableFactor::XmlTable::namespaces` · sqlparser 0.62.0

```rust
namespaces: Vec<XmlNamespaceDefinition>
```

Source: `src/ast/query.rs:1701`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional XMLNAMESPACES clause (empty if not present)

<a id="op-8ab52460e2a3c4a96f808943"></a>
## passing

`struct_field` · `sqlparser::ast::query::TableFactor::XmlTable::passing` · sqlparser 0.62.0

```rust
passing: XmlPassingClause
```

Source: `src/ast/query.rs:1705`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The PASSING clause specifying the document expression.

<a id="op-89e5368629d263e45436bf2b"></a>
## row_expression

`struct_field` · `sqlparser::ast::query::TableFactor::XmlTable::row_expression` · sqlparser 0.62.0

```rust
row_expression: Expr
```

Source: `src/ast/query.rs:1703`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The row-generating XPath expression.
