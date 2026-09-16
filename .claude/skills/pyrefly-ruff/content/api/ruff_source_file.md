# `ruff_source_file`

Crate `ruff_source_file` · 6 public items · structured records in [`model/ruff_source_file.json`](../model/ruff_source_file.json)

## SourceRow

`enum` · `ruff_source_file::SourceRow`

```rust
enum SourceRow
```

**Variants**: `Notebook`, `SourceFile`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

---

## LineColumn

`struct` · `ruff_source_file::LineColumn`

```rust
struct LineColumn
```

**Fields**: `line`, `column`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

The line and column of an offset in a source file.

See [`LineIndex::line_column`] for more information.

---

## SourceCode

`struct` · `ruff_source_file::SourceCode`

```rust
struct SourceCode<'src, 'index>
```

**Derives**: Debug, Eq, PartialEq

**Methods** (11)

```rust
fn line_column(&self, offset: TextSize) -> LineColumn
fn line_count(&self) -> usize
fn line_end(&self, line: OneIndexed) -> TextSize
fn line_end_exclusive(&self, line: OneIndexed) -> TextSize
fn line_index(&self, offset: TextSize) -> OneIndexed
fn line_start(&self, line: OneIndexed) -> TextSize
fn line_text(&self, index: OneIndexed) -> &'src str
fn new(content: &'src str, index: &'index LineIndex) -> Self
fn slice<T: Ranged>(&self, ranged: T) -> &'src str
fn source_location(&self, offset: TextSize, position_encoding: PositionEncoding) -> SourceLocation
fn text(&self) -> &'src str
```

Gives access to the source code of a file and allows mapping between [`TextSize`] and [`LineColumn`].

---

## SourceFile

`struct` · `ruff_source_file::SourceFile`

```rust
struct SourceFile
```

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn index(&self) -> &LineIndex
fn name(&self) -> &str
fn source_text(&self) -> &str
fn to_source_code(&self) -> SourceCode<'_, '_>
```

A source file that is identified by its name. Optionally stores the source code and [`LineIndex`].

Cloning a [`SourceFile`] is cheap, because it only requires bumping a reference count.

---

## SourceFileBuilder

`struct` · `ruff_source_file::SourceFileBuilder`

```rust
struct SourceFileBuilder
```

**Methods** (2)

```rust
fn finish(self) -> SourceFile
fn new<Name: Into<Box<str>>, Code: Into<Box<str>>>(name: Name, code: Code) -> Self
```

A Builder for constructing a [`SourceFile`]

---

## SourceLocation

`struct` · `ruff_source_file::SourceLocation`

```rust
struct SourceLocation
```

**Fields**: `line`, `character_offset`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

A position into a source file represented by the line number and the offset to that character relative to the start of that line.

---
