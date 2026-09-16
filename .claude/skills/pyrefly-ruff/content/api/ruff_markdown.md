# `ruff_markdown`

Crate `ruff_markdown` · 2 public items · structured records in [`model/ruff_markdown.json`](../model/ruff_markdown.json)

## MarkdownResult

`enum` · `ruff_markdown::MarkdownResult`

```rust
enum MarkdownResult
```

**Variants**: `Formatted`, `Unchanged`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

---

## format_code_blocks

`function` · `ruff_markdown::format_code_blocks`

```rust
fn format_code_blocks(source: &str, path: Option<&std::path::Path>, settings: &ruff_workspace::FormatterSettings) -> MarkdownResult
```

---
