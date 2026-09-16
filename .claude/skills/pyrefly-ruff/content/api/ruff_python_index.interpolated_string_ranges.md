# `ruff_python_index::interpolated_string_ranges`

Crate `ruff_python_index` · 1 public items · structured records in [`model/ruff_python_index.interpolated_string_ranges.json`](../model/ruff_python_index.interpolated_string_ranges.json)

## InterpolatedStringRanges

`struct` · `ruff_python_index::interpolated_string_ranges::InterpolatedStringRanges`

```rust
struct InterpolatedStringRanges
```

Stores the ranges of all interpolated strings in a file sorted by [`TextRange::start`].
There can be multiple overlapping ranges for nested interpolated strings.

Note that the ranges for all unterminated interpolated strings are not stored.

---
