# `datafusion_physical_expr_common::metrics::MetricsSet`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.metrics.MetricsSet.json).

<a id="op-c077c70588e76ca4a87a0b1b"></a>
## MetricsSet

`struct` · `datafusion_physical_expr_common::metrics::MetricsSet` · datafusion-physical-expr-common 55.1.0

```rust
struct MetricsSet
```

Source: `src/metrics/mod.rs:217`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

A snapshot of the metrics for a particular execution plan.

<a id="op-288bfa0e9fb3e0da8f781470"></a>
## IntoIter

`assoc_type` · `datafusion_physical_expr_common::metrics::MetricsSet::IntoIter` · datafusion-physical-expr-common 55.1.0

```rust
IntoIter
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::MetricsSet", "path": "MetricsSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [455, 1], "end": [462, 2], "filename": "src/metrics/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/metrics/mod.rs:457`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f028db4cc5f72448a0421753"></a>
## Item

`assoc_type` · `datafusion_physical_expr_common::metrics::MetricsSet::Item` · datafusion-physical-expr-common 55.1.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::MetricsSet", "path": "MetricsSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [455, 1], "end": [462, 2], "filename": "src/metrics/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/metrics/mod.rs:456`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25f85034dfd8127a644a7874"></a>
## aggregate_by_name

`function` · `datafusion_physical_expr_common::metrics::MetricsSet::aggregate_by_name` · datafusion-physical-expr-common 55.1.0

```rust
fn aggregate_by_name(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::MetricsSet", "path": "MetricsSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [436, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:325`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Returns a new derived `MetricsSet` where all metrics
that had the same name have been
aggregated together. The resulting `MetricsSet` has all
metrics with `Partition=None`

<a id="op-c2bfb0f7dda389be1c3da95f"></a>
## clone

`function` · `datafusion_physical_expr_common::metrics::MetricsSet::clone` · datafusion-physical-expr-common 55.1.0

```rust
fn clone(&self) -> MetricsSet
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::MetricsSet", "path": "MetricsSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [216, 26], "end": [216, 31], "filename": "src/metrics/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/metrics/mod.rs:216`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0869cd16c6f8ed78a4f981f0"></a>
## default

`function` · `datafusion_physical_expr_common::metrics::MetricsSet::default` · datafusion-physical-expr-common 55.1.0

```rust
fn default() -> MetricsSet
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::MetricsSet", "path": "MetricsSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [216, 10], "end": [216, 17], "filename": "src/metrics/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/metrics/mod.rs:216`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7df4cb009cec48ba0a8f1a0b"></a>
## elapsed_compute

`function` · `datafusion_physical_expr_common::metrics::MetricsSet::elapsed_compute` · datafusion-physical-expr-common 55.1.0

```rust
fn elapsed_compute(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::MetricsSet", "path": "MetricsSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [436, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:267`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Convenience: return the amount of elapsed CPU time spent,
aggregated across partitions or `None` if no metric is present

<a id="op-7ffb88cdd4f7c3febd311c3c"></a>
## extend

`function` · `datafusion_physical_expr_common::metrics::MetricsSet::extend` · datafusion-physical-expr-common 55.1.0

```rust
fn extend<I: IntoIterator<Item = Arc<Metric>>>(&mut self, iter: I)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::MetricsSet", "path": "MetricsSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [473, 1], "end": [477, 2], "filename": "src/metrics/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::Metric", "path": "Metric"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "core::iter::traits::collect::Extend", "path": "Extend"}, "trait_path": "core::iter::traits::collect::Extend"}`

Source: `src/metrics/mod.rs:474`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09eb3b05b592a2fba714ab6c"></a>
## filter_by_categories

`function` · `datafusion_physical_expr_common::metrics::MetricsSet::filter_by_categories` · datafusion-physical-expr-common 55.1.0

