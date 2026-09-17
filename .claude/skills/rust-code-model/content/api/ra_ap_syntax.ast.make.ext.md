# `ra_ap_syntax::ast::make::ext`

Crate `ra_ap_syntax` · 23 public items · structured records in [`model/ra_ap_syntax.ast.make.ext.json`](../model/ra_ap_syntax.ast.make.ext.json)

## default_bool

`function` · `ra_ap_syntax::ast::make::ext::default_bool`

```rust
fn default_bool() -> ast::Expr
```

---

## empty_block_expr

`function` · `ra_ap_syntax::ast::make::ext::empty_block_expr`

```rust
fn empty_block_expr() -> ast::BlockExpr
```

---

## empty_char

`function` · `ra_ap_syntax::ast::make::ext::empty_char`

```rust
fn empty_char() -> ast::Expr
```

---

## empty_str

`function` · `ra_ap_syntax::ast::make::ext::empty_str`

```rust
fn empty_str() -> ast::Expr
```

---

## expr_self

`function` · `ra_ap_syntax::ast::make::ext::expr_self`

```rust
fn expr_self() -> ast::Expr
```

---

## expr_todo

`function` · `ra_ap_syntax::ast::make::ext::expr_todo`

```rust
fn expr_todo() -> ast::Expr
```

---

## expr_ty_default

`function` · `ra_ap_syntax::ast::make::ext::expr_ty_default`

```rust
fn expr_ty_default(ty: &ast::Type) -> ast::Expr
```

---

## expr_ty_new

`function` · `ra_ap_syntax::ast::make::ext::expr_ty_new`

```rust
fn expr_ty_new(ty: &ast::Type) -> ast::Expr
```

---

## expr_underscore

`function` · `ra_ap_syntax::ast::make::ext::expr_underscore`

```rust
fn expr_underscore() -> ast::Expr
```

---

## expr_unit

`function` · `ra_ap_syntax::ast::make::ext::expr_unit`

```rust
fn expr_unit() -> ast::Expr
```

---

## expr_unreachable

`function` · `ra_ap_syntax::ast::make::ext::expr_unreachable`

```rust
fn expr_unreachable() -> ast::Expr
```

---

## field_from_idents

`function` · `ra_ap_syntax::ast::make::ext::field_from_idents`

```rust
fn field_from_idents<'a>(parts: impl std::iter::IntoIterator<Item = &'a str>) -> Option<ast::Expr>
```

---

## ident_path

`function` · `ra_ap_syntax::ast::make::ext::ident_path`

```rust
fn ident_path(ident: &str) -> ast::Path
```

---

## option_none

`function` · `ra_ap_syntax::ast::make::ext::option_none`

```rust
fn option_none() -> ast::Expr
```

---

## path_from_idents

`function` · `ra_ap_syntax::ast::make::ext::path_from_idents`

```rust
fn path_from_idents<'a>(parts: impl std::iter::IntoIterator<Item = &'a str>) -> Option<ast::Path>
```

---

## simple_ident_pat

`function` · `ra_ap_syntax::ast::make::ext::simple_ident_pat`

```rust
fn simple_ident_pat(name: ast::Name) -> ast::IdentPat
```

---

## token_tree_from_node

`function` · `ra_ap_syntax::ast::make::ext::token_tree_from_node`

```rust
fn token_tree_from_node(node: &syntax_node::SyntaxNode) -> ast::TokenTree
```

---

## ty_bool

`function` · `ra_ap_syntax::ast::make::ext::ty_bool`

```rust
fn ty_bool() -> ast::Type
```

---

## ty_name

`function` · `ra_ap_syntax::ast::make::ext::ty_name`

```rust
fn ty_name(name: ast::Name) -> ast::Type
```

---

## ty_option

`function` · `ra_ap_syntax::ast::make::ext::ty_option`

```rust
fn ty_option(t: ast::Type) -> ast::Type
```

---

## ty_result

`function` · `ra_ap_syntax::ast::make::ext::ty_result`

```rust
fn ty_result(t: ast::Type, e: ast::Type) -> ast::Type
```

---

## zero_float

`function` · `ra_ap_syntax::ast::make::ext::zero_float`

```rust
fn zero_float() -> ast::Expr
```

---

## zero_number

`function` · `ra_ap_syntax::ast::make::ext::zero_number`

```rust
fn zero_number() -> ast::Expr
```

---
