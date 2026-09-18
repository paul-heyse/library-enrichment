# `sqlparser::ast::query::TableFactor::SemanticView`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TableFactor.SemanticView.json).

<a id="op-95ac038e6cbbcfa43528c1d8"></a>
## alias

`struct_field` · `sqlparser::ast::query::TableFactor::SemanticView::alias` · sqlparser 0.62.0

```rust
alias: Option<TableAlias>
```

Source: `src/ast/query.rs:1734`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The alias for the table

<a id="op-127a14bdb45a4fe918daf690"></a>
## dimensions

`struct_field` · `sqlparser::ast::query::TableFactor::SemanticView::dimensions` · sqlparser 0.62.0

```rust
dimensions: Vec<Expr>
```

Source: `src/ast/query.rs:1726`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

List of dimensions or expression referring to dimensions (e.g. DATE_PART('year', col))

<a id="op-b1c6a72da555dc0712cb4334"></a>
## facts

`struct_field` · `sqlparser::ast::query::TableFactor::SemanticView::facts` · sqlparser 0.62.0

```rust
facts: Vec<Expr>
```

Source: `src/ast/query.rs:1730`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

List of facts or expressions referring to facts or dimensions.

<a id="op-f0751bdd934f4c65dec2f6f4"></a>
## metrics

`struct_field` · `sqlparser::ast::query::TableFactor::SemanticView::metrics` · sqlparser 0.62.0

```rust
metrics: Vec<Expr>
```

Source: `src/ast/query.rs:1728`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

List of metrics (references to objects like orders.value, value, orders.*)

<a id="op-9d2b12f6e5e1ff2c5e06fb96"></a>
## name

`struct_field` · `sqlparser::ast::query::TableFactor::SemanticView::name` · sqlparser 0.62.0

```rust
name: ObjectName
```

Source: `src/ast/query.rs:1724`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The name of the semantic model

<a id="op-70448a6d17f288356a7a2b12"></a>
## where_clause

`struct_field` · `sqlparser::ast::query::TableFactor::SemanticView::where_clause` · sqlparser 0.62.0

```rust
where_clause: Option<Expr>
```

Source: `src/ast/query.rs:1732`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

WHERE clause for filtering
