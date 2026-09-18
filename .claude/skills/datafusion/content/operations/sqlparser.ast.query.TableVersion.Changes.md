# `sqlparser::ast::query::TableVersion::Changes`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TableVersion.Changes.json).

<a id="op-df37b3aa117762d4ee739c75"></a>
## at

`struct_field` · `sqlparser::ast::query::TableVersion::Changes::at` · sqlparser 0.62.0

```rust
at: Expr
```

Source: `src/ast/query.rs:2624`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `AT(TIMESTAMP => ...)` function-call expression.

<a id="op-81c8f8614e5021dacf4ba0b9"></a>
## changes

`struct_field` · `sqlparser::ast::query::TableVersion::Changes::changes` · sqlparser 0.62.0

```rust
changes: Expr
```

Source: `src/ast/query.rs:2622`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `CHANGES(INFORMATION => ...)` function-call expression.

<a id="op-2fe8a64d359b2d4dd07615eb"></a>
## end

`struct_field` · `sqlparser::ast::query::TableVersion::Changes::end` · sqlparser 0.62.0

```rust
end: Option<Expr>
```

Source: `src/ast/query.rs:2626`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The optional `END(TIMESTAMP => ...)` function-call expression.
