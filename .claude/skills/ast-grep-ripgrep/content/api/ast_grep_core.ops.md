# `ast_grep_core::ops`

Crate `ast-grep-core` · 6 public items · structured records in [`model/ast_grep_core.ops.json`](../model/ast_grep_core.ops.json)

## All

`struct` · `ast_grep_core::ops::All`

```rust
struct All<P: Matcher>
```

**Implements**: `ast_grep_core::matcher::Matcher`

**Derives**: Clone

**Methods** (2)

```rust
fn inner(&self) -> &[P]
fn new<PS: IntoIterator<Item = P>>(patterns: PS) -> Self
```

**via `ast_grep_core::matcher::Matcher`**

```rust
fn match_node_with_env<'tree, D: Doc>(&self, node: Node<'tree, D>, env: &mut Cow<'_, MetaVarEnv<'tree, D>>) -> Option<Node<'tree, D>>
fn potential_kinds(&self) -> Option<BitSet>
```

---

## And

`struct` · `ast_grep_core::ops::And`

```rust
struct And<P1: Matcher, P2: Matcher>
```

**Implements**: `ast_grep_core::matcher::Matcher`

**via `ast_grep_core::matcher::Matcher`**

```rust
fn match_node_with_env<'tree, D: Doc>(&self, node: Node<'tree, D>, env: &mut Cow<'_, MetaVarEnv<'tree, D>>) -> Option<Node<'tree, D>>
fn potential_kinds(&self) -> Option<BitSet>
```

---

## Any

`struct` · `ast_grep_core::ops::Any`

```rust
struct Any<P>
```

**Implements**: `ast_grep_core::matcher::Matcher`

**Methods** (2)

```rust
fn inner(&self) -> &[P]
fn new<PS: IntoIterator<Item = P>>(patterns: PS) -> Self
```

**via `ast_grep_core::matcher::Matcher`**

```rust
fn match_node_with_env<'tree, D: Doc>(&self, node: Node<'tree, D>, env: &mut Cow<'_, MetaVarEnv<'tree, D>>) -> Option<Node<'tree, D>>
fn potential_kinds(&self) -> Option<BitSet>
```

---

## Not

`struct` · `ast_grep_core::ops::Not`

```rust
struct Not<M: Matcher>
```

**Implements**: `ast_grep_core::matcher::Matcher`

**Methods** (2)

```rust
fn inner(&self) -> &M
fn new(not: M) -> Self
```

**via `ast_grep_core::matcher::Matcher`**

```rust
fn match_node_with_env<'tree, D: Doc>(&self, node: Node<'tree, D>, env: &mut Cow<'_, MetaVarEnv<'tree, D>>) -> Option<Node<'tree, D>>
```

---

## Op

`struct` · `ast_grep_core::ops::Op`

```rust
struct Op<M: Matcher>
```

**Implements**: `ast_grep_core::matcher::Matcher`

**Derives**: Clone

**Methods** (8)

```rust
fn all<MS: IntoIterator<Item = M>>(patterns: MS) -> All<M>
fn and<O: Matcher>(self, other: O) -> Op<And<And<M, N>, O>>
fn any<MS: IntoIterator<Item = M>>(patterns: MS) -> Any<M>
fn either(pattern: M) -> Op<Or<M, MatchNone>>
fn every(pattern: M) -> Op<And<M, MatchAll>>
fn new(matcher: M) -> Op<M>
fn not(pattern: M) -> Not<M>
fn or<O: Matcher>(self, other: O) -> Op<Or<Or<M, N>, O>>
```

**via `ast_grep_core::matcher::Matcher`**

```rust
fn match_node_with_env<'tree, D: Doc>(&self, node: Node<'tree, D>, env: &mut Cow<'_, MetaVarEnv<'tree, D>>) -> Option<Node<'tree, D>>
fn potential_kinds(&self) -> Option<BitSet>
```

---

## Or

`struct` · `ast_grep_core::ops::Or`

```rust
struct Or<P1: Matcher, P2: Matcher>
```

**Implements**: `ast_grep_core::matcher::Matcher`

**via `ast_grep_core::matcher::Matcher`**

```rust
fn match_node_with_env<'tree, D: Doc>(&self, node: Node<'tree, D>, env: &mut Cow<'_, MetaVarEnv<'tree, D>>) -> Option<Node<'tree, D>>
fn potential_kinds(&self) -> Option<BitSet>
```

---
