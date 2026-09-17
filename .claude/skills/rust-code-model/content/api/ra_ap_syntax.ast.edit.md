# `ra_ap_syntax::ast::edit`

Crate `ra_ap_syntax` · 4 public items · structured records in [`model/ra_ap_syntax.ast.edit.json`](../model/ra_ap_syntax.ast.edit.json)

## indent

`function` · `ra_ap_syntax::ast::edit::indent`

```rust
fn indent(node: &SyntaxNode, level: IndentLevel) -> SyntaxNode
```

---

## IndentLevel

`struct` · `ra_ap_syntax::ast::edit::IndentLevel`

```rust
struct IndentLevel
```

**Implements**: `core::convert::From`, `core::fmt::Display`, `core::ops::arith::Add`, `core::ops::arith::AddAssign`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn from_element(element: &SyntaxElement) -> IndentLevel
fn from_node(node: &SyntaxNode) -> IndentLevel
fn from_token(token: &SyntaxToken) -> IndentLevel
fn is_zero(&self) -> bool
fn zero() -> IndentLevel
```

**via `core::convert::From`**

```rust
fn from(level: u8) -> IndentLevel
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::ops::arith::Add`**

```rust
fn add(self, rhs: u8) -> IndentLevel
```

**via `core::ops::arith::AddAssign`**

```rust
fn add_assign(&mut self, rhs: u8)
```

---

## AstNodeEdit

`trait` · `ra_ap_syntax::ast::edit::AstNodeEdit`

```rust
trait AstNodeEdit: AstNode + Clone + Sized
```

**Methods** (5)

```rust
fn dedent(&self, level: IndentLevel) -> Self
fn indent(&self, level: IndentLevel) -> Self
fn indent_level(&self) -> IndentLevel
fn indent_with_mapping(&self, level: IndentLevel, make: &SyntaxFactory) -> Self
fn reset_indent(&self) -> Self
```

---

## AttrsOwnerEdit

`trait` · `ra_ap_syntax::ast::edit::AttrsOwnerEdit`

```rust
trait AttrsOwnerEdit: ast::HasAttrs
```

**Methods** (1)

```rust
fn remove_attrs_and_docs(&self, editor: &SyntaxEditor)
```

---