```rust
fn filter_by_categories(self, allowed: &[MetricCategory]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::MetricsSet", "path": "MetricsSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [436, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:404`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Returns a new `MetricsSet` filtered by [`MetricCategory`](../operations/datafusion_common.format.MetricCategory.md#op-d05732d49bd39fe3321965e4).

- Metrics that declared a category are kept only when that
  category appears in `allowed`.
- Metrics with **no** declared category are treated as
  [`Uncategorized`](MetricCategory::Uncategorized) for filtering.
- An **empty** `allowed` slice means "plan only": all metrics are
  removed.

<a id="op-bd228799cb4953a576f8c0c5"></a>
## filter_by_metric_types

`function` · `datafusion_physical_expr_common::metrics::MetricsSet::filter_by_metric_types` · datafusion-physical-expr-common 55.1.0

```rust
fn filter_by_metric_types(self, allowed: &[MetricType]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::MetricsSet", "path": "MetricsSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [436, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:383`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Returns a new derived `MetricsSet` containing only metrics whose
[`MetricType`](../operations/datafusion_common.format.MetricType.md#op-fb6314931b98a471bb076826) appears in `allowed`.

<a id="op-388a1a21521fe87772a4337e"></a>
## filter_by_names

`function` · `datafusion_physical_expr_common::metrics::MetricsSet::filter_by_names` · datafusion-physical-expr-common 55.1.0

```rust
fn filter_by_names(self, names: &[String]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::MetricsSet", "path": "MetricsSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [436, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:424`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Returns a new `MetricsSet` filtered by metric name.
Only metrics with the names appearing the list will be kept.

<a id="op-577be95d229b9834408d5d42"></a>
## fmt

`function` · `datafusion_physical_expr_common::metrics::MetricsSet::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::MetricsSet", "path": "MetricsSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [438, 1], "end": [453, 2], "filename": "src/metrics/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/metrics/mod.rs:440`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Format the [`MetricsSet`](../operations/datafusion_physical_expr_common.metrics.MetricsSet.md#op-c077c70588e76ca4a87a0b1b) as a single string

<a id="op-dff81e646a1ecd188815c8fb"></a>
## fmt

`function` · `datafusion_physical_expr_common::metrics::MetricsSet::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::MetricsSet", "path": "MetricsSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [216, 19], "end": [216, 24], "filename": "src/metrics/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/metrics/mod.rs:216`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5337600c6f3d6d2c012f6992"></a>
## from_iter

`function` · `datafusion_physical_expr_common::metrics::MetricsSet::from_iter` · datafusion-physical-expr-common 55.1.0

```rust
fn from_iter<T: IntoIterator<Item = Arc<Metric>>>(iter: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::MetricsSet", "path": "MetricsSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [479, 1], "end": [485, 2], "filename": "src/metrics/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::Metric", "path": "Metric"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/metrics/mod.rs:480`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8886df0be9473c98f25ae3ba"></a>
## into_iter

`function` · `datafusion_physical_expr_common::metrics::MetricsSet::into_iter` · datafusion-physical-expr-common 55.1.0

```rust
fn into_iter(self) -> Self::IntoIter
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::MetricsSet", "path": "MetricsSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [455, 1], "end": [462, 2], "filename": "src/metrics/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/metrics/mod.rs:459`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c05426fa04a26d6a971f4ee"></a>
## iter

`function` · `datafusion_physical_expr_common::metrics::MetricsSet::iter` · datafusion-physical-expr-common 55.1.0

```rust
fn iter(&self) -> impl Iterator<Item = &Arc<Metric>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::MetricsSet", "path": "MetricsSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [436, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:233`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Returns an iterator across all metrics

<a id="op-9ef1a9004cfb54f8ff40e03f"></a>
## new

`function` · `datafusion_physical_expr_common::metrics::MetricsSet::new` · datafusion-physical-expr-common 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::MetricsSet", "path": "MetricsSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [436, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:223`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Create a new container of metrics

<a id="op-bd7015ea0bfd5e1f7d85816e"></a>
## output_rows

`function` · `datafusion_physical_expr_common::metrics::MetricsSet::output_rows` · datafusion-physical-expr-common 55.1.0

```rust
fn output_rows(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::MetricsSet", "path": "MetricsSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [436, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:239`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Convenience: return the number of rows produced, aggregated
across partitions or `None` if no metric is present

<a id="op-1df83eb4b20fb826966ab765"></a>
## push

`function` · `datafusion_physical_expr_common::metrics::MetricsSet::push` · datafusion-physical-expr-common 55.1.0

```rust
fn push(&mut self, metric: Arc<Metric>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::MetricsSet", "path": "MetricsSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [436, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:228`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Add the specified metric

<a id="op-f84d28894416c0c0cd9832cb"></a>
## sorted_for_display

`function` · `datafusion_physical_expr_common::metrics::MetricsSet::sorted_for_display` · datafusion-physical-expr-common 55.1.0

```rust
fn sorted_for_display(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::MetricsSet", "path": "MetricsSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [436, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:359`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Sort the order of metrics so the "most useful" show up first

<a id="op-9ecadbc3142c998f13a79b3f"></a>
## spill_count

`function` · `datafusion_physical_expr_common::metrics::MetricsSet::spill_count` · datafusion-physical-expr-common 55.1.0

```rust
fn spill_count(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::MetricsSet", "path": "MetricsSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [436, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:246`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Convenience: return the count of spills, aggregated
across partitions or `None` if no metric is present

<a id="op-e0623a05e4db79687dcc9360"></a>
## spilled_bytes

`function` · `datafusion_physical_expr_common::metrics::MetricsSet::spilled_bytes` · datafusion-physical-expr-common 55.1.0

```rust
fn spilled_bytes(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::MetricsSet", "path": "MetricsSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [436, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:253`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Convenience: return the total byte size of spills, aggregated
across partitions or `None` if no metric is present

<a id="op-0fc0607462d12f6240289f80"></a>
## spilled_rows

`function` · `datafusion_physical_expr_common::metrics::MetricsSet::spilled_rows` · datafusion-physical-expr-common 55.1.0

```rust
fn spilled_rows(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::MetricsSet", "path": "MetricsSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [436, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:260`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Convenience: return the total rows of spills, aggregated
across partitions or `None` if no metric is present

<a id="op-bc4be38e2b6064d6b27d6b69"></a>
## sum

`function` · `datafusion_physical_expr_common::metrics::MetricsSet::sum` · datafusion-physical-expr-common 55.1.0

```rust
fn sum<F>(&self, f: F) -> Option<MetricValue> where F: FnMut(&Metric) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::MetricsSet", "path": "MetricsSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [436, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:275`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Sums the values for metrics for which `f(metric)` returns
`true`, and returns the value. Returns `None` if no metrics match
the predicate.

<a id="op-6624226e31667214f2755341"></a>
## sum_by_name

`function` · `datafusion_physical_expr_common::metrics::MetricsSet::sum_by_name` · datafusion-physical-expr-common 55.1.0

```rust
fn sum_by_name(&self, metric_name: &str) -> Option<MetricValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::MetricsSet", "path": "MetricsSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [436, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:299`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Returns the sum of all the metrics with the specified name
in the returned set.

<a id="op-a7c8570891342765977e9df3"></a>
## timestamps_removed

`function` · `datafusion_physical_expr_common::metrics::MetricsSet::timestamps_removed` · datafusion-physical-expr-common 55.1.0

```rust
fn timestamps_removed(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::MetricsSet", "path": "MetricsSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [436, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:370`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Remove all timestamp metrics (for more compact display)
