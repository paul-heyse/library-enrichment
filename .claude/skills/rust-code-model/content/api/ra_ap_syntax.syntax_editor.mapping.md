# `ra_ap_syntax::syntax_editor::mapping`

Crate `ra_ap_syntax` · 2 public items · structured records in [`model/ra_ap_syntax.syntax_editor.mapping.json`](../model/ra_ap_syntax.syntax_editor.mapping.json)

## SyntaxMapping

`struct` · `ra_ap_syntax::syntax_editor::mapping::SyntaxMapping`

Also reachable as `ra_ap_syntax::syntax_editor::SyntaxMapping`

```rust
struct SyntaxMapping
```

**Derives**: Debug, Default

**Methods** (2)

```rust
fn add_mapping(&mut self, syntax_mapping: SyntaxMappingBuilder)
fn merge(&mut self, other: SyntaxMapping)
```

---

## SyntaxMappingBuilder

`struct` · `ra_ap_syntax::syntax_editor::mapping::SyntaxMappingBuilder`

Also reachable as `ra_ap_syntax::syntax_editor::SyntaxMappingBuilder`

```rust
struct SyntaxMappingBuilder
```

**Derives**: Debug

**Methods** (4)

```rust
fn finish(self, mappings: &mut SyntaxMapping)
fn map_children(&mut self, input: impl IntoIterator<Item = SyntaxNode>, output: impl IntoIterator<Item = SyntaxNode>)
fn map_node(&mut self, input: SyntaxNode, output: SyntaxNode)
fn new(parent_node: SyntaxNode) -> Self
```

---
