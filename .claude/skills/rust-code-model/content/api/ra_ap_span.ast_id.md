# `ra_ap_span::ast_id`

Crate `ra_ap_span` · 7 public items · structured records in [`model/ra_ap_span.ast_id.json`](../model/ra_ap_span.ast_id.json)

## FIXUP_ERASED_FILE_AST_ID_MARKER

`constant` · `ra_ap_span::ast_id::FIXUP_ERASED_FILE_AST_ID_MARKER`

Also reachable as `ra_ap_span::FIXUP_ERASED_FILE_AST_ID_MARKER`

```rust
const FIXUP_ERASED_FILE_AST_ID_MARKER: ErasedFileAstId = _
```

ErasedFileAstId used as the span for syntax node fixups. Any Span containing this file id is to be
considered fake.
Do not modify this, it is used by the proc-macro server.

---

## NO_DOWNMAP_ERASED_FILE_AST_ID_MARKER

`constant` · `ra_ap_span::ast_id::NO_DOWNMAP_ERASED_FILE_AST_ID_MARKER`

Also reachable as `ra_ap_span::NO_DOWNMAP_ERASED_FILE_AST_ID_MARKER`

```rust
const NO_DOWNMAP_ERASED_FILE_AST_ID_MARKER: ErasedFileAstId = _
```

[`ErasedFileAstId`] used as the span for syntax nodes that should not be mapped down to
macro expansion. Any `Span` containing this file id is to be considered fake.

---

## ROOT_ERASED_FILE_AST_ID

`constant` · `ra_ap_span::ast_id::ROOT_ERASED_FILE_AST_ID`

Also reachable as `ra_ap_span::ROOT_ERASED_FILE_AST_ID`

```rust
const ROOT_ERASED_FILE_AST_ID: ErasedFileAstId = _
```

The root ast id always points to the encompassing file, using this in spans is discouraged as
any range relative to it will be effectively absolute, ruining the entire point of anchored
relative text ranges.

---

## AstIdMap

`struct` · `ra_ap_span::ast_id::AstIdMap`

Also reachable as `ra_ap_span::AstIdMap`

```rust
struct AstIdMap
```

**Implements**: `core::ops::drop::Drop`

**Derives**: Debug, Default, Eq, PartialEq

**Methods** (8)

```rust
fn ast_id<N: AstIdNode>(&self, item: &N) -> FileAstId<N>
fn ast_id_for_block(&self, block: &ast::BlockExpr) -> Option<FileAstId<ast::BlockExpr>>
fn ast_id_for_ptr<N: AstIdNode>(&self, ptr: AstPtr<N>) -> FileAstId<N>
fn ast_id_for_ptr_for_block(&self, ptr: AstPtr<ast::BlockExpr>) -> Option<FileAstId<ast::BlockExpr>>
fn from_source(node: &SyntaxNode) -> AstIdMap
fn get<N: AstNode>(&self, id: FileAstId<N>) -> AstPtr<N>
fn get_erased(&self, id: ErasedFileAstId) -> SyntaxNodePtr
fn root(&self) -> SyntaxNodePtr
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

Maps items' `SyntaxNode`s to `ErasedFileAstId`s and back.

---

## ErasedFileAstId

`struct` · `ra_ap_span::ast_id::ErasedFileAstId`

Also reachable as `ra_ap_span::ErasedFileAstId`

```rust
struct ErasedFileAstId
```

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
const fn from_raw(v: u32) -> Self
fn into_raw(self) -> u32
fn is_root(self) -> bool
```

This is a type erased FileAstId.

---

## FileAstId

`struct` · `ra_ap_span::ast_id::FileAstId`

Also reachable as `ra_ap_span::FileAstId`

```rust
struct FileAstId<N>
```

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn downcast_unchecked<M: AstIdNode + Into<N>>(self) -> FileAstId<M>
fn erase(self) -> ErasedFileAstId
fn upcast<M: AstIdNode>(self) -> FileAstId<M> where N: Into<M>
```

`AstId` points to an AST node in a specific file.

---

## AstIdNode

`trait` · `ra_ap_span::ast_id::AstIdNode`

Also reachable as `ra_ap_span::AstIdNode`

```rust
trait AstIdNode: AstNode
```

**Implementors** (23)

- `ra_ap_syntax::ast::generated::nodes::Adt`
- `ra_ap_syntax::ast::generated::nodes::AnyHasGenericParams`
- `ra_ap_syntax::ast::generated::nodes::AsmExpr`
- `ra_ap_syntax::ast::generated::nodes::AssocItem`
- `ra_ap_syntax::ast::generated::nodes::Const`
- `ra_ap_syntax::ast::generated::nodes::Enum`
- `ra_ap_syntax::ast::generated::nodes::ExternBlock`
- `ra_ap_syntax::ast::generated::nodes::ExternCrate`
- `ra_ap_syntax::ast::generated::nodes::Fn`
- `ra_ap_syntax::ast::generated::nodes::Impl`
- `ra_ap_syntax::ast::generated::nodes::Item`
- `ra_ap_syntax::ast::generated::nodes::MacroCall`
- `ra_ap_syntax::ast::generated::nodes::MacroDef`
- `ra_ap_syntax::ast::generated::nodes::MacroRules`
- `ra_ap_syntax::ast::generated::nodes::Module`
- `ra_ap_syntax::ast::generated::nodes::Static`
- `ra_ap_syntax::ast::generated::nodes::Struct`
- `ra_ap_syntax::ast::generated::nodes::Trait`
- `ra_ap_syntax::ast::generated::nodes::TypeAlias`
- `ra_ap_syntax::ast::generated::nodes::Union`
- `ra_ap_syntax::ast::generated::nodes::Use`
- `ra_ap_syntax::ast::generated::nodes::Variant`
- `ra_ap_syntax::ast::node_ext::Macro`

---
