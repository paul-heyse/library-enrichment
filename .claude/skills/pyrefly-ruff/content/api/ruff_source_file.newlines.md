# `ruff_source_file::newlines`

Crate `ruff_source_file` · 6 public items · structured records in [`model/ruff_source_file.newlines.json`](../model/ruff_source_file.newlines.json)

## LineEnding

`enum` · `ruff_source_file::newlines::LineEnding`

Also reachable as `ruff_source_file::LineEnding`

```rust
enum LineEnding
```

**Variants**: `Lf`, `Cr`, `CrLf`

**Implements**: `core::ops::deref::Deref`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
const fn as_str(&self) -> &'static str
const fn len(&self) -> usize
const fn text_len(&self) -> TextSize
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

The line ending style used in Python source code.
See <https://docs.python.org/3/reference/lexical_analysis.html#physical-lines>

---

## find_newline

`function` · `ruff_source_file::newlines::find_newline`

Also reachable as `ruff_source_file::find_newline`

```rust
fn find_newline(text: &str) -> Option<(usize, LineEnding)>
```

Finds the next newline character. Returns its position and the [`LineEnding`].

---

## Line

`struct` · `ruff_source_file::newlines::Line`

Also reachable as `ruff_source_file::Line`

```rust
struct Line<'a>
```

**Implements**: `core::ops::deref::Deref`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (9)

```rust
fn as_full_str(&self) -> &'a str
fn as_str(&self) -> &'a str
fn end(&self) -> TextSize
fn full_end(&self) -> TextSize
fn full_range(&self) -> TextRange
fn line_ending(&self) -> Option<LineEnding>
fn new(text: &'a str, offset: TextSize) -> Self
fn range(&self) -> TextRange
const fn start(&self) -> TextSize
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

---

## NewlineWithTrailingNewline

`struct` · `ruff_source_file::newlines::NewlineWithTrailingNewline`

Also reachable as `ruff_source_file::NewlineWithTrailingNewline`

```rust
struct NewlineWithTrailingNewline<'a>
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::iterator::Iterator`

**Methods** (2)

```rust
fn from(input: &'a str) -> NewlineWithTrailingNewline<'a>
fn with_offset(input: &'a str, offset: TextSize) -> Self
```

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> Option<Self::Item>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

Like [`UniversalNewlineIterator`], but includes a trailing newline as an empty line.

---

## UniversalNewlineIterator

`struct` · `ruff_source_file::newlines::UniversalNewlineIterator`

Also reachable as `ruff_source_file::UniversalNewlineIterator`

```rust
struct UniversalNewlineIterator<'a>
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone

**Methods** (2)

```rust
fn from(text: &'a str) -> UniversalNewlineIterator<'a>
fn with_offset(text: &'a str, offset: TextSize) -> UniversalNewlineIterator<'a>
```

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> Option<Self::Item>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn last(self) -> Option<Self::Item>
fn next(&mut self) -> Option<Line<'a>>
```

Like [`str::lines`], but accommodates LF, CRLF, and CR line endings,
the latter of which are not supported by [`str::lines`].

## Examples

```rust
# use ruff_text_size::TextSize;
# use ruff_source_file::{Line, UniversalNewlineIterator};
let mut lines = UniversalNewlineIterator::from("foo\nbar\n\r\nbaz\rbop");

assert_eq!(lines.next_back(), Some(Line::new("bop", TextSize::from(14))));
assert_eq!(lines.next(), Some(Line::new("foo\n", TextSize::from(0))));
assert_eq!(lines.next_back(), Some(Line::new("baz\r", TextSize::from(10))));
assert_eq!(lines.next(), Some(Line::new("bar\n", TextSize::from(4))));
assert_eq!(lines.next_back(), Some(Line::new("\r\n", TextSize::from(8))));
assert_eq!(lines.next(), None);
```

---

## UniversalNewlines

`trait` · `ruff_source_file::newlines::UniversalNewlines`

Also reachable as `ruff_source_file::UniversalNewlines`

```rust
trait UniversalNewlines
```

**Methods** (1)

```rust
fn universal_newlines(&self) -> UniversalNewlineIterator<'_>
```

Extension trait for [`str`] that provides a [`UniversalNewlineIterator`].

---
