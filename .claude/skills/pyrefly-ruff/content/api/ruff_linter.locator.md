# `ruff_linter::locator`

Crate `ruff_linter` · 1 public items · structured records in [`model/ruff_linter.locator.json`](../model/ruff_linter.locator.json)

## Locator

`struct` · `ruff_linter::locator::Locator`

Also reachable as `ruff_linter::Locator`

```rust
struct Locator<'a>
```

**Implements**: `ruff_source_file::line_ranges::LineRanges`

**Derives**: Debug

**Methods** (5)

```rust
const fn contents(&self) -> &'a str
fn floor_char_boundary(&self, offset: TextSize) -> TextSize
const fn new(contents: &'a str) -> Self
fn to_index(&self) -> &LineIndex
fn to_source_code(&self) -> SourceCode<'_, '_>
```

**via `ruff_source_file::line_ranges::LineRanges`**

```rust
fn bom_start_offset(&self) -> TextSize
fn contains_line_break(&self, range: TextRange) -> bool
fn full_line_end(&self, offset: TextSize) -> TextSize
fn full_line_str(&self, offset: TextSize) -> &str
fn full_lines_str(&self, range: TextRange) -> &str
fn line_end(&self, offset: TextSize) -> TextSize
fn line_start(&self, offset: TextSize) -> TextSize
fn line_str(&self, offset: TextSize) -> &str
fn lines_str(&self, range: TextRange) -> &str
```

---
