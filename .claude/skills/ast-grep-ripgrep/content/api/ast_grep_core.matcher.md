# `ast_grep_core::matcher`

Crate `ast-grep-core` · 4 public items · structured records in [`model/ast_grep_core.matcher.json`](../model/ast_grep_core.matcher.json)

## MatchAll

`struct` · `ast_grep_core::matcher::MatchAll`

```rust
struct MatchAll
```

**Implements**: `ast_grep_core::matcher::Matcher`

**via `ast_grep_core::matcher::Matcher`**

```rust
fn match_node_with_env<'tree, D: Doc>(&self, node: Node<'tree, D>, _env: &mut Cow<'_, MetaVarEnv<'tree, D>>) -> Option<Node<'tree, D>>
fn potential_kinds(&self) -> Option<BitSet>
```

---

## MatchNone

`struct` · `ast_grep_core::matcher::MatchNone`

```rust
struct MatchNone
```

**Implements**: `ast_grep_core::matcher::Matcher`

**via `ast_grep_core::matcher::Matcher`**

```rust
fn match_node_with_env<'tree, D: Doc>(&self, _node: Node<'tree, D>, _env: &mut Cow<'_, MetaVarEnv<'tree, D>>) -> Option<Node<'tree, D>>
fn potential_kinds(&self) -> Option<BitSet>
```

---

## Matcher

`trait` · `ast_grep_core::matcher::Matcher`

Also reachable as `ast_grep_core::Matcher`

```rust
trait Matcher
```

**Implementors** (13)

- `ast_grep_config::rule::Rule`
- `ast_grep_config::rule_core::RuleCore`
- `ast_grep_core::matcher::MatchAll`
- `ast_grep_core::matcher::MatchNone`
- `ast_grep_core::matcher::kind::KindMatcher`
- `ast_grep_core::matcher::pattern::Pattern`
- `ast_grep_core::matcher::text::RegexMatcher`
- `ast_grep_core::ops::All`
- `ast_grep_core::ops::And`
- `ast_grep_core::ops::Any`
- `ast_grep_core::ops::Not`
- `ast_grep_core::ops::Op`
- `ast_grep_core::ops::Or`

**Methods** (3)

```rust
fn get_match_len<D: Doc>(&self, _node: Node<'_, D>) -> Option<usize>
fn match_node_with_env<'tree, D: Doc>(&self, _node: Node<'tree, D>, _env: &mut Cow<'_, MetaVarEnv<'tree, D>>) -> Option<Node<'tree, D>>
fn potential_kinds(&self) -> Option<BitSet>
```

`Matcher` defines whether a tree-sitter node matches certain pattern,
and update the matched meta-variable values in `MetaVarEnv`.
N.B. At least one positive term is required for matching

---

## MatcherExt

`trait` · `ast_grep_core::matcher::MatcherExt`

```rust
trait MatcherExt: Matcher
```

**Methods** (2)

```rust
fn find_node<'tree, D: Doc>(&self, node: Node<'tree, D>) -> Option<NodeMatch<'tree, D>>
fn match_node<'tree, D: Doc>(&self, node: Node<'tree, D>) -> Option<NodeMatch<'tree, D>>
```

MatcherExt provides additional utility methods for `Matcher`.
It is implemented for all types that implement `Matcher`.
N.B. This trait is not intended to be implemented by users.

---
