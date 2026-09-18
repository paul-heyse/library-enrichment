# `datafusion_physical_expr::expressions::is_null::IsNullExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.expressions.is_null.IsNullExpr.json).

<a id="op-1fc6c15332b76b8a01207b52"></a>
## IsNullExpr

`struct` · `datafusion_physical_expr::expressions::is_null::IsNullExpr` · datafusion-physical-expr 55.1.0

```rust
struct IsNullExpr
```

Source: `src/expressions/is_null.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

IS NULL expression

<a id="op-c89e5463a3b23154edd9e1e7"></a>
## arg

`function` · `datafusion_physical_expr::expressions::is_null::IsNullExpr::arg` · datafusion-physical-expr 55.1.0

```rust
fn arg(&self) -> &Arc<dyn PhysicalExpr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::is_null::IsNullExpr", "path": "IsNullExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [60, 2], "filename": "src/expressions/is_null.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/is_null.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Get the input expression

<a id="op-3cc4fb94a65ecc85a0955322"></a>
## children

`function` · `datafusion_physical_expr::expressions::is_null::IsNullExpr::children` · datafusion-physical-expr 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::is_null::IsNullExpr", "path": "IsNullExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [121, 2], "filename": "src/expressions/is_null.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/is_null.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-61c0fc2cd885f91e8cde4fac"></a>
## data_type

`function` · `datafusion_physical_expr::expressions::is_null::IsNullExpr::data_type` · datafusion-physical-expr 55.1.0

```rust
fn data_type(&self, _input_schema: &Schema) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::is_null::IsNullExpr", "path": "IsNullExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [121, 2], "filename": "src/expressions/is_null.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/is_null.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-395b8dbf50a4f4830ab986c0"></a>
## eq

`function` · `datafusion_physical_expr::expressions::is_null::IsNullExpr::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::is_null::IsNullExpr", "path": "IsNullExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [42, 2], "filename": "src/expressions/is_null.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expressions/is_null.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d9f7003f329a7cad9c188c6"></a>
## evaluate

`function` · `datafusion_physical_expr::expressions::is_null::IsNullExpr::evaluate` · datafusion-physical-expr 55.1.0

```rust
fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::is_null::IsNullExpr", "path": "IsNullExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [121, 2], "filename": "src/expressions/is_null.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/is_null.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b9ae113642d76d4eced382a"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::is_null::IsNullExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::is_null::IsNullExpr", "path": "IsNullExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 10], "end": [31, 15], "filename": "src/expressions/is_null.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expressions/is_null.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de4592289ee132c7b239ba25"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::is_null::IsNullExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::is_null::IsNullExpr", "path": "IsNullExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [66, 2], "filename": "src/expressions/is_null.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/expressions/is_null.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fdca9074aca812e292b129cf"></a>
## fmt_sql

`function` · `datafusion_physical_expr::expressions::is_null::IsNullExpr::fmt_sql` · datafusion-physical-expr 55.1.0

```rust
fn fmt_sql(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::is_null::IsNullExpr", "path": "IsNullExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [121, 2], "filename": "src/expressions/is_null.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/is_null.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70d0bfe9738cbd900c0f5eff"></a>
## hash

`function` · `datafusion_physical_expr::expressions::is_null::IsNullExpr::hash` · datafusion-physical-expr 55.1.0

```rust
fn hash<H: std::hash::Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::is_null::IsNullExpr", "path": "IsNullExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [48, 2], "filename": "src/expressions/is_null.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expressions/is_null.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-048bb97e3693bef233511737"></a>
## new

`function` · `datafusion_physical_expr::expressions::is_null::IsNullExpr::new` · datafusion-physical-expr 55.1.0

```rust
fn new(arg: Arc<dyn PhysicalExpr>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::is_null::IsNullExpr", "path": "IsNullExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [60, 2], "filename": "src/expressions/is_null.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/is_null.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create new not expression

<a id="op-d0f786bfda946f6acc17cb8e"></a>
## nullable

`function` · `datafusion_physical_expr::expressions::is_null::IsNullExpr::nullable` · datafusion-physical-expr 55.1.0

```rust
fn nullable(&self, _input_schema: &Schema) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::is_null::IsNullExpr", "path": "IsNullExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [121, 2], "filename": "src/expressions/is_null.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/is_null.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-149d244dda453523f1380ad9"></a>
## try_from_proto

`function` · `datafusion_physical_expr::expressions::is_null::IsNullExpr::try_from_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalExprNode, ctx: &datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::is_null::IsNullExpr", "path": "IsNullExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 1], "end": [143, 2], "filename": "src/expressions/is_null.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/is_null.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Reconstruct an [`IsNullExpr`](../operations/datafusion_physical_expr.expressions.is_null.IsNullExpr.md#op-1fc6c15332b76b8a01207b52) from its protobuf representation.

<a id="op-3290b6915c89445ba4f167ad"></a>
## try_to_proto

`function` · `datafusion_physical_expr::expressions::is_null::IsNullExpr::try_to_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_to_proto(&self, ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::is_null::IsNullExpr", "path": "IsNullExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [121, 2], "filename": "src/expressions/is_null.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/is_null.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6bc3c74048c5b5dab0887cbe"></a>
## with_new_children

`function` · `datafusion_physical_expr::expressions::is_null::IsNullExpr::with_new_children` · datafusion-physical-expr 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::is_null::IsNullExpr", "path": "IsNullExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [121, 2], "filename": "src/expressions/is_null.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/is_null.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
