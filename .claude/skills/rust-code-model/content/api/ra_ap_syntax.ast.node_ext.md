# `ra_ap_syntax::ast::node_ext`

Crate `ra_ap_syntax` · 14 public items · structured records in [`model/ra_ap_syntax.ast.node_ext.json`](../model/ra_ap_syntax.ast.node_ext.json)

## AttrKind

`enum` · `ra_ap_syntax::ast::node_ext::AttrKind`

Also reachable as `ra_ap_syntax::ast::AttrKind`

```rust
enum AttrKind
```

**Variants**: `Inner`, `Outer`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn is_inner(&self) -> bool
fn is_outer(&self) -> bool
```

---

## CfgAtomKey

`enum` · `ra_ap_syntax::ast::node_ext::CfgAtomKey`

Also reachable as `ra_ap_syntax::ast::CfgAtomKey`

```rust
enum CfgAtomKey
```

**Variants**: `True`, `False`, `Ident`

**Derives**: Clone, Debug

---

## FieldKind

`enum` · `ra_ap_syntax::ast::node_ext::FieldKind`

Also reachable as `ra_ap_syntax::ast::FieldKind`

```rust
enum FieldKind
```

**Variants**: `Name`, `Index`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---

## Macro

`enum` · `ra_ap_syntax::ast::node_ext::Macro`

Also reachable as `ra_ap_syntax::ast::Macro`

```rust
enum Macro
```

**Variants**: `MacroRules`, `MacroDef`

**Implements**: `core::convert::From`, `ra_ap_span::ast_id::AstIdNode`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`, `ra_ap_syntax::ast::traits::HasName`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(it: ast::MacroDef) -> Self
fn from(it: ast::MacroRules) -> Self
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn syntax(&self) -> &SyntaxNode
```

**via `ra_ap_syntax::ast::traits::HasName`**

```rust
fn name(&self) -> Option<ast::Name>
```

---

## NameLike

`enum` · `ra_ap_syntax::ast::node_ext::NameLike`

Also reachable as `ra_ap_syntax::ast::NameLike`

```rust
enum NameLike
```

**Variants**: `NameRef`, `Name`, `Lifetime`

**Implements**: `core::convert::From`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn as_lifetime(&self) -> Option<&ast::Lifetime>
fn as_name_ref(&self) -> Option<&ast::NameRef>
fn text(&self) -> &str
```

**via `core::convert::From`**

```rust
fn from(it: Name) -> NameLike
fn from(it: NameRef) -> NameLike
fn from(it: Lifetime) -> NameLike
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn syntax(&self) -> &SyntaxNode
```

---

## NameOrNameRef

`enum` · `ra_ap_syntax::ast::node_ext::NameOrNameRef`

Also reachable as `ra_ap_syntax::ast::NameOrNameRef`

```rust
enum NameOrNameRef
```

**Variants**: `Name`, `NameRef`

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn text(&self) -> &str
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn syntax(&self) -> &SyntaxNode
```

---

## PathSegmentKind

`enum` · `ra_ap_syntax::ast::node_ext::PathSegmentKind`

Also reachable as `ra_ap_syntax::ast::PathSegmentKind`

```rust
enum PathSegmentKind
```

**Variants**: `Name`, `Type`, `SelfTypeKw`, `SelfKw`, `SuperKw`, `CrateKw`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---

## SelfParamKind

`enum` · `ra_ap_syntax::ast::node_ext::SelfParamKind`

Also reachable as `ra_ap_syntax::ast::SelfParamKind`

```rust
enum SelfParamKind
```

**Variants**: `Owned`, `Ref`, `MutRef`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## StructKind

`enum` · `ra_ap_syntax::ast::node_ext::StructKind`

Also reachable as `ra_ap_syntax::ast::StructKind`

```rust
enum StructKind
```

**Variants**: `Record`, `Tuple`, `Unit`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---

## TypeBoundKind

`enum` · `ra_ap_syntax::ast::node_ext::TypeBoundKind`

Also reachable as `ra_ap_syntax::ast::TypeBoundKind`

```rust
enum TypeBoundKind
```

**Variants**: `PathType`, `Use`, `Lifetime`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## TypeOrConstParam

`enum` · `ra_ap_syntax::ast::node_ext::TypeOrConstParam`

Also reachable as `ra_ap_syntax::ast::TypeOrConstParam`

```rust
enum TypeOrConstParam
```

**Variants**: `Type`, `Const`

**Implements**: `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn name(&self) -> Option<ast::Name>
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool where Self: Sized
fn cast(syntax: SyntaxNode) -> Option<Self> where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## VisibilityKind

`enum` · `ra_ap_syntax::ast::node_ext::VisibilityKind`

Also reachable as `ra_ap_syntax::ast::VisibilityKind`

```rust
enum VisibilityKind
```

**Variants**: `In`, `PubCrate`, `PubSuper`, `PubSelf`, `Pub`

---

## SlicePatComponents

`struct` · `ra_ap_syntax::ast::node_ext::SlicePatComponents`

Also reachable as `ra_ap_syntax::ast::SlicePatComponents`

```rust
struct SlicePatComponents
```

**Fields**: `prefix`, `slice`, `suffix`

---

## TokenTreeChildren

`struct` · `ra_ap_syntax::ast::node_ext::TokenTreeChildren`

Also reachable as `ra_ap_syntax::ast::TokenTreeChildren`

```rust
struct TokenTreeChildren
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Clone

**Methods** (1)

```rust
fn new(tt: &ast::TokenTree) -> Self
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

An iterator over the elements in an [`ast::TokenTree`].

Does not yield trivia or the delimiters.

---
