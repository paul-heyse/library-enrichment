# `regex::builders::string`

Crate `regex` · 2 public items · structured records in [`model/regex.builders.string.json`](../model/regex.builders.string.json)

## RegexBuilder

`struct` · `regex::builders::string::RegexBuilder`

```rust
struct RegexBuilder
```

**Derives**: Clone, Debug

**Methods** (14)

```rust
fn build(&self) -> Result<Regex, Error>
fn case_insensitive(&mut self, yes: bool) -> &mut RegexBuilder
fn crlf(&mut self, yes: bool) -> &mut RegexBuilder
fn dfa_size_limit(&mut self, bytes: usize) -> &mut RegexBuilder
fn dot_matches_new_line(&mut self, yes: bool) -> &mut RegexBuilder
fn ignore_whitespace(&mut self, yes: bool) -> &mut RegexBuilder
fn line_terminator(&mut self, byte: u8) -> &mut RegexBuilder
fn multi_line(&mut self, yes: bool) -> &mut RegexBuilder
fn nest_limit(&mut self, limit: u32) -> &mut RegexBuilder
fn new(pattern: &str) -> RegexBuilder
fn octal(&mut self, yes: bool) -> &mut RegexBuilder
fn size_limit(&mut self, bytes: usize) -> &mut RegexBuilder
fn swap_greed(&mut self, yes: bool) -> &mut RegexBuilder
fn unicode(&mut self, yes: bool) -> &mut RegexBuilder
```

A configurable builder for a [`Regex`].

This builder can be used to programmatically set flags such as `i`
(case insensitive) and `x` (for verbose mode). This builder can also be
used to configure things like the line terminator and a size limit on
the compiled regular expression.

---

## RegexSetBuilder

`struct` · `regex::builders::string::RegexSetBuilder`

```rust
struct RegexSetBuilder
```

**Derives**: Clone, Debug

**Methods** (14)

```rust
fn build(&self) -> Result<RegexSet, Error>
fn case_insensitive(&mut self, yes: bool) -> &mut RegexSetBuilder
fn crlf(&mut self, yes: bool) -> &mut RegexSetBuilder
fn dfa_size_limit(&mut self, bytes: usize) -> &mut RegexSetBuilder
fn dot_matches_new_line(&mut self, yes: bool) -> &mut RegexSetBuilder
fn ignore_whitespace(&mut self, yes: bool) -> &mut RegexSetBuilder
fn line_terminator(&mut self, byte: u8) -> &mut RegexSetBuilder
fn multi_line(&mut self, yes: bool) -> &mut RegexSetBuilder
fn nest_limit(&mut self, limit: u32) -> &mut RegexSetBuilder
fn new<I, S>(patterns: I) -> RegexSetBuilder where I: IntoIterator<Item = S>, S: AsRef<str>
fn octal(&mut self, yes: bool) -> &mut RegexSetBuilder
fn size_limit(&mut self, bytes: usize) -> &mut RegexSetBuilder
fn swap_greed(&mut self, yes: bool) -> &mut RegexSetBuilder
fn unicode(&mut self, yes: bool) -> &mut RegexSetBuilder
```

A configurable builder for a [`RegexSet`].

This builder can be used to programmatically set flags such as
`i` (case insensitive) and `x` (for verbose mode). This builder
can also be used to configure things like the line terminator
and a size limit on the compiled regular expression.

---
