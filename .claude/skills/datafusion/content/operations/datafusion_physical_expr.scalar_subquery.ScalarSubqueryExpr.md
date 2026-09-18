# `datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.scalar_subquery.ScalarSubqueryExpr.json).

<a id="op-4ace2e23e91e08dd59fef19d"></a>
## ScalarSubqueryExpr

`struct` · `datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr` · datafusion-physical-expr 55.1.0

```rust
struct ScalarSubqueryExpr
```

Source: `src/scalar_subquery.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

A physical expression whose value is provided by a scalar subquery.

Subquery execution is handled by `ScalarSubqueryExec`, which stores the
result in a shared [`ScalarSubqueryResults`](../operations/datafusion_expr.physical_planning_context.ScalarSubqueryResults.md#op-6b0683e0d87a9cc813cb37e5) container. This expression
simply reads from that container at the appropriate index.

<a id="op-efbb0962982749069665c664"></a>
## children

`function` · `datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr::children` · datafusion-physical-expr 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr", "path": "ScalarSubqueryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [177, 2], "filename": "src/scalar_subquery.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/scalar_subquery.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-832963e40748bd9c1c022a9c"></a>
## data_type

`function` · `datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr::data_type` · datafusion-physical-expr 55.1.0

```rust
fn data_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr", "path": "ScalarSubqueryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [90, 2], "filename": "src/scalar_subquery.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar_subquery.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce732efe702196bf8ea23680"></a>
## eq

`function` · `datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr", "path": "ScalarSubqueryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [114, 2], "filename": "src/scalar_subquery.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/scalar_subquery.rs:111`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7fe9d94790041dd0e42a3d28"></a>
## evaluate

`function` · `datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr::evaluate` · datafusion-physical-expr 55.1.0

```rust
fn evaluate(&self, _batch: &RecordBatch) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr", "path": "ScalarSubqueryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [177, 2], "filename": "src/scalar_subquery.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/scalar_subquery.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6479c4f6cd679e9f08ec86c9"></a>
## fmt

`function` · `datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr", "path": "ScalarSubqueryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 10], "end": [37, 15], "filename": "src/scalar_subquery.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/scalar_subquery.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f846e50075d9e64715b9543e"></a>
## fmt

`function` · `datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr", "path": "ScalarSubqueryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [99, 2], "filename": "src/scalar_subquery.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/scalar_subquery.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9aff4e95ee296b482be5a886"></a>
## fmt_sql

`function` · `datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr::fmt_sql` · datafusion-physical-expr 55.1.0

```rust
fn fmt_sql(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr", "path": "ScalarSubqueryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [177, 2], "filename": "src/scalar_subquery.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/scalar_subquery.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7b860693c1bc43b49868142"></a>
## get_properties

`function` · `datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr::get_properties` · datafusion-physical-expr 55.1.0

```rust
fn get_properties(&self, _children: &[ExprProperties]) -> Result<ExprProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr", "path": "ScalarSubqueryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [177, 2], "filename": "src/scalar_subquery.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/scalar_subquery.rs:147`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5e4d5669cbf445017b782e9"></a>
## hash

`function` · `datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr::hash` · datafusion-physical-expr 55.1.0

```rust
fn hash<H: std::hash::Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr", "path": "ScalarSubqueryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 1], "end": [108, 2], "filename": "src/scalar_subquery.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/scalar_subquery.rs:104`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b474b83c98ac0a859e690e6"></a>
## index

`function` · `datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr::index` · datafusion-physical-expr 55.1.0

```rust
fn index(&self) -> SubqueryIndex
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr", "path": "ScalarSubqueryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [90, 2], "filename": "src/scalar_subquery.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar_subquery.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Returns the index of this subquery in the shared results container.

<a id="op-a831210b2fd0fb2520097329"></a>
## new

`function` · `datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr::new` · datafusion-physical-expr 55.1.0

```rust
fn new(data_type: DataType, nullable: bool, index: SubqueryIndex, results: ScalarSubqueryResults) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr", "path": "ScalarSubqueryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [90, 2], "filename": "src/scalar_subquery.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar_subquery.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-438d3ce84a20d3ad933be2d8"></a>
## nullable

`function` · `datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr::nullable` · datafusion-physical-expr 55.1.0

```rust
fn nullable(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr", "path": "ScalarSubqueryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [90, 2], "filename": "src/scalar_subquery.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar_subquery.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-043f6a81096e5cc253e1349f"></a>
## results

`function` · `datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr::results` · datafusion-physical-expr 55.1.0

```rust
fn results(&self) -> &ScalarSubqueryResults
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr", "path": "ScalarSubqueryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [90, 2], "filename": "src/scalar_subquery.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar_subquery.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-abb5c2ab2c26ac7925d15389"></a>
## return_field

`function` · `datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr::return_field` · datafusion-physical-expr 55.1.0

```rust
fn return_field(&self, _input_schema: &Schema) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr", "path": "ScalarSubqueryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [177, 2], "filename": "src/scalar_subquery.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/scalar_subquery.rs:119`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd68103f330a8ecfab407922"></a>
## try_from_proto

`function` · `datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr::try_from_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalExprNode, _ctx: &datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>, results: &ScalarSubqueryResults) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr", "path": "ScalarSubqueryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [180, 1], "end": [217, 2], "filename": "src/scalar_subquery.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar_subquery.rs:190`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Reconstruct a [`ScalarSubqueryExpr`](../operations/datafusion_physical_expr.scalar_subquery.ScalarSubqueryExpr.md#op-4ace2e23e91e08dd59fef19d) from its protobuf representation.

Unlike other expressions, this takes a third argument: the shared
[`ScalarSubqueryResults`](../operations/datafusion_expr.physical_planning_context.ScalarSubqueryResults.md#op-6b0683e0d87a9cc813cb37e5) container. That container is a runtime-only
`Arc` shared with the surrounding `ScalarSubqueryExec` and is not part of
the wire format, so it cannot be reconstructed here or carried on the
decode context (which lives in a crate that cannot depend on
`datafusion-expr`). The match arm in `from_proto.rs` fetches it from the
plan-level decode context and passes it in.

<a id="op-8ba8a96b1d3d35cfc950c4b0"></a>
## try_to_proto

`function` · `datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr::try_to_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_to_proto(&self, _ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr", "path": "ScalarSubqueryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [177, 2], "filename": "src/scalar_subquery.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/scalar_subquery.rs:156`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22e06856f9fc7d4ab3d0b00a"></a>
## with_new_children

`function` · `datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr::with_new_children` · datafusion-physical-expr 55.1.0

```rust
fn with_new_children(Arc<self>, _children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr", "path": "ScalarSubqueryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [177, 2], "filename": "src/scalar_subquery.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/scalar_subquery.rs:140`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
