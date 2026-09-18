# `sqlparser::ast::Expr::CompoundFieldAccess`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Expr.CompoundFieldAccess.json).

<a id="op-41f032535cb3d95b0370874f"></a>
## access_chain

`struct_field` · `sqlparser::ast::Expr::CompoundFieldAccess::access_chain` · sqlparser 0.62.0

```rust
access_chain: Vec<AccessExpr>
```

Source: `src/ast/mod.rs:898`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Sequence of access operations (subscript or identifier accesses).

<a id="op-2b881134ca52de3e7b92b81c"></a>
## root

`struct_field` · `sqlparser::ast::Expr::CompoundFieldAccess::root` · sqlparser 0.62.0

```rust
root: Box<Expr>
```

Source: `src/ast/mod.rs:896`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The base expression being accessed.
