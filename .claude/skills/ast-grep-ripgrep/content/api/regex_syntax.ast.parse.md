# `regex_syntax::ast::parse`

Crate `regex-syntax` · 2 public items · structured records in [`model/regex_syntax.ast.parse.json`](../model/regex_syntax.ast.parse.json)

## Parser

`struct` · `regex_syntax::ast::parse::Parser`

```rust
struct Parser
```

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn new() -> Parser
fn parse(&mut self, pattern: &str) -> core::result::Result<Ast, ast::Error>
fn parse_with_comments(&mut self, pattern: &str) -> core::result::Result<ast::WithComments, ast::Error>
```

A regular expression parser.

This parses a string representation of a regular expression into an
abstract syntax tree. The size of the tree is proportional to the length
of the regular expression pattern.

A `Parser` can be configured in more detail via a [`ParserBuilder`].

---

## ParserBuilder

`struct` · `regex_syntax::ast::parse::ParserBuilder`

```rust
struct ParserBuilder
```

**Derives**: Clone, Debug, Default

**Methods** (6)

```rust
fn build(&self) -> Parser
fn empty_min_range(&mut self, yes: bool) -> &mut ParserBuilder
fn ignore_whitespace(&mut self, yes: bool) -> &mut ParserBuilder
fn nest_limit(&mut self, limit: u32) -> &mut ParserBuilder
fn new() -> ParserBuilder
fn octal(&mut self, yes: bool) -> &mut ParserBuilder
```

A builder for a regular expression parser.

This builder permits modifying configuration options for the parser.

---
