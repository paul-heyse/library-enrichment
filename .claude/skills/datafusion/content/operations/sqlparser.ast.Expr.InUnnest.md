# `sqlparser::ast::Expr::InUnnest`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Expr.InUnnest.json).

<a id="op-a11f3b5a2ef76c16693ba4f2"></a>
## array_expr

`struct_field` · `sqlparser::ast::Expr::InUnnest::array_expr` · sqlparser 0.62.0

```rust
array_expr: Box<Expr>
```

Source: `src/ast/mod.rs:963`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Array expression being unnested.

<a id="op-3bcdc6998481fcfc9b8f9549"></a>
## expr

`struct_field` · `sqlparser::ast::Expr::InUnnest::expr` · sqlparser 0.62.0

```rust
expr: Box<Expr>
```

Source: `src/ast/mod.rs:961`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Left-hand expression to test for membership.

<a id="op-23aa9b03a6b7308a32808e5f"></a>
## negated

`struct_field` · `sqlparser::ast::Expr::InUnnest::negated` · sqlparser 0.62.0

```rust
negated: bool
```

Source: `src/ast/mod.rs:965`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when the `NOT` modifier is present.
