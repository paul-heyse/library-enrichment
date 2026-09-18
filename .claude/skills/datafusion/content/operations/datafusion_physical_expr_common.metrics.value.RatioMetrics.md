# `datafusion_physical_expr_common::metrics::value::RatioMetrics`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.metrics.value.RatioMetrics.json).

<a id="op-a47dbfa1bdc8f5bf2bf4eb22"></a>
## RatioMetrics

`struct` · `datafusion_physical_expr_common::metrics::value::RatioMetrics` · datafusion-physical-expr-common 55.1.0

```rust
struct RatioMetrics
```

Source: `src/metrics/value.rs:467`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Counters tracking ratio metrics (e.g. matched vs total)

The counters are thread-safe and shared across clones.

<a id="op-9f581faf954e762ab90218b9"></a>
## add_part

`function` · `datafusion_physical_expr_common::metrics::value::RatioMetrics::add_part` · datafusion-physical-expr-common 55.1.0

```rust
fn add_part(&self, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::RatioMetrics", "path": "RatioMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [562, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:505`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Add `n` to the numerator (`part`) value

<a id="op-4128b1e79ba012ae174e363d"></a>
## add_total

`function` · `datafusion_physical_expr_common::metrics::value::RatioMetrics::add_total` · datafusion-physical-expr-common 55.1.0

```rust
fn add_total(&self, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::RatioMetrics", "path": "RatioMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [562, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:510`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Add `n` to the denominator (`total`) value

<a id="op-36dbc0287c190fd9804ced7d"></a>
## clone

`function` · `datafusion_physical_expr_common::metrics::value::RatioMetrics::clone` · datafusion-physical-expr-common 55.1.0

```rust
fn clone(&self) -> RatioMetrics
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::RatioMetrics", "path": "RatioMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [466, 17], "end": [466, 22], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/metrics/value.rs:466`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86c5fcf186ed75cdb83a4a6e"></a>
## default

`function` · `datafusion_physical_expr_common::metrics::value::RatioMetrics::default` · datafusion-physical-expr-common 55.1.0

```rust
fn default() -> RatioMetrics
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::RatioMetrics", "path": "RatioMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [466, 24], "end": [466, 31], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/metrics/value.rs:466`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f77fba1107c7643765ff9b1"></a>
## display_raw_values

`function` · `datafusion_physical_expr_common::metrics::value::RatioMetrics::display_raw_values` · datafusion-physical-expr-common 55.1.0

```rust
fn display_raw_values(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::RatioMetrics", "path": "RatioMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [562, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:559`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Whether `Display` for this metric appends the raw `(part/total)` numbers
alongside the percentage

<a id="op-f627452c7977487c9d917f28"></a>
## eq

`function` · `datafusion_physical_expr_common::metrics::value::RatioMetrics::eq` · datafusion-physical-expr-common 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::RatioMetrics", "path": "RatioMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [564, 1], "end": [570, 2], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/metrics/value.rs:565`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ed52ed2449893754e71388e"></a>
## fmt

`function` · `datafusion_physical_expr_common::metrics::value::RatioMetrics::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::RatioMetrics", "path": "RatioMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [572, 1], "end": [618, 2], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/metrics/value.rs:574`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Format the ratio to a format like '18.26% (220/1150)'

<a id="op-ba2ff80584b230b4775a55b4"></a>
## fmt

`function` · `datafusion_physical_expr_common::metrics::value::RatioMetrics::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::RatioMetrics", "path": "RatioMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [466, 10], "end": [466, 15], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/metrics/value.rs:466`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-19d8f85468054f9347dadd7d"></a>
## merge

`function` · `datafusion_physical_expr_common::metrics::value::RatioMetrics::merge` · datafusion-physical-expr-common 55.1.0

```rust
fn merge(&self, other: &Self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::RatioMetrics", "path": "RatioMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [562, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:525`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Merge the value from `other` into `self`

<a id="op-c08818dd25fa666521b4f055"></a>
## merge_strategy

`function` · `datafusion_physical_expr_common::metrics::value::RatioMetrics::merge_strategy` · datafusion-physical-expr-common 55.1.0

```rust
fn merge_strategy(&self) -> &RatioMergeStrategy
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::RatioMetrics", "path": "RatioMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [562, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:553`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Return the strategy used to merge two [`RatioMetrics`](../operations/datafusion_physical_expr_common.metrics.value.RatioMetrics.md#op-a47dbfa1bdc8f5bf2bf4eb22) values

<a id="op-b4f21779ae531e82cdf567c9"></a>
## new

`function` · `datafusion_physical_expr_common::metrics::value::RatioMetrics::new` · datafusion-physical-expr-common 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::RatioMetrics", "path": "RatioMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [562, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:485`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Create a new [`RatioMetrics`](../operations/datafusion_physical_expr_common.metrics.value.RatioMetrics.md#op-a47dbfa1bdc8f5bf2bf4eb22)

<a id="op-a10c8646225bc36afb9a35ec"></a>
## part

`function` · `datafusion_physical_expr_common::metrics::value::RatioMetrics::part` · datafusion-physical-expr-common 55.1.0

```rust
fn part(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::RatioMetrics", "path": "RatioMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [562, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:543`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Return the numerator (`part`) value

<a id="op-34fd7d2df2e350188c23c054"></a>
## set_part

`function` · `datafusion_physical_expr_common::metrics::value::RatioMetrics::set_part` · datafusion-physical-expr-common 55.1.0

```rust
fn set_part(&self, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::RatioMetrics", "path": "RatioMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [562, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:515`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Set the numerator (`part`) value to `n`, overwriting any existing value

<a id="op-3a937b93e04dbb0fb723f0f5"></a>
## set_total

`function` · `datafusion_physical_expr_common::metrics::value::RatioMetrics::set_total` · datafusion-physical-expr-common 55.1.0

```rust
fn set_total(&self, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::RatioMetrics", "path": "RatioMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [562, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:520`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Set the denominator (`total`) value to `n`, overwriting any existing value

<a id="op-d4197926cc0707db59b3c49e"></a>
## total

`function` · `datafusion_physical_expr_common::metrics::value::RatioMetrics::total` · datafusion-physical-expr-common 55.1.0

```rust
fn total(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::RatioMetrics", "path": "RatioMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [562, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:548`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Return the denominator (`total`) value

<a id="op-aa723a96f32b3286b08a099b"></a>
## with_display_raw_values

`function` · `datafusion_physical_expr_common::metrics::value::RatioMetrics::with_display_raw_values` · datafusion-physical-expr-common 55.1.0

```rust
fn with_display_raw_values(self, display_raw_values: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::RatioMetrics", "path": "RatioMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [562, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:499`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5fef965fe2943324ea8b7e54"></a>
## with_merge_strategy

`function` · `datafusion_physical_expr_common::metrics::value::RatioMetrics::with_merge_strategy` · datafusion-physical-expr-common 55.1.0

```rust
fn with_merge_strategy(self, merge_strategy: RatioMergeStrategy) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::RatioMetrics", "path": "RatioMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [483, 1], "end": [562, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:494`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
