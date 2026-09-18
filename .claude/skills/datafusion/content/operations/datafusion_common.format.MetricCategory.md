# `datafusion_common::format::MetricCategory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.format.MetricCategory.json).

<a id="op-d05732d49bd39fe3321965e4"></a>
## MetricCategory

`enum` · `datafusion_common::format::MetricCategory` · datafusion-common 55.1.0

```rust
enum MetricCategory
```

Source: `src/format.rs:307`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Classifies a metric by what it measures.

This is orthogonal to [`MetricType`](../operations/datafusion_common.format.MetricType.md#op-fb6314931b98a471bb076826) (Summary / Dev), which controls
*verbosity*. `MetricCategory` controls *what kind of value* is shown,
so that `EXPLAIN ANALYZE` output can be narrowed to only the categories
that are useful in a given context.

In particular this is useful for testing since metrics differ in their stability across runs:
- [`Rows`](Self::Rows) and [`Bytes`](Self::Bytes) depend only on the plan
  and the data, so they are mostly deterministic across runs (given the same
  input). Variations can existing e.g. because of non-deterministic ordering
  of evaluation between threads.
  Running with a single target partition often makes these metrics stable enough to assert on in tests.
- [`Timing`](Self::Timing) depends on hardware, system load, scheduling,
  etc., so it varies from run to run even on the same machine.

[`MetricCategory`](../operations/datafusion_common.format.MetricCategory.md#op-d05732d49bd39fe3321965e4) is especially useful in sqllogictest (`.slt`) files:
setting `datafusion.explain.analyze_categories = 'rows'` lets a test
assert on row-count metrics without sprinkling `<slt:ignore>` over every
timing value.

Metrics that do not declare a category (the default for custom
`Count` / `Gauge` metrics) are treated as
[`Uncategorized`](Self::Uncategorized) for filtering purposes.

<a id="op-b033e3d48f8f8ea5e0bbf38d"></a>
## Bytes

`variant` · `datafusion_common::format::MetricCategory::Bytes` · datafusion-common 55.1.0

```rust
Bytes
```

Source: `src/format.rs:317`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Byte measurements: `output_bytes`, `spilled_bytes`,
`current_memory_usage`, `bytes_scanned`, etc.

Mostly deterministic given the same plan and data.

<a id="op-d58e8cd2c02467c9f4a1c57b"></a>
## Err

`assoc_type` · `datafusion_common::format::MetricCategory::Err` · datafusion-common 55.1.0

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [338, 1], "end": [353, 2], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/format.rs:339`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95b447f9a1b108e0c37644d3"></a>
## Rows

`variant` · `datafusion_common::format::MetricCategory::Rows` · datafusion-common 55.1.0

```rust
Rows
```

Source: `src/format.rs:312`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Row counts and related dimensionless counters: `output_rows`,
`spilled_rows`, `output_batches`, pruning metrics, ratios, etc.

Mostly deterministic given the same plan and data.

<a id="op-d1472eae3e9245e3e2f13372"></a>
## Timing

`variant` · `datafusion_common::format::MetricCategory::Timing` · datafusion-common 55.1.0

```rust
Timing
```

Source: `src/format.rs:323`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Wall-clock durations and timestamps: `elapsed_compute`,
operator-defined `Time` metrics, `start_timestamp` /
`end_timestamp`, etc.

**Non-deterministic** — varies across runs even on the same hardware.

<a id="op-fbd472b286dab3bd4d58ae32"></a>
## Uncategorized

`variant` · `datafusion_common::format::MetricCategory::Uncategorized` · datafusion-common 55.1.0

```rust
Uncategorized
```

Source: `src/format.rs:335`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Catch-all for metrics that do not fit into [`Rows`](Self::Rows),
[`Bytes`](Self::Bytes), or [`Timing`](Self::Timing).

Custom `Count` / `Gauge` metrics that are not explicitly assigned
a category are treated as `Uncategorized` for filtering purposes.

This variant lets users explicitly include or exclude these
metrics, e.g.:
```sql
SET datafusion.explain.analyze_categories = 'rows, bytes, uncategorized';
```

<a id="op-ec5ea2a75ba6f6b792624ae2"></a>
## clone

`function` · `datafusion_common::format::MetricCategory::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> MetricCategory
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [306, 17], "end": [306, 22], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/format.rs:306`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-267dcb7ec9e77a0b402e6d39"></a>
## eq

`function` · `datafusion_common::format::MetricCategory::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &MetricCategory) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [306, 30], "end": [306, 39], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/format.rs:306`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1028a446c5b05a851ce4e4dd"></a>
## fmt

`function` · `datafusion_common::format::MetricCategory::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [306, 10], "end": [306, 15], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/format.rs:306`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ddd466af8b90d98e6e410113"></a>
## fmt

`function` · `datafusion_common::format::MetricCategory::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [355, 1], "end": [364, 2], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/format.rs:356`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c5a625149fbd06977917da2"></a>
## from_str

`function` · `datafusion_common::format::MetricCategory::from_str` · datafusion-common 55.1.0

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [338, 1], "end": [353, 2], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/format.rs:341`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f81887b6efd02f5f37e4a9da"></a>
## hash

`function` · `datafusion_common::format::MetricCategory::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::MetricCategory", "path": "MetricCategory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [306, 45], "end": [306, 49], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/format.rs:306`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
