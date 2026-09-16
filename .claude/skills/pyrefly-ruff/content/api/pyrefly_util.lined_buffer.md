# `pyrefly_util::lined_buffer`

Crate `pyrefly_util` · 5 public items · structured records in [`model/pyrefly_util.lined_buffer.json`](../model/pyrefly_util.lined_buffer.json)

## DisplayPos

`enum` · `pyrefly_util::lined_buffer::DisplayPos`

```rust
enum DisplayPos
```

**Variants**: `Source`, `Notebook`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn cell(self) -> Option<NonZeroU32>
fn column(self) -> NonZeroU32
fn line_within_cell(self) -> LineNumber
fn line_within_file(self) -> LineNumber
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

The line and column of an offset in a source file.

---

## DisplayRange

`struct` · `pyrefly_util::lined_buffer::DisplayRange`

```rust
struct DisplayRange
```

**Fields**: `start`, `end`

**Implements**: `core::fmt::Display`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer
```

A range in a file, with a start and end, both containing line and column.
Stored in terms of characters, not including any BOM.

---

## LineNumber

`struct` · `pyrefly_util::lined_buffer::LineNumber`

```rust
struct LineNumber
```

**Implements**: `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (8)

```rust
fn decrement(&self) -> Option<Self>
fn from_one_indexed(x: OneIndexed) -> Self
fn from_zero_indexed(x: u32) -> Self
fn get(self) -> u32
fn increment(self) -> Self
fn new(x: u32) -> Option<Self>
fn to_one_indexed(self) -> OneIndexed
fn to_zero_indexed(self) -> u32
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

A line number in a file.

---

## LinedBuffer

`struct` · `pyrefly_util::lined_buffer::LinedBuffer`

```rust
struct LinedBuffer
```

**Derives**: Clone, Debug

**Methods** (19)

```rust
fn clamp_position(&self, offset: TextSize) -> TextSize
fn code_at(&self, range: TextRange) -> &str
fn content_in_line_range(&self, start_line: LineNumber, end_line: LineNumber) -> &str
fn contents(&self) -> &Arc<String>
fn display_pos(&self, offset: TextSize, notebook: Option<&Notebook>) -> DisplayPos
fn display_range(&self, range: TextRange, notebook: Option<&Notebook>) -> DisplayRange
fn from_display_pos(&self, pos: DisplayPos) -> TextSize
fn from_display_range(&self, source_range: &DisplayRange) -> TextRange
fn from_lsp_position(&self, position: lsp_types::Position, notebook_and_cell: Option<(&Notebook, usize)>) -> TextSize
fn from_lsp_range(&self, position: lsp_types::Range, notebook_and_cell: Option<(&Notebook, usize)>) -> TextRange
fn line_count(&self) -> usize
fn line_index(&self) -> &LineIndex
fn line_start(&self, line: LineNumber) -> TextSize
fn lines(&self) -> Lines<'_>
fn new(buffer: Arc<String>) -> Self
fn python_ast_range_for_expr(&self, original_range: TextRange, expr: &Expr, parent_expr: Option<&Expr>) -> PythonASTRange
fn to_cell_for_lsp(&self, x: TextSize, notebook: Option<&Notebook>) -> Option<usize>
fn to_lsp_position(&self, x: TextSize, notebook: Option<&Notebook>) -> lsp_types::Position
fn to_lsp_range(&self, x: TextRange, notebook: Option<&Notebook>) -> lsp_types::Range
```

---

## PythonASTRange

`struct` · `pyrefly_util::lined_buffer::PythonASTRange`

```rust
struct PythonASTRange
```

**Fields**: `start_line`, `start_col`, `end_line`, `end_col`

**Implements**: `serde_core::ser::Serialize`

**Derives**: Clone, Debug

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer
```

Source location in Python AST conventions: 1-indexed lines, 0-indexed columns.

Matches the `lineno`/`col_offset`/`end_lineno`/`end_col_offset` fields that
CPython's `ast` module exposes on expression nodes.

---
