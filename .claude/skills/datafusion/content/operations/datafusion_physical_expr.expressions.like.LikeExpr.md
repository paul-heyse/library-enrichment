# `datafusion_physical_expr::expressions::like::LikeExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.expressions.like.LikeExpr.json).

<a id="op-c063df46dae7fec58e7f402c"></a>
## LikeExpr

`struct` · `datafusion_physical_expr::expressions::like::LikeExpr` · datafusion-physical-expr 55.1.0

```rust
struct LikeExpr
```

Source: `src/expressions/like.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-87153532031659a41086c4e6"></a>
## case_insensitive

`function` · `datafusion_physical_expr::expressions::like::LikeExpr::case_insensitive` · datafusion-physical-expr 55.1.0

```rust
fn case_insensitive(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::like::LikeExpr", "path": "LikeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [99, 2], "filename": "src/expressions/like.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/like.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Is case insensitive

<a id="op-32afc1830e460c41372c5969"></a>
## children

`function` · `datafusion_physical_expr::expressions::like::LikeExpr::children` · datafusion-physical-expr 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::like::LikeExpr", "path": "LikeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [168, 2], "filename": "src/expressions/like.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/like.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-afc1e8ce4bcf3b55510f07f3"></a>
## data_type

`function` · `datafusion_physical_expr::expressions::like::LikeExpr::data_type` · datafusion-physical-expr 55.1.0

```rust
fn data_type(&self, _input_schema: &Schema) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::like::LikeExpr", "path": "LikeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [168, 2], "filename": "src/expressions/like.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/like.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9061ddf04943795d25b751a6"></a>
## eq

`function` · `datafusion_physical_expr::expressions::like::LikeExpr::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::like::LikeExpr", "path": "LikeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [44, 2], "filename": "src/expressions/like.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expressions/like.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6374ab4c00ce28deb6aeb31d"></a>
## evaluate

`function` · `datafusion_physical_expr::expressions::like::LikeExpr::evaluate` · datafusion-physical-expr 55.1.0

```rust
fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::like::LikeExpr", "path": "LikeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [168, 2], "filename": "src/expressions/like.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/like.rs:116`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81315f0c89e4a9a286cd5543"></a>
## expr

`function` · `datafusion_physical_expr::expressions::like::LikeExpr::expr` · datafusion-physical-expr 55.1.0

```rust
fn expr(&self) -> &Arc<dyn PhysicalExpr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::like::LikeExpr", "path": "LikeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [99, 2], "filename": "src/expressions/like.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/like.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Input expression

<a id="op-3fe0af5da51452c9691bed2e"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::like::LikeExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::like::LikeExpr", "path": "LikeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [105, 2], "filename": "src/expressions/like.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/expressions/like.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca4f76b9da8de8642bda4bc5"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::like::LikeExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::like::LikeExpr", "path": "LikeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [28, 10], "end": [28, 15], "filename": "src/expressions/like.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expressions/like.rs:28`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e4e9ec4491dbb030bfdb98a"></a>
## fmt_sql

`function` · `datafusion_physical_expr::expressions::like::LikeExpr::fmt_sql` · datafusion-physical-expr 55.1.0

```rust
fn fmt_sql(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::like::LikeExpr", "path": "LikeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [168, 2], "filename": "src/expressions/like.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/like.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ba754418fb07c5268e08b04"></a>
## hash

`function` · `datafusion_physical_expr::expressions::like::LikeExpr::hash` · datafusion-physical-expr 55.1.0

```rust
fn hash<H: std::hash::Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::like::LikeExpr", "path": "LikeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [53, 2], "filename": "src/expressions/like.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expressions/like.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31fd1322ec230d90f2848235"></a>
## negated

`function` · `datafusion_physical_expr::expressions::like::LikeExpr::negated` · datafusion-physical-expr 55.1.0

```rust
fn negated(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::like::LikeExpr", "path": "LikeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [99, 2], "filename": "src/expressions/like.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/like.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Is negated

<a id="op-d640ac2762bab989f994bdf2"></a>
## new

`function` · `datafusion_physical_expr::expressions::like::LikeExpr::new` · datafusion-physical-expr 55.1.0

```rust
fn new(negated: bool, case_insensitive: bool, expr: Arc<dyn PhysicalExpr>, pattern: Arc<dyn PhysicalExpr>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::like::LikeExpr", "path": "LikeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [99, 2], "filename": "src/expressions/like.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/like.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-072b3c99ec2d70c69ad38753"></a>
## nullable

`function` · `datafusion_physical_expr::expressions::like::LikeExpr::nullable` · datafusion-physical-expr 55.1.0

```rust
fn nullable(&self, input_schema: &Schema) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::like::LikeExpr", "path": "LikeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [168, 2], "filename": "src/expressions/like.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/like.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5da762dfd948c70cb8a31d33"></a>
## pattern

`function` · `datafusion_physical_expr::expressions::like::LikeExpr::pattern` · datafusion-physical-expr 55.1.0

```rust
fn pattern(&self) -> &Arc<dyn PhysicalExpr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::like::LikeExpr", "path": "LikeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [99, 2], "filename": "src/expressions/like.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/like.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Pattern expression

<a id="op-370a4f7325b45ff117a3d281"></a>
## try_from_proto

`function` · `datafusion_physical_expr::expressions::like::LikeExpr::try_from_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalExprNode, ctx: &datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::like::LikeExpr", "path": "LikeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 1], "end": [207, 2], "filename": "src/expressions/like.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/like.rs:179`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Reconstruct a [`LikeExpr`](../operations/datafusion_physical_expr.expressions.like.LikeExpr.md#op-c063df46dae7fec58e7f402c) from its protobuf representation.

Takes the whole [`PhysicalExprNode`] so the decode signature matches
other migrated expressions and can inspect outer-node metadata if
needed in the future.

[`PhysicalExprNode`]: datafusion_proto_models::protobuf::PhysicalExprNode

<a id="op-c7d39c812936072112d9d5f2"></a>
## try_to_proto

`function` · `datafusion_physical_expr::expressions::like::LikeExpr::try_to_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_to_proto(&self, ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::like::LikeExpr", "path": "LikeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [168, 2], "filename": "src/expressions/like.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/like.rs:150`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3523a0eb85d2383c03b9f33"></a>
## with_new_children

`function` · `datafusion_physical_expr::expressions::like::LikeExpr::with_new_children` · datafusion-physical-expr 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::like::LikeExpr", "path": "LikeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [168, 2], "filename": "src/expressions/like.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/like.rs:131`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
