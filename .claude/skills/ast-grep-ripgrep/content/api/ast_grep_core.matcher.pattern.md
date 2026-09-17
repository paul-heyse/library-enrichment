# `ast_grep_core::matcher::pattern`

Crate `ast-grep-core` · 5 public items · structured records in [`model/ast_grep_core.matcher.pattern.json`](../model/ast_grep_core.matcher.pattern.json)

## PatternError

`enum` · `ast_grep_core::matcher::pattern::PatternError`

Also reachable as `ast_grep_core::PatternError`, `ast_grep_core::matcher::PatternError`

```rust
enum PatternError
```

**Variants**: `Parse`, `NoContent`, `MultipleNode`, `RootMultiMetaVar`, `InvalidKind`, `NoSelectorInContext`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(source: KindMatcherError) -> Self
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

## PatternNode

`enum` · `ast_grep_core::matcher::pattern::PatternNode`

Also reachable as `ast_grep_core::matcher::PatternNode`

```rust
enum PatternNode
```

**Variants**: `MetaVar`, `Terminal`, `Internal`

**Implements**: `core::convert::From`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn fixed_string(&self) -> Cow<'_, str>
fn is_trivial(&self) -> bool
```

**via `core::convert::From`**

```rust
fn from(node: Node<'r, D>) -> Self
```

---

## DumpPattern

`struct` · `ast_grep_core::matcher::pattern::DumpPattern`

Also reachable as `ast_grep_core::matcher::DumpPattern`

```rust
struct DumpPattern<'p>
```

**Fields**: `is_meta_var`, `kind`, `text`, `children`

---

## Pattern

`struct` · `ast_grep_core::matcher::pattern::Pattern`

Also reachable as `ast_grep_core::Pattern`, `ast_grep_core::matcher::Pattern`

```rust
struct Pattern
```

**Fields**: `node`, `strictness`

**Implements**: `ast_grep_core::matcher::Matcher`, `core::convert::From`

**Derives**: Clone, Debug

**Methods** (8)

```rust
fn contextual<L: Language>(context: &str, selector: &str, lang: L) -> Result<Self, PatternError>
fn defined_vars(&self) -> HashSet<&str>
fn dump(&self, kind_id_to_name: &impl Fn(u16) -> Option<Cow<'static, str>>) -> Option<DumpPattern<'_>>
fn fixed_string(&self) -> Cow<'_, str>
fn has_error(&self) -> bool
fn new<L: Language>(src: &str, lang: L) -> Self
fn try_new<L: Language>(src: &str, lang: L) -> Result<Self, PatternError>
fn with_strictness(self, strictness: MatchStrictness) -> Self
```

**via `ast_grep_core::matcher::Matcher`**

```rust
fn get_match_len<D: Doc>(&self, node: Node<'_, D>) -> Option<usize>
fn match_node_with_env<'tree, D: Doc>(&self, node: Node<'tree, D>, env: &mut Cow<'_, MetaVarEnv<'tree, D>>) -> Option<Node<'tree, D>>
fn potential_kinds(&self) -> Option<bit_set::BitSet>
```

**via `core::convert::From`**

```rust
fn from(node: Node<'r, D>) -> Self
```

---

## PatternBuilder

`struct` · `ast_grep_core::matcher::pattern::PatternBuilder`

Also reachable as `ast_grep_core::matcher::PatternBuilder`

```rust
struct PatternBuilder<'a>
```

**Methods** (1)

```rust
fn build<D, F>(&self, parse: F) -> Result<Pattern, PatternError> where F: FnOnce(&str) -> Result<D, String>, D: Doc
```

---
