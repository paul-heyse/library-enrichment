# `datafusion_physical_expr::expressions::is_not_null::IsNotNullExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.expressions.is_not_null.IsNotNullExpr.json).

<a id="op-3d8c42ad03cc8b72d67c47fd"></a>
## IsNotNullExpr

`struct` · `datafusion_physical_expr::expressions::is_not_null::IsNotNullExpr` · datafusion-physical-expr 55.1.0

```rust
struct IsNotNullExpr
```

Source: `src/expressions/is_not_null.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

IS NOT NULL expression

<a id="op-445a61fd279b1ba759eab3f0"></a>
## arg

`function` · `datafusion_physical_expr::expressions::is_not_null::IsNotNullExpr::arg` · datafusion-physical-expr 55.1.0

```rust
fn arg(&self) -> &Arc<dyn PhysicalExpr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::is_not_null::IsNotNullExpr", "path": "IsNotNullExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [60, 2], "filename": "src/expressions/is_not_null.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/is_not_null.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Get the input expression

<a id="op-c35bbc0418ffb3b2bc4dbda5"></a>
## children

`function` · `datafusion_physical_expr::expressions::is_not_null::IsNotNullExpr::children` · datafusion-physical-expr 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::is_not_null::IsNotNullExpr", "path": "IsNotNullExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [122, 2], "filename": "src/expressions/is_not_null.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/is_not_null.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a50096bcb33ab58ace7ca89"></a>
## data_type

`function` · `datafusion_physical_expr::expressions::is_not_null::IsNotNullExpr::data_type` · datafusion-physical-expr 55.1.0

```rust
fn data_type(&self, _input_schema: &Schema) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::is_not_null::IsNotNullExpr", "path": "IsNotNullExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [122, 2], "filename": "src/expressions/is_not_null.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/is_not_null.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34ff27c0f6e5e78e30347128"></a>
## eq

`function` · `datafusion_physical_expr::expressions::is_not_null::IsNotNullExpr::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::is_not_null::IsNotNullExpr", "path": "IsNotNullExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [42, 2], "filename": "src/expressions/is_not_null.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expressions/is_not_null.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ce8ce54ad71c05ffb98c9bb"></a>
## evaluate

`function` · `datafusion_physical_expr::expressions::is_not_null::IsNotNullExpr::evaluate` · datafusion-physical-expr 55.1.0

```rust
fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::is_not_null::IsNotNullExpr", "path": "IsNotNullExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [122, 2], "filename": "src/expressions/is_not_null.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/is_not_null.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2166cc474201f10dd35d801d"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::is_not_null::IsNotNullExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::is_not_null::IsNotNullExpr", "path": "IsNotNullExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 10], "end": [31, 15], "filename": "src/expressions/is_not_null.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expressions/is_not_null.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef73cb4ea069dfddc35d88c2"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::is_not_null::IsNotNullExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::is_not_null::IsNotNullExpr", "path": "IsNotNullExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [66, 2], "filename": "src/expressions/is_not_null.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/expressions/is_not_null.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3eacaa126649a8ac3272197f"></a>
## fmt_sql

`function` · `datafusion_physical_expr::expressions::is_not_null::IsNotNullExpr::fmt_sql` · datafusion-physical-expr 55.1.0

```rust
fn fmt_sql(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::is_not_null::IsNotNullExpr", "path": "IsNotNullExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [122, 2], "filename": "src/expressions/is_not_null.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/is_not_null.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27a3fdd4b48d1507e782f16b"></a>
## hash

`function` · `datafusion_physical_expr::expressions::is_not_null::IsNotNullExpr::hash` · datafusion-physical-expr 55.1.0

```rust
fn hash<H: std::hash::Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::is_not_null::IsNotNullExpr", "path": "IsNotNullExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [48, 2], "filename": "src/expressions/is_not_null.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expressions/is_not_null.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91de789a86d972d6f9628186"></a>
## new

`function` · `datafusion_physical_expr::expressions::is_not_null::IsNotNullExpr::new` · datafusion-physical-expr 55.1.0

```rust
fn new(arg: Arc<dyn PhysicalExpr>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::is_not_null::IsNotNullExpr", "path": "IsNotNullExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [60, 2], "filename": "src/expressions/is_not_null.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/is_not_null.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create new not expression

<a id="op-6c08334390319257d07e8b0e"></a>
## nullable

`function` · `datafusion_physical_expr::expressions::is_not_null::IsNotNullExpr::nullable` · datafusion-physical-expr 55.1.0

```rust
fn nullable(&self, _input_schema: &Schema) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::is_not_null::IsNotNullExpr", "path": "IsNotNullExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [122, 2], "filename": "src/expressions/is_not_null.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/is_not_null.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e533d4b8ad3ec40c4c2363c1"></a>
## try_from_proto

`function` · `datafusion_physical_expr::expressions::is_not_null::IsNotNullExpr::try_from_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalExprNode, ctx: &datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::is_not_null::IsNotNullExpr", "path": "IsNotNullExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [125, 1], "end": [147, 2], "filename": "src/expressions/is_not_null.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/is_not_null.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Reconstruct an [`IsNotNullExpr`](../operations/datafusion_physical_expr.expressions.is_not_null.IsNotNullExpr.md#op-3d8c42ad03cc8b72d67c47fd) from its protobuf representation.

<a id="op-d579054ebd9eeecf18269a4e"></a>
## try_to_proto

`function` · `datafusion_physical_expr::expressions::is_not_null::IsNotNullExpr::try_to_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_to_proto(&self, ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::is_not_null::IsNotNullExpr", "path": "IsNotNullExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [122, 2], "filename": "src/expressions/is_not_null.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/is_not_null.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2076a856631129dec71ac2b"></a>
## with_new_children

`function` · `datafusion_physical_expr::expressions::is_not_null::IsNotNullExpr::with_new_children` · datafusion-physical-expr 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::is_not_null::IsNotNullExpr", "path": "IsNotNullExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [122, 2], "filename": "src/expressions/is_not_null.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/is_not_null.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
