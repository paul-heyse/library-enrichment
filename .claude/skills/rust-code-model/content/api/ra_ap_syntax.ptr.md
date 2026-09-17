# `ra_ap_syntax::ptr`

Crate `ra_ap_syntax` · 2 public items · structured records in [`model/ra_ap_syntax.ptr.json`](../model/ra_ap_syntax.ptr.json)

## AstPtr

`struct` · `ra_ap_syntax::ptr::AstPtr`

Also reachable as `ra_ap_syntax::AstPtr`

```rust
struct AstPtr<N: AstNode>
```

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq

**Methods** (10)

```rust
fn cast<U: AstNode>(self) -> Option<AstPtr<U>>
fn kind(&self) -> parser::SyntaxKind
fn new(node: &N) -> AstPtr<N>
fn syntax_node_ptr(&self) -> SyntaxNodePtr
fn text_range(self) -> TextRange
fn to_node(&self, root: &SyntaxNode) -> N
fn try_from_raw(raw: SyntaxNodePtr) -> Option<AstPtr<N>>
fn upcast<M: AstNode>(self) -> AstPtr<M> where N: Into<M>
fn wrap_left<R>(self) -> AstPtr<either::Either<N, R>> where either::Either<N, R>: AstNode
fn wrap_right<L>(self) -> AstPtr<either::Either<L, N>> where either::Either<L, N>: AstNode
```

Like [`SyntaxNodePtr`], but remembers the type of node.

---

## SyntaxNodePtr

`type_alias` · `ra_ap_syntax::ptr::SyntaxNodePtr`

Also reachable as `ra_ap_syntax::SyntaxNodePtr`

```rust
type SyntaxNodePtr = rowan::ast::SyntaxNodePtr<syntax_node::RustLanguage>
```

**Implements**: `core::convert::From`

**via `core::convert::From`**

```rust
fn from(ptr: AstPtr<N>) -> SyntaxNodePtr
```

A "pointer" to a [`SyntaxNode`], via location in the source code.

---
