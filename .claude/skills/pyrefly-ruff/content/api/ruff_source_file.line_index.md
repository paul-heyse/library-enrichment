# `ruff_source_file::line_index`

Crate `ruff_source_file` · 3 public items · structured records in [`model/ruff_source_file.line_index.json`](../model/ruff_source_file.line_index.json)

## PositionEncoding

`enum` · `ruff_source_file::line_index::PositionEncoding`

Also reachable as `ruff_source_file::PositionEncoding`

```rust
enum PositionEncoding
```

**Variants**: `Utf8`, `Utf16`, `Utf32`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug

---

## LineIndex

`struct` · `ruff_source_file::line_index::LineIndex`

Also reachable as `ruff_source_file::LineIndex`

```rust
struct LineIndex
```

**Implements**: `core::ops::deref::Deref`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (11)

```rust
fn from_source_text(text: &str) -> Self
fn line_column(&self, offset: TextSize, content: &str) -> LineColumn
fn line_count(&self) -> usize
fn line_end(&self, line: OneIndexed, contents: &str) -> TextSize
fn line_index(&self, offset: TextSize) -> OneIndexed
fn line_len(&self, line: OneIndexed, text: &str, encoding: PositionEncoding) -> usize
fn line_range(&self, line: OneIndexed, contents: &str) -> TextRange
fn line_start(&self, line: OneIndexed, contents: &str) -> TextSize
fn line_starts(&self) -> &[TextSize]
fn offset(&self, position: SourceLocation, text: &str, position_encoding: PositionEncoding) -> TextSize
fn source_location(&self, offset: TextSize, text: &str, encoding: PositionEncoding) -> SourceLocation
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

Index for fast [byte offset](TextSize) to [`LineColumn`] conversions.

Cloning a [`LineIndex`] is cheap because it only requires bumping a reference count.

---

## OneIndexed

`struct` · `ruff_source_file::line_index::OneIndexed`

Also reachable as `ruff_source_file::OneIndexed`

```rust
struct OneIndexed
```

**Implements**: `core::fmt::Display`, `core::str::traits::FromStr`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (9)

```rust
fn checked_add(self, rhs: Self) -> Option<Self>
fn checked_sub(self, rhs: Self) -> Option<Self>
const fn digits(self) -> NonZeroUsize
const fn from_zero_indexed(value: usize) -> Self
const fn get(self) -> usize
const fn new(value: usize) -> Option<Self>
const fn saturating_add(self, rhs: usize) -> Self
const fn saturating_sub(self, rhs: usize) -> Self
const fn to_zero_indexed(self) -> usize
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

Type-safe wrapper for a value whose logical range starts at `1`, for
instance the line or column numbers in a file

Internally this is represented as a [`NonZeroUsize`], this enables some
memory optimizations

---
