# `datafusion_physical_expr::expressions::literal::Literal`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.expressions.literal.Literal.json).

<a id="op-ea98b9107dc90c46e8d38265"></a>
## Literal

`struct` · `datafusion_physical_expr::expressions::literal::Literal` · datafusion-physical-expr 55.1.0

```rust
struct Literal
```

Source: `src/expressions/literal.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Represents a literal value

<a id="op-fb878df38d680bbd677f0d89"></a>
## children

`function` · `datafusion_physical_expr::expressions::literal::Literal::children` · datafusion-physical-expr 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::literal::Literal", "path": "Literal"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [153, 2], "filename": "src/expressions/literal.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/literal.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5e675d1c84796af0f85bee0"></a>
## clone

`function` · `datafusion_physical_expr::expressions::literal::Literal::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> Literal
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::literal::Literal", "path": "Literal"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 32], "end": [39, 37], "filename": "src/expressions/literal.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expressions/literal.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44b0c6cf647f6b59b3f94e6f"></a>
## data_type

`function` · `datafusion_physical_expr::expressions::literal::Literal::data_type` · datafusion-physical-expr 55.1.0

```rust
fn data_type(&self, _input_schema: &Schema) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::literal::Literal", "path": "Literal"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [153, 2], "filename": "src/expressions/literal.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/literal.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae732ca3096ed74def1185ab"></a>
## eq

`function` · `datafusion_physical_expr::expressions::literal::Literal::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, other: &Literal) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::literal::Literal", "path": "Literal"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 17], "end": [39, 26], "filename": "src/expressions/literal.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expressions/literal.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e809171201104bb8e4cc7d2e"></a>
## evaluate

`function` · `datafusion_physical_expr::expressions::literal::Literal::evaluate` · datafusion-physical-expr 55.1.0

```rust
fn evaluate(&self, _batch: &RecordBatch) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::literal::Literal", "path": "Literal"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [153, 2], "filename": "src/expressions/literal.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/literal.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-512f4e850ddd9063cdac5bba"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::literal::Literal::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::literal::Literal", "path": "Literal"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [91, 2], "filename": "src/expressions/literal.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/expressions/literal.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf34b420dd969cba71b180f5"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::literal::Literal::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::literal::Literal", "path": "Literal"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 10], "end": [39, 15], "filename": "src/expressions/literal.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expressions/literal.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e3bf5ef9726811420c3ed02"></a>
## fmt_sql

`function` · `datafusion_physical_expr::expressions::literal::Literal::fmt_sql` · datafusion-physical-expr 55.1.0

```rust
fn fmt_sql(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::literal::Literal", "path": "Literal"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [153, 2], "filename": "src/expressions/literal.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/literal.rs:131`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad0083732577be1c7ea58d8c"></a>
## get_properties

`function` · `datafusion_physical_expr::expressions::literal::Literal::get_properties` · datafusion-physical-expr 55.1.0

```rust
fn get_properties(&self, _children: &[ExprProperties]) -> Result<ExprProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::literal::Literal", "path": "Literal"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [153, 2], "filename": "src/expressions/literal.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/literal.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c671d352b75ac75f9f3f42b5"></a>
## hash

`function` · `datafusion_physical_expr::expressions::literal::Literal::hash` · datafusion-physical-expr 55.1.0

```rust
fn hash<H: std::hash::Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::literal::Literal", "path": "Literal"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [56, 2], "filename": "src/expressions/literal.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expressions/literal.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22ba4af24575d7f0e524a6b1"></a>
## new

`function` · `datafusion_physical_expr::expressions::literal::Literal::new` · datafusion-physical-expr 55.1.0

```rust
fn new(value: ScalarValue) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::literal::Literal", "path": "Literal"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [85, 2], "filename": "src/expressions/literal.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/literal.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a literal value expression

<a id="op-6ae09e20dcbaa39adae7f891"></a>
## new_with_metadata

`function` · `datafusion_physical_expr::expressions::literal::Literal::new_with_metadata` · datafusion-physical-expr 55.1.0

```rust
fn new_with_metadata(value: ScalarValue, metadata: Option<FieldMetadata>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::literal::Literal", "path": "Literal"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [85, 2], "filename": "src/expressions/literal.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/literal.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a literal value expression

<a id="op-8c73eeb4362986b7a8aeeb3b"></a>
## nullable

`function` · `datafusion_physical_expr::expressions::literal::Literal::nullable` · datafusion-physical-expr 55.1.0

```rust
fn nullable(&self, _input_schema: &Schema) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::literal::Literal", "path": "Literal"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [153, 2], "filename": "src/expressions/literal.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/literal.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4e52ba2f1c1857062f538d8"></a>
## placement

`function` · `datafusion_physical_expr::expressions::literal::Literal::placement` · datafusion-physical-expr 55.1.0

```rust
fn placement(&self) -> ExpressionPlacement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::literal::Literal", "path": "Literal"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [153, 2], "filename": "src/expressions/literal.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/literal.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d02a184744481e8a12d7cbdd"></a>
## return_field

`function` · `datafusion_physical_expr::expressions::literal::Literal::return_field` · datafusion-physical-expr 55.1.0

```rust
fn return_field(&self, _input_schema: &Schema) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::literal::Literal", "path": "Literal"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [153, 2], "filename": "src/expressions/literal.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/literal.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a63ce4b5efee42f47862a6c3"></a>
## try_from_proto

`function` · `datafusion_physical_expr::expressions::literal::Literal::try_from_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalExprNode, _ctx: &datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::literal::Literal", "path": "Literal"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 1], "end": [173, 2], "filename": "src/expressions/literal.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/literal.rs:158`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Reconstruct a [`Literal`](../operations/datafusion_physical_expr.expressions.literal.Literal.md#op-ea98b9107dc90c46e8d38265) from its protobuf representation.

<a id="op-df39387e2b3030a632db2e08"></a>
## try_to_proto

`function` · `datafusion_physical_expr::expressions::literal::Literal::try_to_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_to_proto(&self, _ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::literal::Literal", "path": "Literal"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [153, 2], "filename": "src/expressions/literal.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/literal.rs:140`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e7d858d4845a4870e4a1699"></a>
## value

`function` · `datafusion_physical_expr::expressions::literal::Literal::value` · datafusion-physical-expr 55.1.0

```rust
fn value(&self) -> &ScalarValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::literal::Literal", "path": "Literal"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [85, 2], "filename": "src/expressions/literal.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/literal.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Get the scalar value

<a id="op-ff4960d6bc47cb90253d8e63"></a>
## with_new_children

`function` · `datafusion_physical_expr::expressions::literal::Literal::with_new_children` · datafusion-physical-expr 55.1.0

```rust
fn with_new_children(Arc<self>, _children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::literal::Literal", "path": "Literal"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [153, 2], "filename": "src/expressions/literal.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/literal.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
