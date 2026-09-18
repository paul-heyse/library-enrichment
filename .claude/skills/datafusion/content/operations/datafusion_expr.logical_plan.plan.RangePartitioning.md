# `datafusion_expr::logical_plan::plan::RangePartitioning`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.plan.RangePartitioning.json).

<a id="op-3ed7bc8fa0984d56a44cea37"></a>
## RangePartitioning

`struct` · `datafusion_expr::logical_plan::plan::RangePartitioning` · datafusion-expr 55.1.0

```rust
struct RangePartitioning
```

Source: `src/logical_plan/plan.rs:4523`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Logical range partitioning.

[`RangePartitioning`](../operations/datafusion_expr.logical_plan.plan.RangePartitioning.md#op-3ed7bc8fa0984d56a44cea37) describes an ordered logical key space with split points.

- `ordering` defines the partitioning key and ordering using logical
  [`SortExpr`](../operations/datafusion_expr.expr.Sort.md#op-db804608f982bce14d75f784)s.
- `split_points` define the boundaries between adjacent partitions.

Comparisons use the lexicographic order defined by `ordering`,
including `ASC`/`DESC` and null ordering. Split points must be ordered
according to that ordering, and each split point must have one value per
ordering expression. See [`SplitPoint`](../operations/datafusion_common.partitioning.SplitPoint.md#op-8de4bfc487ce639976af8dad) for the shared boundary contract.

The expressions are resolved against the declaring plan's schema. This
constructor does not validate split point value types against the resolved
expression types. Like other user-specified data properties such as
sortedness, if a source declares range partitioning, it is responsible for
placing each row in the partition described by the split points. DataFusion
will not validate this is upheld.

NOTE: Range-aware optimizer and execution behavior will be introduced
incrementally. See
<https://github.com/apache/datafusion/issues/22395>.

<a id="op-8256572a2f081b69f52bdc4a"></a>
## clone

`function` · `datafusion_expr::logical_plan::plan::RangePartitioning::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> RangePartitioning
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::RangePartitioning", "path": "RangePartitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4522, 17], "end": [4522, 22], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/plan.rs:4522`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b90ebbd0b00a0384b171a5da"></a>
## eq

`function` · `datafusion_expr::logical_plan::plan::RangePartitioning::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &RangePartitioning) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::RangePartitioning", "path": "RangePartitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4522, 24], "end": [4522, 33], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/plan.rs:4522`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b052986b2832412a1421d44"></a>
## fmt

`function` · `datafusion_expr::logical_plan::plan::RangePartitioning::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::RangePartitioning", "path": "RangePartitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4575, 1], "end": [4589, 2], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/logical_plan/plan.rs:4576`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dda82a9aa55267637b7d76cd"></a>
## fmt

`function` · `datafusion_expr::logical_plan::plan::RangePartitioning::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::RangePartitioning", "path": "RangePartitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4522, 10], "end": [4522, 15], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/plan.rs:4522`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6daa300e554124f6b348042c"></a>
## hash

`function` · `datafusion_expr::logical_plan::plan::RangePartitioning::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::RangePartitioning", "path": "RangePartitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4522, 51], "end": [4522, 55], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/plan.rs:4522`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-236332e56a36be64b5baea27"></a>
## ordering

`function` · `datafusion_expr::logical_plan::plan::RangePartitioning::ordering` · datafusion-expr 55.1.0

```rust
fn ordering(&self) -> &[SortExpr]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::RangePartitioning", "path": "RangePartitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4530, 1], "end": [4563, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:4555`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the ordering that defines the range key.

<a id="op-f3dc1e04f56df196e9deb63f"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::plan::RangePartitioning::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &RangePartitioning) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::RangePartitioning", "path": "RangePartitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4522, 39], "end": [4522, 49], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/plan.rs:4522`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03ba2f7fafa36d4a42df919e"></a>
## partition_count

`function` · `datafusion_expr::logical_plan::plan::RangePartitioning::partition_count` · datafusion-expr 55.1.0

```rust
fn partition_count(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::RangePartitioning", "path": "RangePartitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4530, 1], "end": [4563, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:4550`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return the number of partitions.

<a id="op-e73cc78f57074e42c6aee1b6"></a>
## split_points

`function` · `datafusion_expr::logical_plan::plan::RangePartitioning::split_points` · datafusion-expr 55.1.0

```rust
fn split_points(&self) -> &[SplitPoint]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::RangePartitioning", "path": "RangePartitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4530, 1], "end": [4563, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:4560`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the ordered split points between partitions.

<a id="op-6711aa59815c67ff02f2db1d"></a>
## try_new

`function` · `datafusion_expr::logical_plan::plan::RangePartitioning::try_new` · datafusion-expr 55.1.0

```rust
fn try_new(ordering: Vec<SortExpr>, split_points: Vec<SplitPoint>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::RangePartitioning", "path": "RangePartitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4530, 1], "end": [4563, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:4533`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Creates logical range partitioning metadata and validates split point
shape and ordering.
