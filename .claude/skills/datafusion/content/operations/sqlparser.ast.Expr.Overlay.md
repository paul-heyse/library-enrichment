# `sqlparser::ast::Expr::Overlay`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Expr.Overlay.json).

<a id="op-0d224324606f371091383a9e"></a>
## expr

`struct_field` · `sqlparser::ast::Expr::Overlay::expr` · sqlparser 0.62.0

```rust
expr: Box<Expr>
```

Source: `src/ast/mod.rs:1203`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The target expression being overlayed.

<a id="op-5c66d34dfb71d34ca67fff7f"></a>
## overlay_for

`struct_field` · `sqlparser::ast::Expr::Overlay::overlay_for` · sqlparser 0.62.0

```rust
overlay_for: Option<Box<Expr>>
```

Source: `src/ast/mod.rs:1209`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `FOR` length expression limiting the overlay span.

<a id="op-44d9f8f85cd74f345211bba7"></a>
## overlay_from

`struct_field` · `sqlparser::ast::Expr::Overlay::overlay_from` · sqlparser 0.62.0

```rust
overlay_from: Box<Expr>
```

Source: `src/ast/mod.rs:1207`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `FROM` position expression indicating where to start overlay.

<a id="op-18db4d30bfc940a093c4ad43"></a>
## overlay_what

`struct_field` · `sqlparser::ast::Expr::Overlay::overlay_what` · sqlparser 0.62.0

```rust
overlay_what: Box<Expr>
```

Source: `src/ast/mod.rs:1205`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The expression to place into the target.
