# `ruff_source_file::line_ranges`

Crate `ruff_source_file` · 1 public items · structured records in [`model/ruff_source_file.line_ranges.json`](../model/ruff_source_file.line_ranges.json)

## LineRanges

`trait` · `ruff_source_file::line_ranges::LineRanges`

Also reachable as `ruff_source_file::LineRanges`

```rust
trait LineRanges
```

**Implementors** (1)

- `ruff_linter::locator::Locator`

**Methods** (15)

```rust
fn bom_start_offset(&self) -> TextSize
fn contains_line_break(&self, range: TextRange) -> bool
fn count_lines(&self, range: TextRange) -> u32
fn full_line_end(&self, offset: TextSize) -> TextSize
fn full_line_range(&self, offset: TextSize) -> TextRange
fn full_line_str(&self, offset: TextSize) -> &str
fn full_lines_range(&self, range: TextRange) -> TextRange
fn full_lines_str(&self, range: TextRange) -> &str
fn is_at_start_of_line(&self, offset: TextSize) -> bool
fn line_end(&self, offset: TextSize) -> TextSize
fn line_range(&self, offset: TextSize) -> TextRange
fn line_start(&self, offset: TextSize) -> TextSize
fn line_str(&self, offset: TextSize) -> &str
fn lines_range(&self, range: TextRange) -> TextRange
fn lines_str(&self, range: TextRange) -> &str
```

Extension trait for [`str`] that provides methods for working with ranges of lines.

---
