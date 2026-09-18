# `sqlparser::ast::Expr::Case`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Expr.Case.json).

<a id="op-ca2af6d6a5973ad3dcfdd8c0"></a>
## case_token

`struct_field` · `sqlparser::ast::Expr::Case::case_token` · sqlparser 0.62.0

```rust
case_token: helpers::attached_token::AttachedToken
```

Source: `src/ast/mod.rs:1245`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The attached `CASE` token (keeps original spacing/comments).

<a id="op-48450bd629868d0f98fcdfbf"></a>
## conditions

`struct_field` · `sqlparser::ast::Expr::Case::conditions` · sqlparser 0.62.0

```rust
conditions: Vec<CaseWhen>
```

Source: `src/ast/mod.rs:1251`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `WHEN ... THEN` conditions and results.

<a id="op-68f23788f5b6497d5eef6624"></a>
## else_result

`struct_field` · `sqlparser::ast::Expr::Case::else_result` · sqlparser 0.62.0

```rust
else_result: Option<Box<Expr>>
```

Source: `src/ast/mod.rs:1253`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `ELSE` result expression.

<a id="op-f15c902408c04c6cd488d2bb"></a>
## end_token

`struct_field` · `sqlparser::ast::Expr::Case::end_token` · sqlparser 0.62.0

```rust
end_token: helpers::attached_token::AttachedToken
```

Source: `src/ast/mod.rs:1247`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The attached `END` token (keeps original spacing/comments).

<a id="op-de28267a9ff820cb50032d2e"></a>
## operand

`struct_field` · `sqlparser::ast::Expr::Case::operand` · sqlparser 0.62.0

```rust
operand: Option<Box<Expr>>
```

Source: `src/ast/mod.rs:1249`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional operand expression after `CASE` (for simple CASE).
