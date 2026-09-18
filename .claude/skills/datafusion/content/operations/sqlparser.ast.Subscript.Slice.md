# `sqlparser::ast::Subscript::Slice`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Subscript.Slice.json).

<a id="op-413da743751163fc2a1ce6db"></a>
## lower_bound

`struct_field` · `sqlparser::ast::Subscript::Slice::lower_bound` · sqlparser 0.62.0

```rust
lower_bound: Option<Expr>
```

Source: `src/ast/mod.rs:1417`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional lower bound for the slice (inclusive).

<a id="op-73b9f4add3c931b2034644a5"></a>
## stride

`struct_field` · `sqlparser::ast::Subscript::Slice::stride` · sqlparser 0.62.0

```rust
stride: Option<Expr>
```

Source: `src/ast/mod.rs:1421`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional stride for the slice (step size).

<a id="op-254834ab21db65ffdee86f50"></a>
## upper_bound

`struct_field` · `sqlparser::ast::Subscript::Slice::upper_bound` · sqlparser 0.62.0

```rust
upper_bound: Option<Expr>
```

Source: `src/ast/mod.rs:1419`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional upper bound for the slice (inclusive).
