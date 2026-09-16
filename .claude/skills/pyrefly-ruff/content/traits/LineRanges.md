# LineRanges

`ruff_source_file::line_ranges::LineRanges`

```rust
trait LineRanges
```

Also reachable as `ruff_source_file::LineRanges`

Prose: [`api/ruff_source_file.line_ranges.md`](../api/ruff_source_file.line_ranges.md#lineranges) · records: [`model/ruff_source_file.line_ranges.json`](../model/ruff_source_file.line_ranges.json)

## Required

Every implementation must supply these.

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

## Provided

Defaulted, and this is where the capability hides. The default is the conservative answer -- no pushdown, no statistics, no specialization -- so an implementation that overrides none of these works correctly and performs badly.

```rust
fn count_lines(&self, range: TextRange) -> u32
fn full_line_range(&self, offset: TextSize) -> TextRange
fn full_lines_range(&self, range: TextRange) -> TextRange
fn is_at_start_of_line(&self, offset: TextSize) -> bool
fn line_range(&self, offset: TextSize) -> TextRange
fn lines_range(&self, range: TextRange) -> TextRange
```

## Implementors (1)

Read one before writing your own.

- `ruff_linter::locator::Locator`

## Documentation

Extension trait for [`str`] that provides methods for working with ranges of lines.
