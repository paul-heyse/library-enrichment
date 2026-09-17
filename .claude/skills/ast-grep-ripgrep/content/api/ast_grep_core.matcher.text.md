# `ast_grep_core::matcher::text`

Crate `ast-grep-core` · 2 public items · structured records in [`model/ast_grep_core.matcher.text.json`](../model/ast_grep_core.matcher.text.json)

## RegexMatcherError

`enum` · `ast_grep_core::matcher::text::RegexMatcherError`

Also reachable as `ast_grep_core::matcher::RegexMatcherError`

```rust
enum RegexMatcherError
```

**Variants**: `Regex`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(source: RegexError) -> Self
```

**via `core::error::Error`**

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private20::Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

---

## RegexMatcher

`struct` · `ast_grep_core::matcher::text::RegexMatcher`

Also reachable as `ast_grep_core::matcher::RegexMatcher`

```rust
struct RegexMatcher
```

**Implements**: `ast_grep_core::matcher::Matcher`

**Derives**: Clone

**Methods** (1)

```rust
fn try_new(text: &str) -> Result<Self, RegexMatcherError>
```

**via `ast_grep_core::matcher::Matcher`**

```rust
fn match_node_with_env<'tree, D: Doc>(&self, node: Node<'tree, D>, _env: &mut Cow<'_, MetaVarEnv<'tree, D>>) -> Option<Node<'tree, D>>
fn potential_kinds(&self) -> Option<BitSet>
```

---
