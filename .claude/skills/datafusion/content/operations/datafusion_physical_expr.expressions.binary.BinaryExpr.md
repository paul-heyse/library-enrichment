# `datafusion_physical_expr::expressions::binary::BinaryExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.expressions.binary.BinaryExpr.json).

<a id="op-02fece14fd9be94e44c1a9c8"></a>
## BinaryExpr

`struct` · `datafusion_physical_expr::expressions::binary::BinaryExpr` · datafusion-physical-expr 55.1.0

```rust
struct BinaryExpr
```

Source: `src/expressions/binary.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Binary expression

<a id="op-0c3382d5ced3b439ba3a8996"></a>
## children

`function` · `datafusion_physical_expr::expressions::binary::BinaryExpr::children` · datafusion-physical-expr 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::binary::BinaryExpr", "path": "BinaryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [520, 1], "end": [961, 2], "filename": "src/expressions/binary.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/binary.rs:676`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c6d04c108f673de9d973c3b"></a>
## clone

`function` · `datafusion_physical_expr::expressions::binary::BinaryExpr::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> BinaryExpr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::binary::BinaryExpr", "path": "BinaryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 17], "end": [56, 22], "filename": "src/expressions/binary.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expressions/binary.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5bfb600fbe1e5d4aac04cbf3"></a>
## data_type

`function` · `datafusion_physical_expr::expressions::binary::BinaryExpr::data_type` · datafusion-physical-expr 55.1.0

```rust
fn data_type(&self, input_schema: &Schema) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::binary::BinaryExpr", "path": "BinaryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [520, 1], "end": [961, 2], "filename": "src/expressions/binary.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/binary.rs:521`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5fd658320abbf0d98d80866b"></a>
## eq

`function` · `datafusion_physical_expr::expressions::binary::BinaryExpr::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::binary::BinaryExpr", "path": "BinaryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [73, 2], "filename": "src/expressions/binary.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expressions/binary.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c8d36e2c8235cf4e9a6d471"></a>
## evaluate

`function` · `datafusion_physical_expr::expressions::binary::BinaryExpr::evaluate` · datafusion-physical-expr 55.1.0

```rust
fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::binary::BinaryExpr", "path": "BinaryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [520, 1], "end": [961, 2], "filename": "src/expressions/binary.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/binary.rs:534`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20e7308d751be6239ea155fb"></a>
## evaluate_bounds

`function` · `datafusion_physical_expr::expressions::binary::BinaryExpr::evaluate_bounds` · datafusion-physical-expr 55.1.0

```rust
fn evaluate_bounds(&self, children: &[&Interval]) -> Result<Interval>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::binary::BinaryExpr", "path": "BinaryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [520, 1], "end": [961, 2], "filename": "src/expressions/binary.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/binary.rs:690`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0117a1834f1c0d81082abe89"></a>
## evaluate_statistics

`function` · `datafusion_physical_expr::expressions::binary::BinaryExpr::evaluate_statistics` · datafusion-physical-expr 55.1.0

