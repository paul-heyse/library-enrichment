# `ra_ap_syntax::syntax_editor`

Crate `ra_ap_syntax` · 5 public items · structured records in [`model/ra_ap_syntax.syntax_editor.json`](../model/ra_ap_syntax.syntax_editor.json)

## Position

`struct` · `ra_ap_syntax::syntax_editor::Position`

```rust
struct Position
```

**Derives**: Debug

**Methods** (4)

```rust
fn after(elem: impl Element) -> Position
fn before(elem: impl Element) -> Position
fn first_child_of(node: &impl Into<SyntaxNode> + Clone) -> Position
fn last_child_of(node: &impl Into<SyntaxNode> + Clone) -> Position
```

Position describing where to insert elements

---

## SyntaxAnnotation

`struct` · `ra_ap_syntax::syntax_editor::SyntaxAnnotation`

```rust
struct SyntaxAnnotation
```

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

---

## SyntaxEdit

`struct` · `ra_ap_syntax::syntax_editor::SyntaxEdit`

```rust
struct SyntaxEdit
```

**Methods** (5)

```rust
fn changed_elements(&self) -> &[SyntaxElement]
fn find_annotation(&self, annotation: SyntaxAnnotation) -> &[SyntaxElement]
fn find_element(&self, old_node: &SyntaxNode) -> Option<SyntaxNode>
fn new_root(&self) -> &SyntaxNode
fn old_root(&self) -> &SyntaxNode
```

Represents a completed [`SyntaxEditor`] operation.

---

## SyntaxEditor

`struct` · `ra_ap_syntax::syntax_editor::SyntaxEditor`

```rust
struct SyntaxEditor
```

**Derives**: Debug

**Methods** (18)

```rust
fn add_annotation(&self, element: impl Element, annotation: SyntaxAnnotation)
fn add_annotation_all(&self, elements: Vec<impl Element>, annotation: SyntaxAnnotation)
fn add_generic_param(&self, node: &impl ast::HasGenericParams, new_param: ast::GenericParam)
fn delete(&self, element: impl Element)
fn delete_all(&self, range: RangeInclusive<SyntaxElement>)
fn deleted(&self, element: impl Element) -> bool
fn finish(self) -> SyntaxEdit
fn insert(&self, position: Position, element: impl Element)
fn insert_all(&self, position: Position, elements: Vec<SyntaxElement>)
fn insert_all_with_whitespace(&self, position: Position, elements: Vec<SyntaxElement>)
fn insert_with_whitespace(&self, position: Position, element: impl Element)
fn make(&self) -> &SyntaxFactory
fn merge(&self, other: SyntaxEditor)
fn new(root: SyntaxNode) -> (Self, SyntaxNode)
fn replace(&self, old: impl Element, new: impl Element)
fn replace_all(&self, range: RangeInclusive<SyntaxElement>, new: Vec<SyntaxElement>)
fn replace_with_many(&self, old: impl Element, new: Vec<SyntaxElement>)
fn with_ast_node<T>(root: &T) -> (Self, T) where T: AstNode
```

---

## Element

`trait` · `ra_ap_syntax::syntax_editor::Element`

```rust
trait Element
```

**Implementors** (3)

- `ra_ap_syntax::syntax_node::SyntaxElement`
- `ra_ap_syntax::syntax_node::SyntaxNode`
- `ra_ap_syntax::syntax_node::SyntaxToken`

**Methods** (1)

```rust
fn syntax_element(self) -> SyntaxElement
```

Utility trait to allow calling syntax editor functions with references or owned
nodes. Do not use outside of this module.

---
