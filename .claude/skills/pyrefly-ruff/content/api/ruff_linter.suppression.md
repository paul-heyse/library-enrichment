# `ruff_linter::suppression`

Crate `ruff_linter` · 2 public items · structured records in [`model/ruff_linter.suppression.json`](../model/ruff_linter.suppression.json)

## rule_identifier_range_at_offset

`function` · `ruff_linter::suppression::rule_identifier_range_at_offset`

```rust
fn rule_identifier_range_at_offset(source: &str, comment_range: ruff_text_size::TextRange, offset: ruff_text_size::TextSize) -> Option<ruff_text_size::TextRange>
```

Returns the range of the rule identifier at `offset` within a `noqa` or Ruff suppression
comment.

---

## Suppressions

`struct` · `ruff_linter::suppression::Suppressions`

```rust
struct Suppressions
```

**Derives**: Debug, Default

**Methods** (1)

```rust
fn from_tokens(source: &str, tokens: &Tokens, indexer: &Indexer, settings: &LinterSettings) -> Suppressions
```

---
