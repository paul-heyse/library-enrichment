# `sqlparser::ast::query::TableFactor::Derived`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TableFactor.Derived.json).

<a id="op-5f792feb5ee8218a61239149"></a>
## alias

`struct_field` · `sqlparser::ast::query::TableFactor::Derived::alias` · sqlparser 0.62.0

```rust
alias: Option<TableAlias>
```

Source: `src/ast/query.rs:1506`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional alias for the derived table.

<a id="op-e65244cbcd0b838c16b4dd16"></a>
## lateral

`struct_field` · `sqlparser::ast::query::TableFactor::Derived::lateral` · sqlparser 0.62.0

```rust
lateral: bool
```

Source: `src/ast/query.rs:1502`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the derived table is LATERAL.

<a id="op-a4e7bd5727c23cf194fbb8da"></a>
## sample

`struct_field` · `sqlparser::ast::query::TableFactor::Derived::sample` · sqlparser 0.62.0

```rust
sample: Option<TableSampleKind>
```

Source: `src/ast/query.rs:1508`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional table sample modifier

<a id="op-8999416e62fad752bb53bcd9"></a>
## subquery

`struct_field` · `sqlparser::ast::query::TableFactor::Derived::subquery` · sqlparser 0.62.0

```rust
subquery: Box<Query>
```

Source: `src/ast/query.rs:1504`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The subquery producing the derived table.
