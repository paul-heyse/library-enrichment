# `ra_ap_syntax::syntax_node`

Crate `ra_ap_syntax` · 8 public items · structured records in [`model/ra_ap_syntax.syntax_node.json`](../model/ra_ap_syntax.syntax_node.json)

## RustLanguage

`enum` · `ra_ap_syntax::syntax_node::RustLanguage`

Also reachable as `ra_ap_syntax::RustLanguage`

```rust
enum RustLanguage
```

**Implements**: `rowan::api::Language`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `rowan::api::Language`**

```rust
fn kind_from_raw(raw: rowan::SyntaxKind) -> SyntaxKind
fn kind_to_raw(kind: SyntaxKind) -> rowan::SyntaxKind
```

---

## SyntaxTreeBuilder

`struct` · `ra_ap_syntax::syntax_node::SyntaxTreeBuilder`

Also reachable as `ra_ap_syntax::SyntaxTreeBuilder`

```rust
struct SyntaxTreeBuilder
```

**Derives**: Default

**Methods** (5)

```rust
fn error(&mut self, error: String, text_pos: TextSize)
fn finish(self) -> Parse<SyntaxNode>
fn finish_node(&mut self)
fn start_node(&mut self, kind: SyntaxKind)
fn token(&mut self, kind: SyntaxKind, text: &str)
```

---

## PreorderWithTokens

`type_alias` · `ra_ap_syntax::syntax_node::PreorderWithTokens`

Also reachable as `ra_ap_syntax::PreorderWithTokens`

```rust
type PreorderWithTokens = rowan::api::PreorderWithTokens<RustLanguage>
```

---

## SyntaxElement

`type_alias` · `ra_ap_syntax::syntax_node::SyntaxElement`

Also reachable as `ra_ap_syntax::SyntaxElement`

```rust
type SyntaxElement = rowan::SyntaxElement<RustLanguage>
```

**Implements**: `ra_ap_syntax::syntax_editor::Element`

**via `ra_ap_syntax::syntax_editor::Element`**

```rust
fn syntax_element(self) -> SyntaxElement
```

---

## SyntaxElementChildren

`type_alias` · `ra_ap_syntax::syntax_node::SyntaxElementChildren`

Also reachable as `ra_ap_syntax::SyntaxElementChildren`

```rust
type SyntaxElementChildren = rowan::SyntaxElementChildren<RustLanguage>
```

---

## SyntaxNode

`type_alias` · `ra_ap_syntax::syntax_node::SyntaxNode`

Also reachable as `ra_ap_syntax::SyntaxNode`

```rust
type SyntaxNode = rowan::SyntaxNode<RustLanguage>
```

**Implements**: `ra_ap_syntax::syntax_editor::Element`

**via `ra_ap_syntax::syntax_editor::Element`**

```rust
fn syntax_element(self) -> SyntaxElement
```

---

## SyntaxNodeChildren

`type_alias` · `ra_ap_syntax::syntax_node::SyntaxNodeChildren`

Also reachable as `ra_ap_syntax::SyntaxNodeChildren`

```rust
type SyntaxNodeChildren = rowan::SyntaxNodeChildren<RustLanguage>
```

---

## SyntaxToken

`type_alias` · `ra_ap_syntax::syntax_node::SyntaxToken`

Also reachable as `ra_ap_syntax::SyntaxToken`

```rust
type SyntaxToken = rowan::SyntaxToken<RustLanguage>
```

**Implements**: `ra_ap_syntax::syntax_editor::Element`

**via `ra_ap_syntax::syntax_editor::Element`**

```rust
fn syntax_element(self) -> SyntaxElement
```

---
