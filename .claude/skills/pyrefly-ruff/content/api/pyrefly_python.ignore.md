# `pyrefly_python::ignore`

Crate `pyrefly_python` · 9 public items · structured records in [`model/pyrefly_python.ignore.json`](../model/pyrefly_python.ignore.json)

## SuppressionEffect

`enum` · `pyrefly_python::ignore::SuppressionEffect`

```rust
enum SuppressionEffect
```

**Variants**: `None`, `DowngradeToWarning`, `Suppress`

**Derives**: Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, StructuralPartialEq

The effect a suppression has on a diagnostic. The derived `Ord` ordering
(`None` < `DowngradeToWarning` < `Suppress`) encodes suppression precedence:
call sites combine the effects of multiple applicable suppressions with
`.max()` to pick the strongest one, so the variant order here is load-bearing
and must remain weakest-to-strongest.

---

## Tool

`enum` · `pyrefly_python::ignore::Tool`

```rust
enum Tool
```

**Variants**: `Type`, `Pyrefly`, `Pyright`, `Mypy`, `Ty`, `Pyre`, `Zuban`

**Implements**: `clap_builder::derive::ValueEnum`, `dupe::Dupe`, `enum_iterator::Sequence`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn all() -> SmallSet<Self>
fn default_enabled() -> SmallSet<Self>
```

**via `clap_builder::derive::ValueEnum`**

```rust
fn to_possible_value<'a>(&self) -> ::std::option::Option<clap::builder::PossibleValue>
fn value_variants<'a>() -> &'a [Self]
```

**via `enum_iterator::Sequence`**

```rust
fn first() -> ::core::option::Option<Self>
fn last() -> ::core::option::Option<Self>
fn next(&self) -> ::core::option::Option<Self>
fn previous(&self) -> ::core::option::Option<Self>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

The name of the tool that is being suppressed.
Note that the variant names and docstrings are displayed in `pyrefly check --help`.

---

## TypeIgnoreUnknownTagBehavior

`enum` · `pyrefly_python::ignore::TypeIgnoreUnknownTagBehavior`

```rust
enum TypeIgnoreUnknownTagBehavior
```

**Variants**: `NoEffect`, `DowngradeToWarning`, `Suppress`

**Implements**: `clap_builder::derive::ValueEnum`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `clap_builder::derive::ValueEnum`**

```rust
fn to_possible_value<'a>(&self) -> ::std::option::Option<clap::builder::PossibleValue>
fn value_variants<'a>() -> &'a [Self]
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## find_comment_start

`function` · `pyrefly_python::ignore::find_comment_start`

```rust
fn find_comment_start(line: &str, in_triple_quote: Option<char>) -> (Option<usize>, Option<char>)
```

Finds the byte offset of the first '#' character that starts a comment, tracking
whether we're inside a multi-line triple-quoted string.

All interesting characters (`#`, `'`, `"`, `\`) are ASCII, so we operate
on bytes directly — UTF-8 guarantees these never appear inside multi-byte
sequences.

`in_triple_quote` should be `Some('"')` or `Some('\'')` if the line begins
inside an open triple-quoted string from a previous line, or `None` otherwise.

Returns `(comment_start, new_triple_quote_state)`.

---

## find_comment_start_in_line

`function` · `pyrefly_python::ignore::find_comment_start_in_line`

```rust
fn find_comment_start_in_line(line: &str) -> Option<usize>
```

Finds the byte offset of the first '#' character that starts a comment.
Returns None if no comment is found or if all '#' are inside strings.
Handles escape sequences, single/double quotes, and triple-quoted strings.

This is string-aware parsing that avoids treating '#' inside strings as comments.
For example: `x = "hello # world"  # real comment` correctly identifies the second '#'.

---

## misplaced_ignore_errors

`function` · `pyrefly_python::ignore::misplaced_ignore_errors`

```rust
fn misplaced_ignore_errors(code: &str, multiline_string_ranges: &[(pyrefly_util::lined_buffer::LineNumber, pyrefly_util::lined_buffer::LineNumber)]) -> Vec<pyrefly_util::lined_buffer::LineNumber>
```

Find the lines of pyrefly `ignore-errors` directives that appear *after* the
preamble, where a file-level suppression is silently inert.

The preamble is the leading run of blank lines, comments, and docstrings.
A directive there is honored by `parse_ignore_all`, so it is not reported.
Once the first real code line is seen, every subsequent comment-only line is
checked for a pyrefly `ignore-errors` directive (blanket or typed). Line
classification mirrors `parse_ignore_all` so the two functions partition
directives into "honored" (preamble) and "misplaced" (after code).

---

## parse_ignore_all

`function` · `pyrefly_python::ignore::parse_ignore_all`

```rust
fn parse_ignore_all(code: &str, multiline_string_ranges: &[(pyrefly_util::lined_buffer::LineNumber, pyrefly_util::lined_buffer::LineNumber)]) -> Vec<Suppression>
```

Parse top-level `ignore-errors` / `ignore-all-errors` / `type: ignore` directives.

Scans the beginning of the file for comment-only lines (including blank lines
and lines inside multiline strings like docstrings). Returns the file-level
suppressions found; Pyrefly entries may carry specific error codes
(`# pyrefly: ignore-errors[code]`), while other tools are blanket-only.

After a docstring, only `ignore-errors` directives are recognized — bare
`# type: ignore` is not, since it could plausibly be meant as a per-line
suppression for code that follows.

---

## Ignore

`struct` · `pyrefly_python::ignore::Ignore`

```rust
struct Ignore
```

**Derives**: Clone, Debug, Default

**Methods** (8)

```rust
fn get(&self, line: &LineNumber) -> Option<&Vec<Suppression>>
fn get_pyrefly_ignores(&self, all: bool) -> SmallSet<LineNumber>
fn is_empty(&self) -> bool
fn is_ignored(&self, start_line: LineNumber, kind: &str, enabled_ignores: &SmallSet<Tool>) -> bool
fn is_ignored_by_suppression_line(&self, suppression_line: LineNumber, start_line: LineNumber, end_line: LineNumber, kind: &str, enabled_ignores: &SmallSet<Tool>, type_ignore_unknown_tag_behavior: TypeIgnoreUnknownTagBehavior) -> bool
fn iter(&self) -> impl Iterator<Item = (&LineNumber, &Vec<Suppression>)>
fn new(code: &str) -> Self
fn suppression_effect(&self, start_line: LineNumber, kind: &str, enabled_ignores: &SmallSet<Tool>, type_ignore_unknown_tag_behavior: TypeIgnoreUnknownTagBehavior) -> SuppressionEffect
```

Record the position of lines affected by ignore suppressions.

---

## Suppression

`struct` · `pyrefly_python::ignore::Suppression`

```rust
struct Suppression
```

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn comment_line(&self) -> LineNumber
fn comment_offset(&self) -> usize
fn error_codes(&self) -> &[String]
fn tool(&self) -> Tool
```

---
