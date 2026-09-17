# `regex_automata::util::syntax`

Crate `regex-automata` · 5 public items · structured records in [`model/regex_automata.util.syntax.json`](../model/regex_automata.util.syntax.json)

## parse

`function` · `regex_automata::util::syntax::parse`

```rust
fn parse(pattern: &str) -> Result<regex_syntax::hir::Hir, regex_syntax::Error>
```

A convenience routine for parsing a pattern into an HIR value with the
default configuration.

# Example

This shows how to parse a pattern into an HIR value:

```
use regex_automata::util::syntax;

let hir = syntax::parse(r"([a-z]+)|([0-9]+)")?;
assert_eq!(Some(1), hir.properties().static_explicit_captures_len());

# Ok::<(), Box<dyn std::error::Error>>(())
```

---

## parse_many

`function` · `regex_automata::util::syntax::parse_many`

```rust
fn parse_many<P: AsRef<str>>(patterns: &[P]) -> Result<alloc::vec::Vec<regex_syntax::hir::Hir>, regex_syntax::Error>
```

A convenience routine for parsing many patterns into HIR value with the
default configuration.

# Example

This shows how to parse many patterns into an corresponding HIR values:

```
use {
    regex_automata::util::syntax,
    regex_syntax::hir::Properties,
};

let hirs = syntax::parse_many(&[
    r"([a-z]+)|([0-9]+)",
    r"foo(A-Z]+)bar",
])?;
let props = Properties::union(hirs.iter().map(|h| h.properties()));
assert_eq!(Some(1), props.static_explicit_captures_len());

# Ok::<(), Box<dyn std::error::Error>>(())
```

---

## parse_many_with

`function` · `regex_automata::util::syntax::parse_many_with`

```rust
fn parse_many_with<P: AsRef<str>>(patterns: &[P], config: &Config) -> Result<alloc::vec::Vec<regex_syntax::hir::Hir>, regex_syntax::Error>
```

A convenience routine for parsing many patterns into HIR values using a
`Config`.

# Example

This shows how to parse many patterns into an corresponding HIR values
with a non-default configuration:

```
use {
    regex_automata::util::syntax,
    regex_syntax::hir::Properties,
};

let patterns = &[
    r"([a-z]+)|([0-9]+)",
    r"\W",
    r"foo(A-Z]+)bar",
];
let config = syntax::Config::new().unicode(false).utf8(false);
let hirs = syntax::parse_many_with(patterns, &config)?;
let props = Properties::union(hirs.iter().map(|h| h.properties()));
assert!(!props.is_utf8());

# Ok::<(), Box<dyn std::error::Error>>(())
```

---

## parse_with

`function` · `regex_automata::util::syntax::parse_with`

```rust
fn parse_with(pattern: &str, config: &Config) -> Result<regex_syntax::hir::Hir, regex_syntax::Error>
```

A convenience routine for parsing a pattern into an HIR value using a
`Config`.

# Example

This shows how to parse a pattern into an HIR value with a non-default
configuration:

```
use regex_automata::util::syntax;

let hir = syntax::parse_with(
    r"^[a-z]+$",
    &syntax::Config::new().multi_line(true).crlf(true),
)?;
assert!(hir.properties().look_set().contains_anchor_crlf());

# Ok::<(), Box<dyn std::error::Error>>(())
```

---

## Config

`struct` · `regex_automata::util::syntax::Config`

```rust
struct Config
```

**Derives**: Clone, Copy, Debug, Default

**Methods** (23)

```rust
fn case_insensitive(self, yes: bool) -> Config
fn crlf(self, yes: bool) -> Config
fn dot_matches_new_line(self, yes: bool) -> Config
fn get_case_insensitive(&self) -> bool
fn get_crlf(&self) -> bool
fn get_dot_matches_new_line(&self) -> bool
fn get_ignore_whitespace(&self) -> bool
fn get_line_terminator(&self) -> u8
fn get_multi_line(&self) -> bool
fn get_nest_limit(&self) -> u32
fn get_octal(&self) -> bool
fn get_swap_greed(&self) -> bool
fn get_unicode(&self) -> bool
fn get_utf8(&self) -> bool
fn ignore_whitespace(self, yes: bool) -> Config
fn line_terminator(self, byte: u8) -> Config
fn multi_line(self, yes: bool) -> Config
fn nest_limit(self, limit: u32) -> Config
fn new() -> Config
fn octal(self, yes: bool) -> Config
fn swap_greed(self, yes: bool) -> Config
fn unicode(self, yes: bool) -> Config
fn utf8(self, yes: bool) -> Config
```

A common set of configuration options that apply to the syntax of a regex.

This represents a group of configuration options that specifically apply
to how the concrete syntax of a regular expression is interpreted. In
particular, they are generally forwarded to the
[`ParserBuilder`](https://docs.rs/regex-syntax/*/regex_syntax/struct.ParserBuilder.html)
in the
[`regex-syntax`](https://docs.rs/regex-syntax)
crate when building a regex from its concrete syntax directly.

These options are defined as a group since they apply to every regex engine
in this crate. Instead of re-defining them on every engine's builder, they
are instead provided here as one cohesive unit.

---
