# `datafusion_physical_expr::partitioning::Distribution`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.partitioning.Distribution.json).

<a id="op-1570134c2bb176dfcdc9c836"></a>
## Distribution

`enum` · `datafusion_physical_expr::partitioning::Distribution` · datafusion-physical-expr 55.1.0

```rust
enum Distribution
```

Source: `src/partitioning.rs:693`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

How data is distributed amongst partitions. See [`Partitioning`](../operations/datafusion_physical_expr.partitioning.Partitioning.md#op-b631e1d1890e8ea81608b4a4) for more
details.

<a id="op-97548ab7465a6de021630426"></a>
## HashPartitioned

`variant` · `datafusion_physical_expr::partitioning::Distribution::HashPartitioned` · datafusion-physical-expr 55.1.0

```rust
HashPartitioned
```

Source: `src/partitioning.rs:701`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Deprecated historical name for [`Distribution::KeyPartitioned`](../operations/datafusion_physical_expr.partitioning.Distribution.md#op-622b310e47f6959dc4667ef7).
See <https://github.com/apache/datafusion/issues/23236> for details.

<a id="op-622b310e47f6959dc4667ef7"></a>
## KeyPartitioned

`variant` · `datafusion_physical_expr::partitioning::Distribution::KeyPartitioned` · datafusion-physical-expr 55.1.0

```rust
KeyPartitioned
```

Source: `src/partitioning.rs:704`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Requires children to be distributed in such a way that the same
values of the keys end up in the same partition

<a id="op-680dd37e58998e663a01d458"></a>
## SinglePartition

`variant` · `datafusion_physical_expr::partitioning::Distribution::SinglePartition` · datafusion-physical-expr 55.1.0

```rust
SinglePartition
```

Source: `src/partitioning.rs:697`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

A single partition is required

<a id="op-28e6f98afaf45f86586579b0"></a>
## UnspecifiedDistribution

`variant` · `datafusion_physical_expr::partitioning::Distribution::UnspecifiedDistribution` · datafusion-physical-expr 55.1.0

```rust
UnspecifiedDistribution
```

Source: `src/partitioning.rs:695`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Unspecified distribution

<a id="op-0764dfb1fb9b2e23a309383a"></a>
## clone

`function` · `datafusion_physical_expr::partitioning::Distribution::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> Distribution
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::partitioning::Distribution", "path": "Distribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [692, 17], "end": [692, 22], "filename": "src/partitioning.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/partitioning.rs:692`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b37de251631ca73319b4118"></a>
## create_partitioning

`function` · `datafusion_physical_expr::partitioning::Distribution::create_partitioning` · datafusion-physical-expr 55.1.0

```rust
fn create_partitioning(self, partition_count: usize) -> Partitioning
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::partitioning::Distribution", "path": "Distribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [711, 1], "end": [724, 2], "filename": "src/partitioning.rs"}, "trait": null, "trait_path": null}`

Source: `src/partitioning.rs:713`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Creates a `Partitioning` that satisfies this `Distribution`

<a id="op-7760754c41d3befc0008e91d"></a>
## fmt

`function` · `datafusion_physical_expr::partitioning::Distribution::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::partitioning::Distribution", "path": "Distribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [692, 10], "end": [692, 15], "filename": "src/partitioning.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/partitioning.rs:692`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80cf6d76f3172548f5970fb2"></a>
## fmt

`function` · `datafusion_physical_expr::partitioning::Distribution::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::partitioning::Distribution", "path": "Distribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [730, 1], "end": [743, 2], "filename": "src/partitioning.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/partitioning.rs:731`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
