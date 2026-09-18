# `datafusion_physical_expr::expressions::lambda_variable::LambdaVariable`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.expressions.lambda_variable.LambdaVariable.json).

<a id="op-2851bfdd6803d9f01ee53ea2"></a>
## LambdaVariable

`struct` · `datafusion_physical_expr::expressions::lambda_variable::LambdaVariable` · datafusion-physical-expr 55.1.0

```rust
struct LambdaVariable
```

Source: `src/expressions/lambda_variable.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Represents the lambda variable with a given index and field

<a id="op-45dac8c0ec92571604b4aace"></a>
## children

`function` · `datafusion_physical_expr::expressions::lambda_variable::LambdaVariable::children` · datafusion-physical-expr 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda_variable::LambdaVariable", "path": "LambdaVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [109, 1], "end": [182, 2], "filename": "src/expressions/lambda_variable.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/lambda_variable.rs:150`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-404d55ed6cde9172038a979f"></a>
## clone

`function` · `datafusion_physical_expr::expressions::lambda_variable::LambdaVariable::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> LambdaVariable
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda_variable::LambdaVariable", "path": "LambdaVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 17], "end": [34, 22], "filename": "src/expressions/lambda_variable.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expressions/lambda_variable.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b5e6ef47c9fa8b1132c2801"></a>
## data_type

`function` · `datafusion_physical_expr::expressions::lambda_variable::LambdaVariable::data_type` · datafusion-physical-expr 55.1.0

```rust
fn data_type(&self, _input_schema: &Schema) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda_variable::LambdaVariable", "path": "LambdaVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [109, 1], "end": [182, 2], "filename": "src/expressions/lambda_variable.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/lambda_variable.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-539944a40f2a8602bea95a82"></a>
## eq

`function` · `datafusion_physical_expr::expressions::lambda_variable::LambdaVariable::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda_variable::LambdaVariable", "path": "LambdaVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [46, 2], "filename": "src/expressions/lambda_variable.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expressions/lambda_variable.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fcaf00f10f796899dad9ab49"></a>
## evaluate

`function` · `datafusion_physical_expr::expressions::lambda_variable::LambdaVariable::evaluate` · datafusion-physical-expr 55.1.0

```rust
fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda_variable::LambdaVariable", "path": "LambdaVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [109, 1], "end": [182, 2], "filename": "src/expressions/lambda_variable.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/lambda_variable.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14f6d8170ef1285fdb45f928"></a>
## field

`function` · `datafusion_physical_expr::expressions::lambda_variable::LambdaVariable::field` · datafusion-physical-expr 55.1.0

```rust
fn field(&self) -> &FieldRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda_variable::LambdaVariable", "path": "LambdaVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [101, 2], "filename": "src/expressions/lambda_variable.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/lambda_variable.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Get the variable's field

<a id="op-04c834a8b417f8d21e8fdc92"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::lambda_variable::LambdaVariable::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda_variable::LambdaVariable", "path": "LambdaVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 10], "end": [34, 15], "filename": "src/expressions/lambda_variable.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expressions/lambda_variable.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1f641c54a2abda1b857d89c"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::lambda_variable::LambdaVariable::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda_variable::LambdaVariable", "path": "LambdaVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 1], "end": [107, 2], "filename": "src/expressions/lambda_variable.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/expressions/lambda_variable.rs:104`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-efcde70abcedab5fdd8846f3"></a>
## fmt_sql

`function` · `datafusion_physical_expr::expressions::lambda_variable::LambdaVariable::fmt_sql` · datafusion-physical-expr 55.1.0

```rust
fn fmt_sql(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda_variable::LambdaVariable", "path": "LambdaVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [109, 1], "end": [182, 2], "filename": "src/expressions/lambda_variable.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/lambda_variable.rs:161`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1cf30b56c17c1a17a8919327"></a>
## hash

`function` · `datafusion_physical_expr::expressions::lambda_variable::LambdaVariable::hash` · datafusion-physical-expr 55.1.0

```rust
fn hash<H: std::hash::Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda_variable::LambdaVariable", "path": "LambdaVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [53, 2], "filename": "src/expressions/lambda_variable.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expressions/lambda_variable.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06a90c29d41410077264a307"></a>
## index

`function` · `datafusion_physical_expr::expressions::lambda_variable::LambdaVariable::index` · datafusion-physical-expr 55.1.0

```rust
fn index(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda_variable::LambdaVariable", "path": "LambdaVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [101, 2], "filename": "src/expressions/lambda_variable.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/lambda_variable.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Get the variable's index

<a id="op-72abb70357f662d942e59acc"></a>
## name

`function` · `datafusion_physical_expr::expressions::lambda_variable::LambdaVariable::name` · datafusion-physical-expr 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda_variable::LambdaVariable", "path": "LambdaVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [101, 2], "filename": "src/expressions/lambda_variable.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/lambda_variable.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Get the variable's name

<a id="op-64ef0a69da5a1b92bdb15145"></a>
## new

`function` · `datafusion_physical_expr::expressions::lambda_variable::LambdaVariable::new` · datafusion-physical-expr 55.1.0

```rust
fn new(index: usize, field: FieldRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda_variable::LambdaVariable", "path": "LambdaVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [101, 2], "filename": "src/expressions/lambda_variable.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/lambda_variable.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a new lambda variable expression

<a id="op-3b9e1416ae82a6cffdcf4310"></a>
## nullable

`function` · `datafusion_physical_expr::expressions::lambda_variable::LambdaVariable::nullable` · datafusion-physical-expr 55.1.0

```rust
fn nullable(&self, _input_schema: &Schema) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda_variable::LambdaVariable", "path": "LambdaVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [109, 1], "end": [182, 2], "filename": "src/expressions/lambda_variable.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/lambda_variable.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d261b0950cc89ea0ce7e5d06"></a>
## return_field

`function` · `datafusion_physical_expr::expressions::lambda_variable::LambdaVariable::return_field` · datafusion-physical-expr 55.1.0

```rust
fn return_field(&self, _input_schema: &Schema) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda_variable::LambdaVariable", "path": "LambdaVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [109, 1], "end": [182, 2], "filename": "src/expressions/lambda_variable.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/lambda_variable.rs:146`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-215d101afc0d1961521d7b0d"></a>
## try_from_proto

`function` · `datafusion_physical_expr::expressions::lambda_variable::LambdaVariable::try_from_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalExprNode, _ctx: &datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda_variable::LambdaVariable", "path": "LambdaVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [101, 2], "filename": "src/expressions/lambda_variable.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/lambda_variable.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Reconstruct a [`LambdaVariable`](../operations/datafusion_physical_expr.expressions.lambda_variable.LambdaVariable.md#op-2851bfdd6803d9f01ee53ea2) from a proto node.

<a id="op-3e12421329d523add6e032f0"></a>
## try_to_proto

`function` · `datafusion_physical_expr::expressions::lambda_variable::LambdaVariable::try_to_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_to_proto(&self, _ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda_variable::LambdaVariable", "path": "LambdaVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [109, 1], "end": [182, 2], "filename": "src/expressions/lambda_variable.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/lambda_variable.rs:166`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-87393b6c11f9f58cc33890da"></a>
## with_new_children

`function` · `datafusion_physical_expr::expressions::lambda_variable::LambdaVariable::with_new_children` · datafusion-physical-expr 55.1.0

```rust
fn with_new_children(Arc<self>, _children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::lambda_variable::LambdaVariable", "path": "LambdaVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [109, 1], "end": [182, 2], "filename": "src/expressions/lambda_variable.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/lambda_variable.rs:154`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
