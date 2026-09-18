# `datafusion_physical_expr::expressions::unknown_column::UnKnownColumn`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.expressions.unknown_column.UnKnownColumn.json).

<a id="op-f60e7f35be7b68f3037f5fcf"></a>
## UnKnownColumn

`struct` · `datafusion_physical_expr::expressions::unknown_column::UnKnownColumn` · datafusion-physical-expr 55.1.0

```rust
struct UnKnownColumn
```

Source: `src/expressions/unknown_column.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-75c44f9c69b2395024e8d0d9"></a>
## children

`function` · `datafusion_physical_expr::expressions::unknown_column::UnKnownColumn::children` · datafusion-physical-expr 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::unknown_column::UnKnownColumn", "path": "UnKnownColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [105, 2], "filename": "src/expressions/unknown_column.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/unknown_column.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2eeed892ff66d58fe81dc2cb"></a>
## clone

`function` · `datafusion_physical_expr::expressions::unknown_column::UnKnownColumn::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> UnKnownColumn
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::unknown_column::UnKnownColumn", "path": "UnKnownColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 17], "end": [33, 22], "filename": "src/expressions/unknown_column.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expressions/unknown_column.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-17c4fd5b99c60475018997a0"></a>
## data_type

`function` · `datafusion_physical_expr::expressions::unknown_column::UnKnownColumn::data_type` · datafusion-physical-expr 55.1.0

```rust
fn data_type(&self, _input_schema: &Schema) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::unknown_column::UnKnownColumn", "path": "UnKnownColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [105, 2], "filename": "src/expressions/unknown_column.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/unknown_column.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Get the data type of this expression, given the schema of the input

<a id="op-a623224c180d44d93d9671dd"></a>
## eq

`function` · `datafusion_physical_expr::expressions::unknown_column::UnKnownColumn::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, _other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::unknown_column::UnKnownColumn", "path": "UnKnownColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [138, 2], "filename": "src/expressions/unknown_column.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expressions/unknown_column.rs:133`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b17bcdca6efc16fecc7bf74"></a>
## evaluate

`function` · `datafusion_physical_expr::expressions::unknown_column::UnKnownColumn::evaluate` · datafusion-physical-expr 55.1.0

```rust
fn evaluate(&self, _batch: &RecordBatch) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::unknown_column::UnKnownColumn", "path": "UnKnownColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [105, 2], "filename": "src/expressions/unknown_column.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/unknown_column.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Evaluate the expression

<a id="op-9159b0ddfe810aabd0e4143b"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::unknown_column::UnKnownColumn::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::unknown_column::UnKnownColumn", "path": "UnKnownColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [56, 2], "filename": "src/expressions/unknown_column.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/expressions/unknown_column.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9420a32ebdc56ecd2f7237c8"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::unknown_column::UnKnownColumn::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::unknown_column::UnKnownColumn", "path": "UnKnownColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 10], "end": [33, 15], "filename": "src/expressions/unknown_column.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expressions/unknown_column.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-357890e6b84ef69d0db9b3fe"></a>
## fmt_sql

`function` · `datafusion_physical_expr::expressions::unknown_column::UnKnownColumn::fmt_sql` · datafusion-physical-expr 55.1.0

```rust
fn fmt_sql(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::unknown_column::UnKnownColumn", "path": "UnKnownColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [105, 2], "filename": "src/expressions/unknown_column.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/unknown_column.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9fc3215cacf6e3651ee6c3c"></a>
## hash

`function` · `datafusion_physical_expr::expressions::unknown_column::UnKnownColumn::hash` · datafusion-physical-expr 55.1.0

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::unknown_column::UnKnownColumn", "path": "UnKnownColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [126, 1], "end": [130, 2], "filename": "src/expressions/unknown_column.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expressions/unknown_column.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a95cd167fba19864e056f45c"></a>
## name

`function` · `datafusion_physical_expr::expressions::unknown_column::UnKnownColumn::name` · datafusion-physical-expr 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::unknown_column::UnKnownColumn", "path": "UnKnownColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [50, 2], "filename": "src/expressions/unknown_column.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/unknown_column.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Get the column name

<a id="op-b580072390aac81c71964494"></a>
## new

`function` · `datafusion_physical_expr::expressions::unknown_column::UnKnownColumn::new` · datafusion-physical-expr 55.1.0

```rust
fn new(name: &str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::unknown_column::UnKnownColumn", "path": "UnKnownColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [50, 2], "filename": "src/expressions/unknown_column.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/unknown_column.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a new unknown column expression

<a id="op-a2d4362e6f315cdca87fd599"></a>
## nullable

`function` · `datafusion_physical_expr::expressions::unknown_column::UnKnownColumn::nullable` · datafusion-physical-expr 55.1.0

```rust
fn nullable(&self, _input_schema: &Schema) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::unknown_column::UnKnownColumn", "path": "UnKnownColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [105, 2], "filename": "src/expressions/unknown_column.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/unknown_column.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Decide whether this expression is nullable, given the schema of the input

<a id="op-c16289b6173d1a55e16f31dc"></a>
## try_from_proto

`function` · `datafusion_physical_expr::expressions::unknown_column::UnKnownColumn::try_from_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalExprNode, _ctx: &datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::unknown_column::UnKnownColumn", "path": "UnKnownColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [124, 2], "filename": "src/expressions/unknown_column.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/unknown_column.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Reconstruct an [`UnKnownColumn`](../operations/datafusion_physical_expr.expressions.unknown_column.UnKnownColumn.md#op-f60e7f35be7b68f3037f5fcf) from its protobuf representation.

<a id="op-30874fa5703e32a2115120a4"></a>
## try_to_proto

`function` · `datafusion_physical_expr::expressions::unknown_column::UnKnownColumn::try_to_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_to_proto(&self, _ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::unknown_column::UnKnownColumn", "path": "UnKnownColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [105, 2], "filename": "src/expressions/unknown_column.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/unknown_column.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06935360991f37b3153acd74"></a>
## with_new_children

`function` · `datafusion_physical_expr::expressions::unknown_column::UnKnownColumn::with_new_children` · datafusion-physical-expr 55.1.0

```rust
fn with_new_children(Arc<self>, _children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::unknown_column::UnKnownColumn", "path": "UnKnownColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [105, 2], "filename": "src/expressions/unknown_column.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/unknown_column.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
