# `ra_ap_syntax::ast::expr_ext`

Crate `ra_ap_syntax` · 5 public items · structured records in [`model/ra_ap_syntax.ast.expr_ext.json`](../model/ra_ap_syntax.ast.expr_ext.json)

## ArrayExprKind

`enum` · `ra_ap_syntax::ast::expr_ext::ArrayExprKind`

Also reachable as `ra_ap_syntax::ast::ArrayExprKind`

```rust
enum ArrayExprKind
```

**Variants**: `Repeat`, `ElementList`

---

## BlockModifier

`enum` · `ra_ap_syntax::ast::expr_ext::BlockModifier`

Also reachable as `ra_ap_syntax::ast::BlockModifier`

```rust
enum BlockModifier
```

**Variants**: `Async`, `Unsafe`, `Try`, `Const`, `AsyncGen`, `Gen`, `Label`

---

## CallableExpr

`enum` · `ra_ap_syntax::ast::expr_ext::CallableExpr`

Also reachable as `ra_ap_syntax::ast::CallableExpr`

```rust
enum CallableExpr
```

**Variants**: `Call`, `MethodCall`

**Implements**: `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasArgList`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: parser::SyntaxKind) -> bool where Self: Sized
fn cast(syntax: SyntaxNode) -> Option<Self> where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## ElseBranch

`enum` · `ra_ap_syntax::ast::expr_ext::ElseBranch`

Also reachable as `ra_ap_syntax::ast::ElseBranch`

```rust
enum ElseBranch
```

**Variants**: `Block`, `IfExpr`

**Implements**: `core::convert::From`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(if_expr: ast::IfExpr) -> Self
fn from(block_expr: ast::BlockExpr) -> Self
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn syntax(&self) -> &SyntaxNode
```

---

## LiteralKind

`enum` · `ra_ap_syntax::ast::expr_ext::LiteralKind`

Also reachable as `ra_ap_syntax::ast::LiteralKind`

```rust
enum LiteralKind
```

**Variants**: `String`, `ByteString`, `CString`, `IntNumber`, `FloatNumber`, `Char`, `Byte`, `Bool`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

---
