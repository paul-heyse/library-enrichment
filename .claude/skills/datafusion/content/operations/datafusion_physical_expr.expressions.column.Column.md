# `datafusion_physical_expr::expressions::column::Column`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.expressions.column.Column.json).

<a id="op-8a412ca3bf3f178f4986ec28"></a>
## Column

`struct` · `datafusion_physical_expr::expressions::column::Column` · datafusion-physical-expr 55.1.0

```rust
struct Column
```

Source: `src/expressions/column.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Represents the column at a given index in a RecordBatch

This is a physical expression that represents a column at a given index in an
arrow [`Schema`](../operations/arrow_schema.schema.Schema.md#op-144709aa539d6163483c2050) / [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34).

Unlike the [logical `Expr::Column`], this expression is always resolved by schema index,
even though it does have a name. This is because the physical plan is always
resolved to a specific schema and there is no concept of "relation"

# Example:
 If the schema is `a`, `b`, `c` the `Column` for `b` would be represented by
 index 1, since `b` is the second column in the schema.

```
# use datafusion_physical_expr::expressions::Column;
# use arrow::datatypes::{DataType, Field, Schema};
// Schema with columns a, b, c
let schema = Schema::new(vec![
    Field::new("a", DataType::Int32, false),
    Field::new("b", DataType::Int32, false),
    Field::new("c", DataType::Int32, false),
]);

// reference to column b is index 1
let column_b = Column::new_with_schema("b", &schema).unwrap();
assert_eq!(column_b.index(), 1);

// reference to column c is index 2
let column_c = Column::new_with_schema("c", &schema).unwrap();
assert_eq!(column_c.index(), 2);
```
[logical `Expr::Column`]: https://docs.rs/datafusion/latest/datafusion/logical_expr/enum.Expr.html#variant.Column

<a id="op-1b6bf4ba654aa5e4a132726b"></a>
## children

`function` · `datafusion_physical_expr::expressions::column::Column::children` · datafusion-physical-expr 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [161, 2], "filename": "src/expressions/column.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/column.rs:131`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bcf3c760e9eae52131f88d4e"></a>
## clone

`function` · `datafusion_physical_expr::expressions::column::Column::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> Column
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 38], "end": [66, 43], "filename": "src/expressions/column.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expressions/column.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a01f05ee011824691862bc5"></a>
## data_type

`function` · `datafusion_physical_expr::expressions::column::Column::data_type` · datafusion-physical-expr 55.1.0

```rust
fn data_type(&self, input_schema: &Schema) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [161, 2], "filename": "src/expressions/column.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/column.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Get the data type of this expression, given the schema of the input

<a id="op-ff26feaa59df882e2a37e678"></a>
## eq

`function` · `datafusion_physical_expr::expressions::column::Column::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, other: &Column) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 23], "end": [66, 32], "filename": "src/expressions/column.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expressions/column.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df3161b603cdae036525aa2e"></a>
## evaluate

`function` · `datafusion_physical_expr::expressions::column::Column::evaluate` · datafusion-physical-expr 55.1.0

```rust
fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [161, 2], "filename": "src/expressions/column.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/column.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Evaluate the expression

<a id="op-1b4c5d102336d3d470723091"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::column::Column::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 10], "end": [66, 15], "filename": "src/expressions/column.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expressions/column.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ff1aab1370683a3f6ee5bfb"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::column::Column::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [105, 2], "filename": "src/expressions/column.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/expressions/column.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f574b88b9ed6d125a1f3b3a1"></a>
## fmt_sql

`function` · `datafusion_physical_expr::expressions::column::Column::fmt_sql` · datafusion-physical-expr 55.1.0

```rust
fn fmt_sql(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [161, 2], "filename": "src/expressions/column.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/column.rs:142`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7758adf03b389aa374de84e"></a>
## from

`function` · `datafusion_physical_expr::expressions::column::Column::from` · datafusion-physical-expr 55.1.0

```rust
fn from(c: &datafusion_proto_models::protobuf::PhysicalColumn) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [164, 1], "end": [168, 2], "filename": "src/expressions/column.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::PhysicalColumn", "path": "PhysicalColumn"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/expressions/column.rs:165`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7b0601623834fff60627955"></a>
## hash

