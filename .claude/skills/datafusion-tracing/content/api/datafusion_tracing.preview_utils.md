# `datafusion_tracing::preview_utils`

Crate `datafusion-tracing` · 3 public items · structured records in [`model/datafusion_tracing.preview_utils.json`](../model/datafusion_tracing.preview_utils.json)

## DEFAULT_PRESET

`constant` · `datafusion_tracing::preview_utils::DEFAULT_PRESET`

```rust
const DEFAULT_PRESET: &str = "||--|=+||-+||++++++"
```

---

## TRUNCATED_PRESET

`constant` · `datafusion_tracing::preview_utils::TRUNCATED_PRESET`

```rust
const TRUNCATED_PRESET: &str = "|…--|=+…|-+|…+++…+…"
```

---

## pretty_format_compact_batch

`function` · `datafusion_tracing::preview_utils::pretty_format_compact_batch`

Also reachable as `datafusion_tracing::pretty_format_compact_batch`

```rust
fn pretty_format_compact_batch(batch: &datafusion::arrow::array::RecordBatch, max_width: usize, max_row_height: usize, min_compacted_col_width: usize) -> Result<impl Display, datafusion::arrow::error::ArrowError>
```

Formats a `RecordBatch` as a neatly aligned ASCII table,
constraining the total width to `max_width`. Columns are
dynamically resized or truncated, and columns that cannot
fit within the given width may be dropped.

---
