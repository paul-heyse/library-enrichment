# `regex_syntax::parser`

Crate `regex-syntax` · 3 public items · structured records in [`model/regex_syntax.parser.json`](../model/regex_syntax.parser.json)

## parse

`function` · `regex_syntax::parser::parse`

Also reachable as `regex_syntax::parse`

```rust
fn parse(pattern: &str) -> Result<hir::Hir, Error>
```

A convenience routine for parsing a regex using default options.

This is equivalent to `Parser::new().parse(pattern)`.

If you need to set non-default options, then use a [`ParserBuilder`].

This routine returns an [`Hir`](hir::Hir) value. Namely, it automatically
parses the pattern as an [`Ast`](ast::Ast) and then invokes the translator
to convert the `Ast` into an `Hir`. If you need access to the `Ast`, then
you should use a [`ast::parse::Parser`].

---

## Parser

`struct` · `regex_syntax::parser::Parser`

Also reachable as `regex_syntax::Parser`

```rust
struct Parser
```

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn new() -> Parser
fn parse(&mut self, pattern: &str) -> Result<hir::Hir, Error>
```

A convenience parser for regular expressions.

This parser takes as input a regular expression pattern string (the
"concrete syntax") and returns a high-level intermediate representation
(the HIR) suitable for most types of analysis. In particular, this parser
hides the intermediate state of producing an AST (the "abstract syntax").
The AST is itself far more complex than the HIR, so this parser serves as a
convenience for never having to deal with it at all.

If callers have more fine grained use cases that need an AST, then please
see the [`ast::parse`] module.

A `Parser` can be configured in more detail via a [`ParserBuilder`].

---

## ParserBuilder

`struct` · `regex_syntax::parser::ParserBuilder`

Also reachable as `regex_syntax::ParserBuilder`

```rust
struct ParserBuilder
```

**Derives**: Clone, Debug, Default

**Methods** (13)

```rust
fn build(&self) -> Parser
fn case_insensitive(&mut self, yes: bool) -> &mut ParserBuilder
fn crlf(&mut self, yes: bool) -> &mut ParserBuilder
fn dot_matches_new_line(&mut self, yes: bool) -> &mut ParserBuilder
fn ignore_whitespace(&mut self, yes: bool) -> &mut ParserBuilder
fn line_terminator(&mut self, byte: u8) -> &mut ParserBuilder
fn multi_line(&mut self, yes: bool) -> &mut ParserBuilder
fn nest_limit(&mut self, limit: u32) -> &mut ParserBuilder
fn new() -> ParserBuilder
fn octal(&mut self, yes: bool) -> &mut ParserBuilder
fn swap_greed(&mut self, yes: bool) -> &mut ParserBuilder
fn unicode(&mut self, yes: bool) -> &mut ParserBuilder
fn utf8(&mut self, yes: bool) -> &mut ParserBuilder
```

A builder for a regular expression parser.

This builder permits modifying configuration options for the parser.

This type combines the builder options for both the [AST
`ParserBuilder`](ast::parse::ParserBuilder) and the [HIR
`TranslatorBuilder`](hir::translate::TranslatorBuilder).

---
