# `sqlparser::ast::Expr::Extract`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Expr.Extract.json).

<a id="op-0f47fe6c19a1b708aefe33dc"></a>
## expr

`struct_field` · `sqlparser::ast::Expr::Extract::expr` · sqlparser 0.62.0

```rust
expr: Box<Expr>
```

Source: `src/ast/mod.rs:1124`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Expression to extract from.

<a id="op-258d558370c47467d62b6dd5"></a>
## field

`struct_field` · `sqlparser::ast::Expr::Extract::field` · sqlparser 0.62.0

```rust
field: DateTimeField
```

Source: `src/ast/mod.rs:1120`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Which datetime field is being extracted.

<a id="op-f582c5878626ea10a4e65c34"></a>
## syntax

`struct_field` · `sqlparser::ast::Expr::Extract::syntax` · sqlparser 0.62.0

```rust
syntax: ExtractSyntax
```

Source: `src/ast/mod.rs:1122`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Syntax variant used (`From` or `Comma`).
