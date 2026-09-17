# `ast_grep_core::matcher::kind`

Crate `ast-grep-core` · 2 public items · structured records in [`model/ast_grep_core.matcher.kind.json`](../model/ast_grep_core.matcher.kind.json)

## KindMatcherError

`enum` · `ast_grep_core::matcher::kind::KindMatcherError`

Also reachable as `ast_grep_core::matcher::KindMatcherError`

```rust
enum KindMatcherError
```

**Variants**: `InvalidKindName`

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

---

## KindMatcher

`struct` · `ast_grep_core::matcher::kind::KindMatcher`

Also reachable as `ast_grep_core::matcher::KindMatcher`

```rust
struct KindMatcher
```

**Implements**: `ast_grep_core::matcher::Matcher`

**Derives**: Clone

**Methods** (5)

```rust
fn error_matcher() -> Self
fn from_id(kind: u16) -> Self
fn is_invalid(&self) -> bool
fn new<L: Language>(node_kind: &str, lang: L) -> Self
fn try_new<L: Language>(node_kind: &str, lang: L) -> Result<Self, KindMatcherError>
```

**via `ast_grep_core::matcher::Matcher`**

```rust
fn match_node_with_env<'tree, D: Doc>(&self, node: Node<'tree, D>, _env: &mut Cow<'_, MetaVarEnv<'tree, D>>) -> Option<Node<'tree, D>>
fn potential_kinds(&self) -> Option<BitSet>
```

---
