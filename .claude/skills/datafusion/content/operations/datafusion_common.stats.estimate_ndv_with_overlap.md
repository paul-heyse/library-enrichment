# `datafusion_common::stats::estimate_ndv_with_overlap`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.stats.estimate_ndv_with_overlap.json).

<a id="op-d4db5800523d8e40ea42dcf1"></a>
## estimate_ndv_with_overlap

`function` · `datafusion_common::stats::estimate_ndv_with_overlap` · datafusion-common 55.1.0

```rust
fn estimate_ndv_with_overlap(left: &ColumnStatistics, right: &ColumnStatistics, ndv_left: usize, ndv_right: usize) -> Option<usize>
```

Source: `src/stats.rs:831`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Estimates the combined number of distinct values (NDV) when merging two
column statistics, using range overlap to avoid double-counting shared values.

Assumes values are distributed uniformly within each input's
`[min, max]` range (the standard assumption when only summary
statistics are available). Under uniformity the fraction of an input's
distinct values that land in a sub-range equals the fraction of
the range that sub-range covers.

The combined value space is split into three disjoint regions:

```text
  |-- only A --|-- overlap --|-- only B --|
```

* **Only in A/B** - values outside the other input's range
  contribute `(1 - overlap_a) * NDV_a` and `(1 - overlap_b) * NDV_b`.
* **Overlap** - both inputs may produce values here. We take
  `max(overlap_a * NDV_a, overlap_b * NDV_b)` rather than the
  sum because values in the same sub-range are likely shared
  (the smaller set is assumed to be a subset of the larger).

The formula ranges between `[max(NDV_a, NDV_b), NDV_a + NDV_b]`,
from full overlap to no overlap.

```text
NDV = max(overlap_a * NDV_a, overlap_b * NDV_b)   [intersection]
    + (1 - overlap_a) * NDV_a                      [only in A]
    + (1 - overlap_b) * NDV_b                      [only in B]
```

Returns `None` when min/max are absent or distance is unsupported
(e.g. strings), in which case the caller should fall back to a simpler
estimate.
