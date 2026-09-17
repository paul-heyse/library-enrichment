# `ast_grep_config::fixer`

Crate `ast-grep-config` · 4 public items · structured records in [`model/ast_grep_config.fixer.json`](../model/ast_grep_config.fixer.json)

## FixerError

`enum` · `ast_grep_config::fixer::FixerError`

```rust
enum FixerError
```

**Variants**: `InvalidTemplate`, `WrongExpansion`, `InvalidRewriter`, `MissingTitle`

---

## SerializableFixer

`enum` · `ast_grep_config::fixer::SerializableFixer`

```rust
enum SerializableFixer
```

**Variants**: `Str`, `Config`, `List`

A pattern string or fix object to auto fix the issue.
It can reference metavariables appeared in rule.

---

## Fixer

`struct` · `ast_grep_config::fixer::Fixer`

Also reachable as `ast_grep_config::Fixer`

```rust
struct Fixer
```

**Implements**: `ast_grep_core::replacer::Replacer`

**Methods** (3)

```rust
fn from_str<L: Language>(src: &str, lang: &L) -> Result<Self, FixerError>
fn parse<L: Language>(fixer: &SerializableFixer, env: &DeserializeEnv<L>, transform: &Option<HashMap<String, Transformation>>) -> Result<Vec<Self>, FixerError>
fn title(&self) -> Option<&str>
```

**via `ast_grep_core::replacer::Replacer`**

```rust
fn generate_replacement(&self, nm: &NodeMatch<'_, D>) -> Vec<C::Underlying>
fn get_replaced_range(&self, nm: &NodeMatch<'_, D>, matcher: impl Matcher) -> Range<usize>
```

---

## SerializableFixConfig

`struct` · `ast_grep_config::fixer::SerializableFixConfig`

```rust
struct SerializableFixConfig
```

---
