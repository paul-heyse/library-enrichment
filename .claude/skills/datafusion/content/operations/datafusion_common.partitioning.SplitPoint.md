# `datafusion_common::partitioning::SplitPoint`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.partitioning.SplitPoint.json).

<a id="op-8de4bfc487ce639976af8dad"></a>
## SplitPoint

`struct` · `datafusion_common::partitioning::SplitPoint` · datafusion-common 55.1.0

```rust
struct SplitPoint
```

Source: `src/partitioning.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A boundary between adjacent range partitions.

A split point is a tuple with one [`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b) per partitioning
expression. Split points are interpreted lexicographically according to the
ordering of the range partitioning that owns them.

`N` split points define `N + 1` partitions:

```text
partition 0: key < split_points[0]
partition 1: split_points[0] <= key < split_points[1]
...
partition N - 1: split_points[N - 2] <= key < split_points[N - 1]
partition N: split_points[N - 1] <= key
```

Values equal to split point `i` belong to partition `i + 1`, so interior
partitions are lower-inclusive and upper-exclusive.

<a id="op-dd02c5c34c45cdcf77825ea7"></a>
## clone

`function` · `datafusion_common::partitioning::SplitPoint::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> SplitPoint
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::partitioning::SplitPoint", "path": "SplitPoint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 17], "end": [42, 22], "filename": "src/partitioning.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/partitioning.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cab8e94d2c1a12f955a17b40"></a>
## eq

`function` · `datafusion_common::partitioning::SplitPoint::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &SplitPoint) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::partitioning::SplitPoint", "path": "SplitPoint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 24], "end": [42, 33], "filename": "src/partitioning.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/partitioning.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-529c2bdcf7900dffe18d1678"></a>
## fmt

`function` · `datafusion_common::partitioning::SplitPoint::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::partitioning::SplitPoint", "path": "SplitPoint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 10], "end": [42, 15], "filename": "src/partitioning.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/partitioning.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6eadbb03b2b9c68f35f09348"></a>
## fmt

`function` · `datafusion_common::partitioning::SplitPoint::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::partitioning::SplitPoint", "path": "SplitPoint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [69, 2], "filename": "src/partitioning.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/partitioning.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f4e78e6d802c5507ebd7f14"></a>
## hash

`function` · `datafusion_common::partitioning::SplitPoint::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::partitioning::SplitPoint", "path": "SplitPoint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 51], "end": [42, 55], "filename": "src/partitioning.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/partitioning.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-16f7e1270a0aff8f4972240a"></a>
## new

`function` · `datafusion_common::partitioning::SplitPoint::new` · datafusion-common 55.1.0

```rust
fn new(values: Vec<ScalarValue>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::partitioning::SplitPoint", "path": "SplitPoint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [57, 2], "filename": "src/partitioning.rs"}, "trait": null, "trait_path": null}`

Source: `src/partitioning.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Creates a new split point from its tuple values.

<a id="op-2aa4655676de721861a337f2"></a>
## partial_cmp

`function` · `datafusion_common::partitioning::SplitPoint::partial_cmp` · datafusion-common 55.1.0

```rust
fn partial_cmp(&self, other: &SplitPoint) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::partitioning::SplitPoint", "path": "SplitPoint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 39], "end": [42, 49], "filename": "src/partitioning.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/partitioning.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10ddf06c97bf750432c0a07d"></a>
## values

`function` · `datafusion_common::partitioning::SplitPoint::values` · datafusion-common 55.1.0

```rust
fn values(&self) -> &[ScalarValue]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::partitioning::SplitPoint", "path": "SplitPoint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [57, 2], "filename": "src/partitioning.rs"}, "trait": null, "trait_path": null}`

Source: `src/partitioning.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns the tuple values for this split point.
