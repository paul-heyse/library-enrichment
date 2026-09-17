# `ast_grep_language::html`

Crate `ast-grep-language` · 1 public items · structured records in [`model/ast_grep_language.html.json`](../model/ast_grep_language.html.json)

## Html

`struct` · `ast_grep_language::html::Html`

Also reachable as `ast_grep_language::Html`

```rust
struct Html
```

**Implements**: `ast_grep_core::language::Language`, `ast_grep_core::tree_sitter::LanguageExt`, `ast_grep_language::Alias`, `core::fmt::Display`, `serde_core::de::Deserialize`

**Derives**: Clone, Copy, Debug

**via `ast_grep_core::language::Language`**

```rust
fn build_pattern(&self, builder: &PatternBuilder<'_>) -> Result<Pattern, PatternError>
fn expando_char(&self) -> char
fn field_to_id(&self, field: &str) -> Option<u16>
fn kind_to_id(&self, kind: &str) -> u16
fn pre_process_pattern<'q>(&self, query: &'q str) -> std::borrow::Cow<'q, str>
```

**via `ast_grep_core::tree_sitter::LanguageExt`**

```rust
fn extract_injections<L: LanguageExt>(&self, root: Node<'_, StrDoc<L>>) -> Vec<(String, Vec<TSRange>)>
fn get_ts_language(&self) -> TSLanguage
fn injectable_languages(&self) -> Option<&'static [&'static str]>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

---
