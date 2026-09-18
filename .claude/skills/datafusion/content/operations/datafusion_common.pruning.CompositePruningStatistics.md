# `datafusion_common::pruning::CompositePruningStatistics`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.pruning.CompositePruningStatistics.json).

<a id="op-a5d7eede713af44570a95609"></a>
## CompositePruningStatistics

`struct` · `datafusion_common::pruning::CompositePruningStatistics` · datafusion-common 55.1.0

```rust
struct CompositePruningStatistics
```

Source: `src/pruning.rs:451`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Combine multiple [`PruningStatistics`](../operations/datafusion_common.pruning.PruningStatistics.md#op-a18da0087d8b285319f91952) into a single
[`CompositePruningStatistics`](../operations/datafusion_common.pruning.CompositePruningStatistics.md#op-a5d7eede713af44570a95609).
This can be used to combine statistics from different sources,
for example partition values and file statistics.
This allows pruning with filters that depend on multiple sources of statistics,
such as `WHERE partition_col = data_col`.
This is done by iterating over the statistics and returning the first
one that has information for the requested column.
If multiple statistics have information for the same column,
the first one is returned without any regard for completeness or accuracy.
That is: if the first statistics has information for a column, even if it is incomplete,
that is returned even if a later statistics has more complete information.

<a id="op-b798de10f9b7775140c11584"></a>
## contained

`function` · `datafusion_common::pruning::CompositePruningStatistics::contained` · datafusion-common 55.1.0

```rust
fn contained(&self, column: &Column, values: &HashSet<ScalarValue>) -> Option<BooleanArray>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::pruning::CompositePruningStatistics", "path": "CompositePruningStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [471, 1], "end": [524, 2], "filename": "src/pruning.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `src/pruning.rs:512`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5097f5d7bf9c8790927ae5bc"></a>
## max_values

`function` · `datafusion_common::pruning::CompositePruningStatistics::max_values` · datafusion-common 55.1.0

```rust
fn max_values(&self, column: &Column) -> Option<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::pruning::CompositePruningStatistics", "path": "CompositePruningStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [471, 1], "end": [524, 2], "filename": "src/pruning.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `src/pruning.rs:481`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-920ab8c800432ed8f1cebec5"></a>
## min_values

`function` · `datafusion_common::pruning::CompositePruningStatistics::min_values` · datafusion-common 55.1.0

```rust
fn min_values(&self, column: &Column) -> Option<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::pruning::CompositePruningStatistics", "path": "CompositePruningStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [471, 1], "end": [524, 2], "filename": "src/pruning.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `src/pruning.rs:472`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8891ab56920144966be51670"></a>
## new

`function` · `datafusion_common::pruning::CompositePruningStatistics::new` · datafusion-common 55.1.0

```rust
fn new(statistics: Vec<Box<dyn PruningStatistics>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::pruning::CompositePruningStatistics", "path": "CompositePruningStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [456, 1], "end": [468, 2], "filename": "src/pruning.rs"}, "trait": null, "trait_path": null}`

Source: `src/pruning.rs:459`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create a new instance of [`CompositePruningStatistics`](../operations/datafusion_common.pruning.CompositePruningStatistics.md#op-a5d7eede713af44570a95609) from
a vector of [`PruningStatistics`](../operations/datafusion_common.pruning.PruningStatistics.md#op-a18da0087d8b285319f91952).

<a id="op-9e47fad54a9f388852c3bb58"></a>
## null_counts

`function` · `datafusion_common::pruning::CompositePruningStatistics::null_counts` · datafusion-common 55.1.0

```rust
fn null_counts(&self, column: &Column) -> Option<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::pruning::CompositePruningStatistics", "path": "CompositePruningStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [471, 1], "end": [524, 2], "filename": "src/pruning.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `src/pruning.rs:494`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-efab5b1369496051e5faff71"></a>
## num_containers

`function` · `datafusion_common::pruning::CompositePruningStatistics::num_containers` · datafusion-common 55.1.0

```rust
fn num_containers(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::pruning::CompositePruningStatistics", "path": "CompositePruningStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [471, 1], "end": [524, 2], "filename": "src/pruning.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `src/pruning.rs:490`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-40b0bcf901ddbdad1fe30b98"></a>
## row_counts

`function` · `datafusion_common::pruning::CompositePruningStatistics::row_counts` · datafusion-common 55.1.0

```rust
fn row_counts(&self) -> Option<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::pruning::CompositePruningStatistics", "path": "CompositePruningStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [471, 1], "end": [524, 2], "filename": "src/pruning.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `src/pruning.rs:503`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d75d765cb1d23ec031c99742"></a>
## statistics

`struct_field` · `datafusion_common::pruning::CompositePruningStatistics::statistics` · datafusion-common 55.1.0

```rust
statistics: Vec<Box<dyn PruningStatistics>>
```

Source: `src/pruning.rs:452`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
