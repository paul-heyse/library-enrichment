# `datafusion_physical_expr::partitioning::RangePartitioning`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.partitioning.RangePartitioning.json).

<a id="op-e2280b607dafb2f25fec61e5"></a>
## RangePartitioning

`struct` · `datafusion_physical_expr::partitioning::RangePartitioning` · datafusion-physical-expr 55.1.0

```rust
struct RangePartitioning
```

Source: `src/partitioning.rs:204`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Physical range partitioning.

[`RangePartitioning`](../operations/datafusion_physical_expr.partitioning.RangePartitioning.md#op-e2280b607dafb2f25fec61e5) describes an ordered key space with split points.

- `ordering` defines the partitioning key and ordering.
- `split_points` define the boundaries between adjacent partitions.

Comparisons use the lexicographic order defined by `ordering`, including
`ASC`/`DESC` and null ordering. Split points must be strictly ordered
according to that ordering, and each split point must have one value per
ordering expression. See [`SplitPoint`](../operations/datafusion_common.partitioning.SplitPoint.md#op-8de4bfc487ce639976af8dad) for the shared boundary convention.

Like other user-specified data properties such as sortedness, if a source
declares range partitioning, it is responsible for placing each row in the
partition described by the split points. DataFusion will not validate this is
upheld.

For a single range key:

```text
ordering = [date ASC NULLS LAST]
split_points = [
  (2022-01-01),
  (2023-01-01),
]

partition 0: date before 2022-01-01
partition 1: date between 2022-01-01 (inclusive) and 2023-01-01 (exclusive)
partition 2: date at/after 2023-01-01
```

The same model extends to compound keys.
For `ordering = [time ASC, city ASC]`, split points are ordered
lexicographically by `(time, city)`:

```text
ordering = [time ASC NULLS LAST, city ASC NULLS LAST]
split_points = [
  (2022, Allston),
  (2023, Allston),
]

partition 0: keys before  (2022, Allston)
partition 1: keys between (2022, Allston) and (2023, Allston)
partition 2: keys at/after (2023, Allston)
```

NOTE: Optimizer and execution behavior for this partitioning is intentionally
not implemented and will be introduced incrementally. See
<https://github.com/apache/datafusion/issues/22395>.

<a id="op-b01e9d2dd955741388329225"></a>
## clone

`function` · `datafusion_physical_expr::partitioning::RangePartitioning::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> RangePartitioning
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::partitioning::RangePartitioning", "path": "RangePartitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [203, 17], "end": [203, 22], "filename": "src/partitioning.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/partitioning.rs:203`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7e4c049aa6333236a093e4e"></a>
## eq

`function` · `datafusion_physical_expr::partitioning::RangePartitioning::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, other: &RangePartitioning) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::partitioning::RangePartitioning", "path": "RangePartitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [203, 24], "end": [203, 33], "filename": "src/partitioning.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/partitioning.rs:203`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-67c01b8bb68fd01d0df79925"></a>
## fmt

`function` · `datafusion_physical_expr::partitioning::RangePartitioning::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::partitioning::RangePartitioning", "path": "RangePartitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [286, 1], "end": [297, 2], "filename": "src/partitioning.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/partitioning.rs:287`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83fe2b02062750753a6d2f44"></a>
## fmt

`function` · `datafusion_physical_expr::partitioning::RangePartitioning::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::partitioning::RangePartitioning", "path": "RangePartitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [203, 10], "end": [203, 15], "filename": "src/partitioning.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/partitioning.rs:203`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fcb4cb0d3fca3d34ce98066c"></a>
## new

`function` · `datafusion_physical_expr::partitioning::RangePartitioning::new` · datafusion-physical-expr 55.1.0

```rust
fn new(ordering: LexOrdering, split_points: Vec<SplitPoint>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::partitioning::RangePartitioning", "path": "RangePartitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [211, 1], "end": [284, 2], "filename": "src/partitioning.rs"}, "trait": null, "trait_path": null}`

Source: `src/partitioning.rs:216`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Creates range partitioning metadata without validating split points.

Use [`Self::try_new`](../operations/datafusion_physical_expr.partitioning.RangePartitioning.md#op-0f0386d3d75c0a77c518f02a) to validate the contract documented on
[`RangePartitioning`](../operations/datafusion_physical_expr.partitioning.RangePartitioning.md#op-e2280b607dafb2f25fec61e5).

<a id="op-140873dd803307a5f16c4d6d"></a>
## ordering

`function` · `datafusion_physical_expr::partitioning::RangePartitioning::ordering` · datafusion-physical-expr 55.1.0

```rust
fn ordering(&self) -> &LexOrdering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::partitioning::RangePartitioning", "path": "RangePartitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [211, 1], "end": [284, 2], "filename": "src/partitioning.rs"}, "trait": null, "trait_path": null}`

Source: `src/partitioning.rs:237`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Returns the ordering that defines the range key.

<a id="op-a1dba03baa1e3a686d3eab59"></a>
## partition_count

`function` · `datafusion_physical_expr::partitioning::RangePartitioning::partition_count` · datafusion-physical-expr 55.1.0

```rust
fn partition_count(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::partitioning::RangePartitioning", "path": "RangePartitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [211, 1], "end": [284, 2], "filename": "src/partitioning.rs"}, "trait": null, "trait_path": null}`

Source: `src/partitioning.rs:247`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Returns the number of partitions.

<a id="op-ba95c38a18795057a14d1d04"></a>
## split_points

`function` · `datafusion_physical_expr::partitioning::RangePartitioning::split_points` · datafusion-physical-expr 55.1.0

```rust
fn split_points(&self) -> &[SplitPoint]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::partitioning::RangePartitioning", "path": "RangePartitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [211, 1], "end": [284, 2], "filename": "src/partitioning.rs"}, "trait": null, "trait_path": null}`

Source: `src/partitioning.rs:242`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Returns the ordered split points between partitions.

<a id="op-0f0386d3d75c0a77c518f02a"></a>
## try_new

`function` · `datafusion_physical_expr::partitioning::RangePartitioning::try_new` · datafusion-physical-expr 55.1.0

```rust
fn try_new(ordering: LexOrdering, split_points: Vec<SplitPoint>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::partitioning::RangePartitioning", "path": "RangePartitioning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [211, 1], "end": [284, 2], "filename": "src/partitioning.rs"}, "trait": null, "trait_path": null}`

Source: `src/partitioning.rs:225`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Creates range partitioning metadata and validates split point shape and
ordering.