`function` · `datafusion_physical_expr::expressions::column::Column::hash` · datafusion-physical-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 17], "end": [66, 21], "filename": "src/expressions/column.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expressions/column.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-024112f5861d789f4d444043"></a>
## index

`function` · `datafusion_physical_expr::expressions::column::Column::index` · datafusion-physical-expr 55.1.0

```rust
fn index(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [99, 2], "filename": "src/expressions/column.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/column.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Get the column's schema index

<a id="op-69ab903867c195f227074301"></a>
## name

`function` · `datafusion_physical_expr::expressions::column::Column::name` · datafusion-physical-expr 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [99, 2], "filename": "src/expressions/column.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/column.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Get the column's name

<a id="op-ab162067c2c4137e8184ca85"></a>
## new

`function` · `datafusion_physical_expr::expressions::column::Column::new` · datafusion-physical-expr 55.1.0

```rust
fn new(name: &str, index: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [99, 2], "filename": "src/expressions/column.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/column.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a new column expression which references the
column with the given index in the schema.

<a id="op-bde318c5eff979ba2c3f1bc6"></a>
## new_with_schema

`function` · `datafusion_physical_expr::expressions::column::Column::new_with_schema` · datafusion-physical-expr 55.1.0

```rust
fn new_with_schema(name: &str, schema: &Schema) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [99, 2], "filename": "src/expressions/column.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/column.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a new column expression which references the
column with the given name in the schema

<a id="op-51370e95e25e00dca5675f9e"></a>
## nullable

`function` · `datafusion_physical_expr::expressions::column::Column::nullable` · datafusion-physical-expr 55.1.0

```rust
fn nullable(&self, input_schema: &Schema) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [161, 2], "filename": "src/expressions/column.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/column.rs:115`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Decide whether this expression is nullable, given the schema of the input

<a id="op-7b6d69874f8947dbd406e345"></a>
## placement

`function` · `datafusion_physical_expr::expressions::column::Column::placement` · datafusion-physical-expr 55.1.0

```rust
fn placement(&self) -> ExpressionPlacement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [161, 2], "filename": "src/expressions/column.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/column.rs:146`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68ff00f941f2311a00d2be74"></a>
## return_field

`function` · `datafusion_physical_expr::expressions::column::Column::return_field` · datafusion-physical-expr 55.1.0

```rust
fn return_field(&self, input_schema: &Schema) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [161, 2], "filename": "src/expressions/column.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/column.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d1c7962187d461a9b219750"></a>
## try_from_proto

`function` · `datafusion_physical_expr::expressions::column::Column::try_from_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalExprNode, _ctx: &datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [206, 2], "filename": "src/expressions/column.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/column.rs:193`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Reconstruct a [`Column`](../operations/datafusion_physical_expr.expressions.column.Column.md#op-8a412ca3bf3f178f4986ec28) from its protobuf representation.

Takes the whole [`PhysicalExprNode`] — the exact inverse of what
[`PhysicalExpr::try_to_proto`] produces — so every expression's
`try_from_proto` shares one signature. The decode context is currently
unused, but is threaded through so that future expressions with child
sub-expressions can recurse via [`PhysicalExprDecodeCtx::decode`].

[`PhysicalExprNode`]: datafusion_proto_models::protobuf::PhysicalExprNode
[`PhysicalExpr::try_to_proto`]: datafusion_physical_expr_common::physical_expr::PhysicalExpr::try_to_proto
[`PhysicalExprDecodeCtx::decode`]: datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx::decode

Unresolved upstream links (retained, not inferred): `datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx::decode`.

<a id="op-de5182af0cc27641d8998838"></a>
## try_to_proto

`function` · `datafusion_physical_expr::expressions::column::Column::try_to_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_to_proto(&self, _ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [161, 2], "filename": "src/expressions/column.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/column.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f025e0f8d25b98fe02fff53"></a>
## with_new_children

`function` · `datafusion_physical_expr::expressions::column::Column::with_new_children` · datafusion-physical-expr 55.1.0

```rust
fn with_new_children(Arc<self>, _children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::column::Column", "path": "Column"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [161, 2], "filename": "src/expressions/column.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/column.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