```rust
fn evaluate_statistics(&self, children: &[&Distribution]) -> Result<Distribution>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::binary::BinaryExpr", "path": "BinaryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [520, 1], "end": [961, 2], "filename": "src/expressions/binary.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/binary.rs:789`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5a0026b8696b74a29043575"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::binary::BinaryExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::binary::BinaryExpr", "path": "BinaryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 10], "end": [56, 15], "filename": "src/expressions/binary.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expressions/binary.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9f8d40a582b6dbf845f86a5"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::binary::BinaryExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::binary::BinaryExpr", "path": "BinaryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [217, 2], "filename": "src/expressions/binary.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/expressions/binary.rs:187`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41da10db40af066ef5e70628"></a>
## fmt_sql

`function` · `datafusion_physical_expr::expressions::binary::BinaryExpr::fmt_sql` · datafusion-physical-expr 55.1.0

```rust
fn fmt_sql(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::binary::BinaryExpr", "path": "BinaryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [520, 1], "end": [961, 2], "filename": "src/expressions/binary.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/binary.rs:894`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-504e6df0af9dcdd2449acbac"></a>
## get_properties

`function` · `datafusion_physical_expr::expressions::binary::BinaryExpr::get_properties` · datafusion-physical-expr 55.1.0

```rust
fn get_properties(&self, children: &[ExprProperties]) -> Result<ExprProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::binary::BinaryExpr", "path": "BinaryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [520, 1], "end": [961, 2], "filename": "src/expressions/binary.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/binary.rs:822`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

For each operator, [`BinaryExpr`](../operations/datafusion_physical_expr.expressions.binary.BinaryExpr.md#op-02fece14fd9be94e44c1a9c8) has distinct rules.
TODO: There may be rules specific to some data types and expression ranges.

<a id="op-016ca44384e414fdbbdf2826"></a>
## hash

`function` · `datafusion_physical_expr::expressions::binary::BinaryExpr::hash` · datafusion-physical-expr 55.1.0

```rust
fn hash<H: std::hash::Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::binary::BinaryExpr", "path": "BinaryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [81, 2], "filename": "src/expressions/binary.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expressions/binary.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ded01d1075c6ff8677a31aa5"></a>
## left

`function` · `datafusion_physical_expr::expressions::binary::BinaryExpr::left` · datafusion-physical-expr 55.1.0

```rust
fn left(&self) -> &Arc<dyn PhysicalExpr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::binary::BinaryExpr", "path": "BinaryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [155, 2], "filename": "src/expressions/binary.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/binary.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Get the left side of the binary expression

<a id="op-04fc8acb6f0da0dc771a5896"></a>
## new

`function` · `datafusion_physical_expr::expressions::binary::BinaryExpr::new` · datafusion-physical-expr 55.1.0

```rust
fn new(left: Arc<dyn PhysicalExpr>, op: Operator, right: Arc<dyn PhysicalExpr>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::binary::BinaryExpr", "path": "BinaryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [155, 2], "filename": "src/expressions/binary.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/binary.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create new binary expression

<a id="op-d3d91e680802e7e3db4390c4"></a>
## nullable

`function` · `datafusion_physical_expr::expressions::binary::BinaryExpr::nullable` · datafusion-physical-expr 55.1.0

```rust
fn nullable(&self, input_schema: &Schema) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::binary::BinaryExpr", "path": "BinaryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [520, 1], "end": [961, 2], "filename": "src/expressions/binary.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/binary.rs:530`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cba71092c34cff0dde7bbc60"></a>
## op

`function` · `datafusion_physical_expr::expressions::binary::BinaryExpr::op` · datafusion-physical-expr 55.1.0

```rust
fn op(&self) -> &Operator
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::binary::BinaryExpr", "path": "BinaryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [155, 2], "filename": "src/expressions/binary.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/binary.rs:119`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Get the operator for this binary expression

<a id="op-7ee9ab580032a40bb3fb6f92"></a>
## propagate_constraints

`function` · `datafusion_physical_expr::expressions::binary::BinaryExpr::propagate_constraints` · datafusion-physical-expr 55.1.0

```rust
fn propagate_constraints(&self, interval: &Interval, children: &[&Interval]) -> Result<Option<Vec<Interval>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::binary::BinaryExpr", "path": "BinaryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [520, 1], "end": [961, 2], "filename": "src/expressions/binary.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/binary.rs:698`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ce225b154b6da9fc1b4be63"></a>
## right

`function` · `datafusion_physical_expr::expressions::binary::BinaryExpr::right` · datafusion-physical-expr 55.1.0

```rust
fn right(&self) -> &Arc<dyn PhysicalExpr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::binary::BinaryExpr", "path": "BinaryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [155, 2], "filename": "src/expressions/binary.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/binary.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Get the right side of the binary expression

<a id="op-f3366af6f339521741ab9496"></a>
## try_from_proto

`function` · `datafusion_physical_expr::expressions::binary::BinaryExpr::try_from_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalExprNode, ctx: &datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::binary::BinaryExpr", "path": "BinaryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [964, 1], "end": [1021, 2], "filename": "src/expressions/binary.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/binary.rs:977`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Reconstruct a [`BinaryExpr`](../operations/datafusion_physical_expr.expressions.binary.BinaryExpr.md#op-02fece14fd9be94e44c1a9c8) (or a left-deep tree of them when the proto
uses the linearized `operands` form) from its protobuf representation.

Takes the whole [`PhysicalExprNode`] — the exact inverse of what
[`PhysicalExpr::try_to_proto`] produces — so every expression's
`try_from_proto` shares one signature. The operator string is parsed
via the canonical [`Operator::from_proto_name`] mapping, so no `op`
argument needs to be threaded in by the caller.

[`PhysicalExprNode`]: datafusion_proto_models::protobuf::PhysicalExprNode
[`PhysicalExpr::try_to_proto`]: datafusion_physical_expr_common::physical_expr::PhysicalExpr::try_to_proto
[`PhysicalExprDecodeCtx::decode`]: datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx::decode

Unresolved upstream links (retained, not inferred): `datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx::decode`, ``Operator::from_proto_name``.

<a id="op-c7dd34de0cd849a180796916"></a>
## try_to_proto

`function` · `datafusion_physical_expr::expressions::binary::BinaryExpr::try_to_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_to_proto(&self, ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::binary::BinaryExpr", "path": "BinaryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [520, 1], "end": [961, 2], "filename": "src/expressions/binary.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/binary.rs:921`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29e6e374ba35a927e3f1942d"></a>
## with_fail_on_overflow

`function` · `datafusion_physical_expr::expressions::binary::BinaryExpr::with_fail_on_overflow` · datafusion-physical-expr 55.1.0

```rust
fn with_fail_on_overflow(self, fail_on_overflow: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::binary::BinaryExpr", "path": "BinaryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [155, 2], "filename": "src/expressions/binary.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/binary.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create new binary expression with explicit fail_on_overflow value

<a id="op-950463850038387c0e78c3e7"></a>
## with_new_children

`function` · `datafusion_physical_expr::expressions::binary::BinaryExpr::with_new_children` · datafusion-physical-expr 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::binary::BinaryExpr", "path": "BinaryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [520, 1], "end": [961, 2], "filename": "src/expressions/binary.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/binary.rs:680`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
