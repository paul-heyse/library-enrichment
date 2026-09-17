# `ast_grep_config::rule::selector`

Crate `ast-grep-config` · 2 public items · structured records in [`model/ast_grep_config.rule.selector.json`](../model/ast_grep_config.rule.selector.json)

## SelectorError

`enum` · `ast_grep_config::rule::selector::SelectorError`

Also reachable as `ast_grep_config::SelectorError`

```rust
enum SelectorError
```

**Variants**: `IllegalCharacter`, `UnexpectedToken`, `MissingSelector`, `InvalidKind`, `Unsupported`, `ExpectedLeftParen`, `ExpectedRightParen`, `UnknownPseudoClass`, `InvalidNthChild`

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

## parse_selector

`function` · `ast_grep_config::rule::selector::parse_selector`

Also reachable as `ast_grep_config::parse_selector`

```rust
fn parse_selector<L: Language>(source: &str, lang: L) -> Result<super::Rule, SelectorError>
```

---
