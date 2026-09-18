# `deltalake_core::operations::optimize::Metrics`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.optimize.Metrics.json).

<a id="op-edac3cfa33e9b7ae2bddbfd8"></a>
## Metrics

`struct` · `deltalake_core::operations::optimize::Metrics` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct Metrics
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L87).

Source: `crates/core/src/operations/optimize.rs:87`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Metrics from Optimize

<a id="op-ae2144ea26d4f59af3d9dd3b"></a>
## add

`function` · `deltalake_core::operations::optimize::Metrics::add` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn add(&mut self, partial: &PartialMetrics)
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L239).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::optimize::Metrics", "path": "Metrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [237, 1], "end": [253, 2], "filename": "crates/core/src/operations/optimize.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/optimize.rs:239`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Add a partial metric to the metrics

<a id="op-cd1f991724989b192190f3e8"></a>
## clone

`function` · `deltalake_core::operations::optimize::Metrics::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> Metrics
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L85).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::optimize::Metrics", "path": "Metrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 37], "end": [85, 42], "filename": "crates/core/src/operations/optimize.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/operations/optimize.rs:85`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef42a1141c7854b5420c7eb2"></a>
## default

`function` · `deltalake_core::operations::optimize::Metrics::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> Metrics
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L85).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::optimize::Metrics", "path": "Metrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 10], "end": [85, 17], "filename": "crates/core/src/operations/optimize.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/operations/optimize.rs:85`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5843cac69301ce033a3a1348"></a>
## deserialize

`function` · `deltalake_core::operations::optimize::Metrics::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L85).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::optimize::Metrics", "path": "Metrics"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 55], "end": [85, 66], "filename": "crates/core/src/operations/optimize.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/operations/optimize.rs:85`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02b3c97402d5e60c5f327e2c"></a>
## eq

`function` · `deltalake_core::operations::optimize::Metrics::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &Metrics) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L85).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::optimize::Metrics", "path": "Metrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 26], "end": [85, 35], "filename": "crates/core/src/operations/optimize.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/operations/optimize.rs:85`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36cdc82152cd2f776764c519"></a>
## files_added

`struct_field` · `deltalake_core::operations::optimize::Metrics::files_added` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
files_added: MetricDetails
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L97).

Source: `crates/core/src/operations/optimize.rs:97`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Detailed metrics for the add operation

<a id="op-e0994f2b219f42755e3af6fb"></a>
## files_removed

`struct_field` · `deltalake_core::operations::optimize::Metrics::files_removed` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
files_removed: MetricDetails
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L103).

Source: `crates/core/src/operations/optimize.rs:103`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Detailed metrics for the remove operation

<a id="op-de9889ac394d59dd4be72794"></a>
## fmt

`function` · `deltalake_core::operations::optimize::Metrics::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L85).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::optimize::Metrics", "path": "Metrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 19], "end": [85, 24], "filename": "crates/core/src/operations/optimize.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/operations/optimize.rs:85`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a029e3ac29cfbf240523725f"></a>
## max_bin_span_files

`struct_field` · `deltalake_core::operations::optimize::Metrics::max_bin_span_files` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
max_bin_span_files: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L119).

Source: `crates/core/src/operations/optimize.rs:119`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Largest count of adjacent input files in one bin

<a id="op-4b883e5a0cb4007fa3440e52"></a>
## num_batches

`struct_field` · `deltalake_core::operations::optimize::Metrics::num_batches` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_batches: u64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L107).

Source: `crates/core/src/operations/optimize.rs:107`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The number of batches written

<a id="op-e898a5816fd8e99c65486611"></a>
## num_files_added

`struct_field` · `deltalake_core::operations::optimize::Metrics::num_files_added` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_files_added: u64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L89).

Source: `crates/core/src/operations/optimize.rs:89`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of optimized files added

<a id="op-106db4bd9fdd7cdb80cf538d"></a>
## num_files_removed

`struct_field` · `deltalake_core::operations::optimize::Metrics::num_files_removed` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_files_removed: u64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L91).

Source: `crates/core/src/operations/optimize.rs:91`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of unoptimized files removed

<a id="op-c616ddec0ac7038d42716764"></a>
## partitions_optimized

`struct_field` · `deltalake_core::operations::optimize::Metrics::partitions_optimized` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
partitions_optimized: u64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L105).

Source: `crates/core/src/operations/optimize.rs:105`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of partitions that had at least one file optimized

<a id="op-36cfef6f31395a7b0705bb54"></a>
## planner_strategy

`struct_field` · `deltalake_core::operations::optimize::Metrics::planner_strategy` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
planner_strategy: PlannerStrategy
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L115).

Source: `crates/core/src/operations/optimize.rs:115`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Planner used for this run

<a id="op-ff6fe86508b170fdc605090c"></a>
## preserve_insertion_order

`struct_field` · `deltalake_core::operations::optimize::Metrics::preserve_insertion_order` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
preserve_insertion_order: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L113).

Source: `crates/core/src/operations/optimize.rs:113`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Compatibility field for `preserved_stable_order`

<a id="op-abf5c0ef7c60ebec7a1ec254"></a>
## preserved_stable_order

`struct_field` · `deltalake_core::operations::optimize::Metrics::preserved_stable_order` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
preserved_stable_order: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L117).

Source: `crates/core/src/operations/optimize.rs:117`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

True when file order is kept within a partition

<a id="op-0a5061121b82003f70f8940d"></a>
## serialize

`function` · `deltalake_core::operations::optimize::Metrics::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L85).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::optimize::Metrics", "path": "Metrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 44], "end": [85, 53], "filename": "crates/core/src/operations/optimize.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/operations/optimize.rs:85`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf958a43f9082ef017b545e9"></a>
## total_considered_files

`struct_field` · `deltalake_core::operations::optimize::Metrics::total_considered_files` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
total_considered_files: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L109).

Source: `crates/core/src/operations/optimize.rs:109`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

How many files were considered during optimization. Not every file considered is optimized

<a id="op-55e62e455b208e56c9e34a00"></a>
## total_files_skipped

`struct_field` · `deltalake_core::operations::optimize::Metrics::total_files_skipped` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
total_files_skipped: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L111).

Source: `crates/core/src/operations/optimize.rs:111`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

How many files were considered for optimization but were skipped

<a id="op-d2108f76e8adcb49ead47cd0"></a>
## from

`function` · `deltalake_core::operations::optimize::Metrics::from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(value: MetricsSerde) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L146).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::optimize::Metrics", "path": "Metrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 1], "end": [168, 2], "filename": "crates/core/src/operations/optimize.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "deltalake_core::operations::optimize::MetricsSerde", "path": "MetricsSerde"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/operations/optimize.rs:146`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
