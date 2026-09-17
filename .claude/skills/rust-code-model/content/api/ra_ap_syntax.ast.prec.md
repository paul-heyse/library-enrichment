# `ra_ap_syntax::ast::prec`

Crate `ra_ap_syntax` · 3 public items · structured records in [`model/ra_ap_syntax.ast.prec.json`](../model/ra_ap_syntax.ast.prec.json)

## ExprPrecedence

`enum` · `ra_ap_syntax::ast::prec::ExprPrecedence`

```rust
enum ExprPrecedence
```

**Variants**: `Jump`, `Assign`, `Range`, `LOr`, `LAnd`, `Compare`, `BitOr`, `BitXor`, `BitAnd`, `Shift`, `Sum`, `Product`, `Cast`, `Prefix`, `Postfix`, `Unambiguous`

**Derives**: Clone, Copy, Debug, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn needs_parentheses_in(self, other: ExprPrecedence) -> bool
```

---

## Fixity

`enum` · `ra_ap_syntax::ast::prec::Fixity`

```rust
enum Fixity
```

**Variants**: `Left`, `Right`, `None`

**Derives**: Debug, PartialEq, StructuralPartialEq

---

## precedence

`function` · `ra_ap_syntax::ast::prec::precedence`

```rust
fn precedence(expr: &ast::Expr) -> ExprPrecedence
```

---
