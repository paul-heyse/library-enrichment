# `datafusion_physical_plan::repartition::RangeExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.repartition.RangeExpr.json).

<a id="op-b27390fffa4862a858121832"></a>
## RangeExpr

`struct` · `datafusion_physical_plan::repartition::RangeExpr` · datafusion-physical-plan 55.1.0

```rust
struct RangeExpr
```

Source: `src/repartition/mod.rs:657`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Physical expression that returns the Range partition for each input row.

This uses the same routing function as [`BatchPartitioner`](../operations/datafusion_physical_plan.repartition.BatchPartitioner.md#op-86d978b7240ddd22811cc71d), so dynamic
filtering and repartitioning agree for every [`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b) comparison.

<a id="op-7643304e8b9f610456a31ee4"></a>
## children

`function` · `datafusion_physical_plan::repartition::RangeExpr::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&PhysicalExprRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RangeExpr", "path": "RangeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [722, 1], "end": [809, 2], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/repartition/mod.rs:723`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-76add31f0ac9a5799b272a96"></a>
## data_type

`function` · `datafusion_physical_plan::repartition::RangeExpr::data_type` · datafusion-physical-plan 55.1.0

```rust
fn data_type(&self, _input_schema: &Schema) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RangeExpr", "path": "RangeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [722, 1], "end": [809, 2], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/repartition/mod.rs:744`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a23c858d90dda33a68356db"></a>
## eq

`function` · `datafusion_physical_plan::repartition::RangeExpr::eq` · datafusion-physical-plan 55.1.0

```rust
fn eq(&self, other: &RangeExpr) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RangeExpr", "path": "RangeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [656, 23], "end": [656, 32], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/repartition/mod.rs:656`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41a835cf58015fd2edbc745f"></a>
## evaluate

`function` · `datafusion_physical_plan::repartition::RangeExpr::evaluate` · datafusion-physical-plan 55.1.0

```rust
fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RangeExpr", "path": "RangeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [722, 1], "end": [809, 2], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/repartition/mod.rs:752`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-42059e522247d93ffd867466"></a>
## fmt

`function` · `datafusion_physical_plan::repartition::RangeExpr::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RangeExpr", "path": "RangeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [716, 1], "end": [720, 2], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/repartition/mod.rs:717`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4229758fbdd665df19519699"></a>
## fmt

`function` · `datafusion_physical_plan::repartition::RangeExpr::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RangeExpr", "path": "RangeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [656, 10], "end": [656, 15], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/repartition/mod.rs:656`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-badef76d1d04eb2230edc9aa"></a>
## fmt_sql

`function` · `datafusion_physical_plan::repartition::RangeExpr::fmt_sql` · datafusion-physical-plan 55.1.0

```rust
fn fmt_sql(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RangeExpr", "path": "RangeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [722, 1], "end": [809, 2], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/repartition/mod.rs:769`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f95e152d0e2e672d9b986e3a"></a>
## hash

`function` · `datafusion_physical_plan::repartition::RangeExpr::hash` · datafusion-physical-plan 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RangeExpr", "path": "RangeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [656, 17], "end": [656, 21], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/repartition/mod.rs:656`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80ef42c39641286a730debfe"></a>
## nullable

`function` · `datafusion_physical_plan::repartition::RangeExpr::nullable` · datafusion-physical-plan 55.1.0

```rust
fn nullable(&self, _input_schema: &Schema) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RangeExpr", "path": "RangeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [722, 1], "end": [809, 2], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/repartition/mod.rs:748`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-863e1dc7c7334886491930a3"></a>
## on_columns

`function` · `datafusion_physical_plan::repartition::RangeExpr::on_columns` · datafusion-physical-plan 55.1.0

```rust
fn on_columns(&self) -> &[PhysicalExprRef]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RangeExpr", "path": "RangeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [663, 1], "end": [714, 2], "filename": "src/repartition/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/repartition/mod.rs:701`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Get the columns used to compute Range partition IDs.

<a id="op-68cdd36ae45c3506b78d2f3f"></a>
## sort_options

`function` · `datafusion_physical_plan::repartition::RangeExpr::sort_options` · datafusion-physical-plan 55.1.0

```rust
fn sort_options(&self) -> &[SortOptions]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RangeExpr", "path": "RangeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [663, 1], "end": [714, 2], "filename": "src/repartition/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/repartition/mod.rs:711`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the per-key sort options used for routing.

<a id="op-fecb39f516e975e4eb030214"></a>
## split_points

`function` · `datafusion_physical_plan::repartition::RangeExpr::split_points` · datafusion-physical-plan 55.1.0

```rust
fn split_points(&self) -> &[SplitPoint]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RangeExpr", "path": "RangeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [663, 1], "end": [714, 2], "filename": "src/repartition/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/repartition/mod.rs:706`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the Range split points used for routing.

<a id="op-67534248564d7017a50333a2"></a>
## try_from_proto

`function` · `datafusion_physical_plan::repartition::RangeExpr::try_from_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_from_proto(node: &protobuf::PhysicalExprNode, ctx: &datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> Result<PhysicalExprRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RangeExpr", "path": "RangeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [812, 1], "end": [846, 2], "filename": "src/repartition/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/repartition/mod.rs:814`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Reconstructs a [`RangeExpr`](../operations/datafusion_physical_plan.repartition.RangeExpr.md#op-b27390fffa4862a858121832) from its protobuf representation.

<a id="op-77e51517ca988332cbd645de"></a>
## try_new

`function` · `datafusion_physical_plan::repartition::RangeExpr::try_new` · datafusion-physical-plan 55.1.0

```rust
fn try_new(on_columns: Vec<PhysicalExprRef>, range_partitioning: &RangePartitioning) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RangeExpr", "path": "RangeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [663, 1], "end": [714, 2], "filename": "src/repartition/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/repartition/mod.rs:666`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Creates a Range expression for `on_columns` using the supplied routing
metadata.

<a id="op-3a00d37c1c3eb610a21bc268"></a>
## try_to_proto

`function` · `datafusion_physical_plan::repartition::RangeExpr::try_to_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_to_proto(&self, ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<protobuf::PhysicalExprNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RangeExpr", "path": "RangeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [722, 1], "end": [809, 2], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/repartition/mod.rs:774`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2cfba500da0b2bf27d69c8ac"></a>
## with_new_children

`function` · `datafusion_physical_plan::repartition::RangeExpr::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<PhysicalExprRef>) -> Result<PhysicalExprRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::RangeExpr", "path": "RangeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [722, 1], "end": [809, 2], "filename": "src/repartition/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/repartition/mod.rs:727`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
