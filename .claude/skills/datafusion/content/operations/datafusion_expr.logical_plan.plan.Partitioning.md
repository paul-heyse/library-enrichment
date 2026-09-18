# `datafusion_expr::logical_plan::plan::Partitioning`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.plan.Partitioning.json).

<a id="op-100c6d964a092c3499015940"></a>
## Partitioning

`enum` · `datafusion_expr::logical_plan::plan::Partitioning` · datafusion-expr 55.1.0

```rust
enum Partitioning
```

Source: `src/logical_plan/plan.rs:4473`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Logical partitioning schemes.

A scheme can describe either requested repartitioning in
[`LogicalPlan::Repartition`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-7bb9b257178543f2e2c9aafd) or a partitioning property declared by a source.
Some schemes are only valid as metadata until planner support is added.

For physical execution partitioning, see
[`datafusion_physical_expr::Partitioning`].

[`datafusion_physical_expr::Partitioning`]: https://docs.rs/datafusion/latest/datafusion/physical_expr/enum.Partitioning.html#

<a id="op-4c49ebb8f39b94225198822a"></a>
## DistributeBy

`variant` · `datafusion_expr::logical_plan::plan::Partitioning::DistributeBy` · datafusion-expr 55.1.0

```rust
DistributeBy
```

Source: `src/logical_plan/plan.rs:4483`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The DISTRIBUTE BY clause is used to repartition the data based on the input expressions

<a id="op-1e0815252ba6b12753914106"></a>
## Hash

`variant` · `datafusion_expr::logical_plan::plan::Partitioning::Hash` · datafusion-expr 55.1.0

```rust
Hash
```

Source: `src/logical_plan/plan.rs:4478`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Allocate rows based on a hash of one of more expressions and the specified number
of partitions.

<a id="op-9bc8d22486bf5db5b41f2a3b"></a>
## Range

`variant` · `datafusion_expr::logical_plan::plan::Partitioning::Range` · datafusion-expr 55.1.0

```rust
Range
```

Source: `src/logical_plan/plan.rs:4481`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Partition rows by ranges.
See [`RangePartitioning`](../operations/datafusion_expr.logical_plan.plan.RangePartitioning.md#op-3ed7bc8fa0984d56a44cea37) for the logical contract.

<a id="op-101dd424142ebaa915413aec"></a>
## RoundRobinBatch

`variant` · `datafusion_expr::logical_plan::plan::Partitioning::RoundRobinBatch` · datafusion-expr 55.1.0

```rust
RoundRobinBatch
```

Source: `src/logical_plan/plan.rs:4475`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Allocate batches using a round-robin algorithm and the specified number of partitions

<a id="op-494dc3d7d8cb55bb28ad2a7a"></a>
## clone

`function` · `datafusion_expr::logical_plan::plan::Partitioning::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> Partitioning
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Partitioning", "path": "Partitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4472, 17], "end": [4472, 22], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/plan.rs:4472`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55b2db643cada196950f645b"></a>
## eq

`function` · `datafusion_expr::logical_plan::plan::Partitioning::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Partitioning) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Partitioning", "path": "Partitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4472, 24], "end": [4472, 33], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/plan.rs:4472`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8ecaa7708f260cce26a8858"></a>
## fmt

`function` · `datafusion_expr::logical_plan::plan::Partitioning::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Partitioning", "path": "Partitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4472, 10], "end": [4472, 15], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/plan.rs:4472`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-abe6f44b628f9e39f6fd30e5"></a>
## hash

`function` · `datafusion_expr::logical_plan::plan::Partitioning::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Partitioning", "path": "Partitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4472, 51], "end": [4472, 55], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/plan.rs:4472`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd0172c82bc0710ca2bbf685"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::plan::Partitioning::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Partitioning) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Partitioning", "path": "Partitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4472, 39], "end": [4472, 49], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/plan.rs:4472`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55bdf7f2da4315efb010ca07"></a>
## partition_count

`function` · `datafusion_expr::logical_plan::plan::Partitioning::partition_count` · datafusion-expr 55.1.0

```rust
fn partition_count(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Partitioning", "path": "Partitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4486, 1], "end": [4497, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:4488`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return the number of partitions, if known.
