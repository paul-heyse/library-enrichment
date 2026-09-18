# `datafusion_physical_expr::expressions::lambda::LambdaExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.expressions.lambda.LambdaExpr.json).

<a id="op-a16ba2aae0fe27d74fa09d31"></a>
## LambdaExpr

`struct` · `datafusion_physical_expr::expressions::lambda::LambdaExpr` · datafusion-physical-expr 55.1.0

```rust
struct LambdaExpr
```

Source: `src/expressions/lambda.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Represents a lambda with the given parameters names and body

<a id="op-0ad190e7bae3d4233b28e15b"></a>
## body

`function` · `datafusion_physical_expr::expressions::lambda::LambdaExpr::body` · datafusion-physical-expr 55.1.0

```rust
fn body(&self) -> &Arc<dyn PhysicalExpr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda::LambdaExpr", "path": "LambdaExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [196, 2], "filename": "src/expressions/lambda.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/lambda.rs:153`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Get the lambda's body

<a id="op-4629c90e6807339227d7066d"></a>
## children

`function` · `datafusion_physical_expr::expressions::lambda::LambdaExpr::children` · datafusion-physical-expr 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda::LambdaExpr", "path": "LambdaExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [316, 2], "filename": "src/expressions/lambda.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/lambda.rs:275`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb9b3d4b7d29bf683ec3a4f6"></a>
## clone

`function` · `datafusion_physical_expr::expressions::lambda::LambdaExpr::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> LambdaExpr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda::LambdaExpr", "path": "LambdaExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 21], "end": [40, 26], "filename": "src/expressions/lambda.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expressions/lambda.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b1f38702dab09512e5a0441"></a>
## data_type

`function` · `datafusion_physical_expr::expressions::lambda::LambdaExpr::data_type` · datafusion-physical-expr 55.1.0

```rust
fn data_type(&self, _input_schema: &Schema) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda::LambdaExpr", "path": "LambdaExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [316, 2], "filename": "src/expressions/lambda.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/lambda.rs:263`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a948ec6b1d4241e676129631"></a>
## eq

`function` · `datafusion_physical_expr::expressions::lambda::LambdaExpr::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda::LambdaExpr", "path": "LambdaExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [54, 2], "filename": "src/expressions/lambda.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expressions/lambda.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cdcc1f7072c0096fe31c527e"></a>
## evaluate

`function` · `datafusion_physical_expr::expressions::lambda::LambdaExpr::evaluate` · datafusion-physical-expr 55.1.0

```rust
fn evaluate(&self, _batch: &RecordBatch) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda::LambdaExpr", "path": "LambdaExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [316, 2], "filename": "src/expressions/lambda.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/lambda.rs:271`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f7d09e29671bf7725118f59"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::lambda::LambdaExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda::LambdaExpr", "path": "LambdaExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 10], "end": [40, 15], "filename": "src/expressions/lambda.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expressions/lambda.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc3eb079fc2ad05a1e21b557"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::lambda::LambdaExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda::LambdaExpr", "path": "LambdaExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [256, 1], "end": [260, 2], "filename": "src/expressions/lambda.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/expressions/lambda.rs:257`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f25c7d90018293565885e23"></a>
## fmt_sql

`function` · `datafusion_physical_expr::expressions::lambda::LambdaExpr::fmt_sql` · datafusion-physical-expr 55.1.0

```rust
fn fmt_sql(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda::LambdaExpr", "path": "LambdaExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [316, 2], "filename": "src/expressions/lambda.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/lambda.rs:295`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8653942d7a4a4db4be5c757"></a>
## hash

`function` · `datafusion_physical_expr::expressions::lambda::LambdaExpr::hash` · datafusion-physical-expr 55.1.0

```rust
fn hash<H: std::hash::Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda::LambdaExpr", "path": "LambdaExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [61, 2], "filename": "src/expressions/lambda.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expressions/lambda.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ead73595328084b7807a820"></a>
## nullable

`function` · `datafusion_physical_expr::expressions::lambda::LambdaExpr::nullable` · datafusion-physical-expr 55.1.0

```rust
fn nullable(&self, _input_schema: &Schema) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda::LambdaExpr", "path": "LambdaExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [316, 2], "filename": "src/expressions/lambda.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/lambda.rs:267`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03a6244680c9e51f21e388ae"></a>
## params

`function` · `datafusion_physical_expr::expressions::lambda::LambdaExpr::params` · datafusion-physical-expr 55.1.0

```rust
fn params(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda::LambdaExpr", "path": "LambdaExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [196, 2], "filename": "src/expressions/lambda.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/lambda.rs:148`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Get the lambda's params names

<a id="op-341ca7712f653c7cc5c805bb"></a>
## try_from_proto

`function` · `datafusion_physical_expr::expressions::lambda::LambdaExpr::try_from_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalExprNode, ctx: &datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda::LambdaExpr", "path": "LambdaExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [196, 2], "filename": "src/expressions/lambda.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/lambda.rs:159`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Reconstruct a [`LambdaExpr`](../operations/datafusion_physical_expr.expressions.lambda.LambdaExpr.md#op-a16ba2aae0fe27d74fa09d31) from a proto node.

<a id="op-1e6c904a2c03e34b1b1b50b0"></a>
## try_new

`function` · `datafusion_physical_expr::expressions::lambda::LambdaExpr::try_new` · datafusion-physical-expr 55.1.0

```rust
fn try_new(params: Vec<String>, body: Arc<dyn PhysicalExpr>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda::LambdaExpr", "path": "LambdaExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [196, 2], "filename": "src/expressions/lambda.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/lambda.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a new lambda expression with the given parameters and body.

<a id="op-5c1706ce18c6da2366e8b45a"></a>
## try_to_proto

`function` · `datafusion_physical_expr::expressions::lambda::LambdaExpr::try_to_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_to_proto(&self, ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda::LambdaExpr", "path": "LambdaExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [316, 2], "filename": "src/expressions/lambda.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/lambda.rs:300`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ce6923474039a7a25bd22ae"></a>
## used_param_indices

`function` · `datafusion_physical_expr::expressions::lambda::LambdaExpr::used_param_indices` · datafusion-physical-expr 55.1.0

```rust
fn used_param_indices(&self) -> &[usize]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda::LambdaExpr", "path": "LambdaExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [196, 2], "filename": "src/expressions/lambda.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/lambda.rs:193`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Indices into [`params`](Self::params) of the parameters the body
actually references, in declaration order. See `CollectUsedVisitor`
in this module.

Relies on the planner appending each lambda's own params after
captures, matching the `captures ++ used_params` layout
`LambdaArgument::new` builds.

<a id="op-ebd550fbfd8e15f6670095d5"></a>
## with_new_children

`function` · `datafusion_physical_expr::expressions::lambda::LambdaExpr::with_new_children` · datafusion-physical-expr 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda::LambdaExpr", "path": "LambdaExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [316, 2], "filename": "src/expressions/lambda.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/lambda.rs:279`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
