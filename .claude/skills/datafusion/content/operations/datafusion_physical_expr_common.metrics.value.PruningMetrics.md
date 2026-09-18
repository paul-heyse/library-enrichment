# `datafusion_physical_expr_common::metrics::value::PruningMetrics`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.metrics.value.PruningMetrics.json).

<a id="op-67d9f137894402b6d11e2f17"></a>
## PruningMetrics

`struct` · `datafusion_physical_expr_common::metrics::value::PruningMetrics` · datafusion-physical-expr-common 55.1.0

```rust
struct PruningMetrics
```

Source: `src/metrics/value.rs:372`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Counters tracking pruning metrics

For example, a file scanner initially is planned to scan 10 files, but skipped
8 of them using statistics, the pruning metrics would look like: 10 total -> 2 matched

Note `clone`ing update the same underlying metrics

<a id="op-33d5ad19dcaf3fcfcd9af4cd"></a>
## add_fully_matched

`function` · `datafusion_physical_expr_common::metrics::value::PruningMetrics::add_fully_matched` · datafusion-physical-expr-common 55.1.0

```rust
fn add_fully_matched(&self, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::PruningMetrics", "path": "PruningMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [409, 1], "end": [461, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:434`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Add `n` to the metric's fully matched value

<a id="op-627fc238a8227794f8dc47e0"></a>
## add_matched

`function` · `datafusion_physical_expr_common::metrics::value::PruningMetrics::add_matched` · datafusion-physical-expr-common 55.1.0

```rust
fn add_matched(&self, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::PruningMetrics", "path": "PruningMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [409, 1], "end": [461, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:427`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Add `n` to the metric's matched value

<a id="op-355f0a1da666b84a78e19a16"></a>
## add_pruned

`function` · `datafusion_physical_expr_common::metrics::value::PruningMetrics::add_pruned` · datafusion-physical-expr-common 55.1.0

```rust
fn add_pruned(&self, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::PruningMetrics", "path": "PruningMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [409, 1], "end": [461, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:420`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Add `n` to the metric's pruned value

<a id="op-7d52bb4402d68124fc8c1e73"></a>
## clone

`function` · `datafusion_physical_expr_common::metrics::value::PruningMetrics::clone` · datafusion-physical-expr-common 55.1.0

```rust
fn clone(&self) -> PruningMetrics
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::PruningMetrics", "path": "PruningMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [371, 17], "end": [371, 22], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/metrics/value.rs:371`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c3dedcb94e639f8c546713d"></a>
## default

`function` · `datafusion_physical_expr_common::metrics::value::PruningMetrics::default` · datafusion-physical-expr-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::PruningMetrics", "path": "PruningMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [407, 2], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/metrics/value.rs:404`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0ffb62bcbf923763a749048"></a>
## fmt

`function` · `datafusion_physical_expr_common::metrics::value::PruningMetrics::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::PruningMetrics", "path": "PruningMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [371, 10], "end": [371, 15], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/metrics/value.rs:371`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f60c0d6ebcff4123fa6eb9f5"></a>
## fmt

`function` · `datafusion_physical_expr_common::metrics::value::PruningMetrics::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::PruningMetrics", "path": "PruningMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [378, 1], "end": [401, 2], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/metrics/value.rs:379`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3905a37fd2bceea3d4f46d02"></a>
## fully_matched

`function` · `datafusion_physical_expr_common::metrics::value::PruningMetrics::fully_matched` · datafusion-physical-expr-common 55.1.0

```rust
fn fully_matched(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::PruningMetrics", "path": "PruningMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [409, 1], "end": [461, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:458`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Number of items fully matched

<a id="op-2231531fef45037be220c713"></a>
## matched

`function` · `datafusion_physical_expr_common::metrics::value::PruningMetrics::matched` · datafusion-physical-expr-common 55.1.0

```rust
fn matched(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::PruningMetrics", "path": "PruningMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [409, 1], "end": [461, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:453`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Number of items matched (not pruned)

<a id="op-d9597654ef14e6a786d00b34"></a>
## new

`function` · `datafusion_physical_expr_common::metrics::value::PruningMetrics::new` · datafusion-physical-expr-common 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::PruningMetrics", "path": "PruningMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [409, 1], "end": [461, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:411`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

create a new PruningMetrics

<a id="op-f308527e6e8ae16aa8b4e806"></a>
## pruned

`function` · `datafusion_physical_expr_common::metrics::value::PruningMetrics::pruned` · datafusion-physical-expr-common 55.1.0

```rust
fn pruned(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::PruningMetrics", "path": "PruningMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [409, 1], "end": [461, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:448`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Number of items pruned

<a id="op-fa96e93a804be734b6104266"></a>
## subtract_matched

`function` · `datafusion_physical_expr_common::metrics::value::PruningMetrics::subtract_matched` · datafusion-physical-expr-common 55.1.0

```rust
fn subtract_matched(&self, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::PruningMetrics", "path": "PruningMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [409, 1], "end": [461, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:441`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Subtract `n` to the metric's matched value.
