# `sqlparser::ast::Expr::IsNormalized`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Expr.IsNormalized.json).

<a id="op-6c801d1aecad510db8192f8a"></a>
## expr

`struct_field` · `sqlparser::ast::Expr::IsNormalized::expr` · sqlparser 0.62.0

```rust
expr: Box<Expr>
```

Source: `src/ast/mod.rs:934`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Expression being tested.

<a id="op-f3ff467555460076628efcbb"></a>
## form

`struct_field` · `sqlparser::ast::Expr::IsNormalized::form` · sqlparser 0.62.0

```rust
form: Option<NormalizationForm>
```

Source: `src/ast/mod.rs:936`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional normalization `form` (e.g., NFC, NFD).

<a id="op-3deb97e286fc54dc5cc73140"></a>
## negated

`struct_field` · `sqlparser::ast::Expr::IsNormalized::negated` · sqlparser 0.62.0

```rust
negated: bool
```

Source: `src/ast/mod.rs:938`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when `NOT` is present.
