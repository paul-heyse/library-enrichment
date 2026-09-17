# `regex_syntax::hir::translate`

Crate `regex-syntax` · 2 public items · structured records in [`model/regex_syntax.hir.translate.json`](../model/regex_syntax.hir.translate.json)

## Translator

`struct` · `regex_syntax::hir::translate::Translator`

```rust
struct Translator
```

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn new() -> Translator
fn translate(&mut self, pattern: &str, ast: &Ast) -> core::result::Result<Hir, hir::Error>
```

A translator maps abstract syntax to a high level intermediate
representation.

A translator may be benefit from reuse. That is, a translator can translate
many abstract syntax trees.

A `Translator` can be configured in more detail via a
[`TranslatorBuilder`].

---

## TranslatorBuilder

`struct` · `regex_syntax::hir::translate::TranslatorBuilder`

```rust
struct TranslatorBuilder
```

**Derives**: Clone, Debug, Default

**Methods** (10)

```rust
fn build(&self) -> Translator
fn case_insensitive(&mut self, yes: bool) -> &mut TranslatorBuilder
fn crlf(&mut self, yes: bool) -> &mut TranslatorBuilder
fn dot_matches_new_line(&mut self, yes: bool) -> &mut TranslatorBuilder
fn line_terminator(&mut self, byte: u8) -> &mut TranslatorBuilder
fn multi_line(&mut self, yes: bool) -> &mut TranslatorBuilder
fn new() -> TranslatorBuilder
fn swap_greed(&mut self, yes: bool) -> &mut TranslatorBuilder
fn unicode(&mut self, yes: bool) -> &mut TranslatorBuilder
fn utf8(&mut self, yes: bool) -> &mut TranslatorBuilder
```

A builder for constructing an AST->HIR translator.

---
