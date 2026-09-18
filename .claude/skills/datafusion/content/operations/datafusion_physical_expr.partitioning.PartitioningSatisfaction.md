# `datafusion_physical_expr::partitioning::PartitioningSatisfaction`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.partitioning.PartitioningSatisfaction.json).

<a id="op-4f76c9a5234b981e4160ba93"></a>
## PartitioningSatisfaction

`enum` · `datafusion_physical_expr::partitioning::PartitioningSatisfaction` · datafusion-physical-expr 55.1.0

```rust
enum PartitioningSatisfaction
```

Source: `src/partitioning.rs:340`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Represents how a [`Partitioning`](../operations/datafusion_physical_expr.partitioning.Partitioning.md#op-b631e1d1890e8ea81608b4a4) satisfies a [`Distribution`](../operations/datafusion_physical_expr.partitioning.Distribution.md#op-1570134c2bb176dfcdc9c836) requirement.

<a id="op-21324006f3c5496898de5c07"></a>
## Exact

`variant` · `datafusion_physical_expr::partitioning::PartitioningSatisfaction::Exact` · datafusion-physical-expr 55.1.0

```rust
Exact
```

Source: `src/partitioning.rs:344`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

The partitioning exactly matches the distribution requirement

<a id="op-81a1410e789131de6f3afad7"></a>
## NotSatisfied

`variant` · `datafusion_physical_expr::partitioning::PartitioningSatisfaction::NotSatisfied` · datafusion-physical-expr 55.1.0

```rust
NotSatisfied
```

Source: `src/partitioning.rs:342`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

The partitioning does not satisfy the distribution requirement

<a id="op-57bcec58f48a64685ec7d4f4"></a>
## Subset

`variant` · `datafusion_physical_expr::partitioning::PartitioningSatisfaction::Subset` · datafusion-physical-expr 55.1.0

```rust
Subset
```

Source: `src/partitioning.rs:346`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

The partitioning satisfies the distribution requirement via subset logic

<a id="op-d9d56c04b283a2b98030748e"></a>
## clone

`function` · `datafusion_physical_expr::partitioning::PartitioningSatisfaction::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> PartitioningSatisfaction
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::partitioning::PartitioningSatisfaction", "path": "PartitioningSatisfaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [339, 17], "end": [339, 22], "filename": "src/partitioning.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/partitioning.rs:339`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a689485594480398b60f471"></a>
## eq

`function` · `datafusion_physical_expr::partitioning::PartitioningSatisfaction::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, other: &PartitioningSatisfaction) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::partitioning::PartitioningSatisfaction", "path": "PartitioningSatisfaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [339, 30], "end": [339, 39], "filename": "src/partitioning.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/partitioning.rs:339`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aba404b7970aa7553dd3ed9f"></a>
## fmt

`function` · `datafusion_physical_expr::partitioning::PartitioningSatisfaction::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::partitioning::PartitioningSatisfaction", "path": "PartitioningSatisfaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [339, 10], "end": [339, 15], "filename": "src/partitioning.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/partitioning.rs:339`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef1ef806038abf1eee22296e"></a>
## is_satisfied

`function` · `datafusion_physical_expr::partitioning::PartitioningSatisfaction::is_satisfied` · datafusion-physical-expr 55.1.0

```rust
fn is_satisfied(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::partitioning::PartitioningSatisfaction", "path": "PartitioningSatisfaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [349, 1], "end": [357, 2], "filename": "src/partitioning.rs"}, "trait": null, "trait_path": null}`

Source: `src/partitioning.rs:350`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e26dd7713912b2e59540a74"></a>
## is_subset

`function` · `datafusion_physical_expr::partitioning::PartitioningSatisfaction::is_subset` · datafusion-physical-expr 55.1.0

```rust
fn is_subset(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::partitioning::PartitioningSatisfaction", "path": "PartitioningSatisfaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [349, 1], "end": [357, 2], "filename": "src/partitioning.rs"}, "trait": null, "trait_path": null}`

Source: `src/partitioning.rs:354`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
