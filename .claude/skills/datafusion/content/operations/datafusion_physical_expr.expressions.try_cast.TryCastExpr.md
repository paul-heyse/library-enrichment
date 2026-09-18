# `datafusion_physical_expr::expressions::try_cast::TryCastExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.expressions.try_cast.TryCastExpr.json).

<a id="op-cea674f9bf9c028f5652a987"></a>
## TryCastExpr

`struct` · `datafusion_physical_expr::expressions::try_cast::TryCastExpr` · datafusion-physical-expr 55.1.0

```rust
struct TryCastExpr
```

Source: `src/expressions/try_cast.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

TRY_CAST expression casts an expression to a specific data type and returns NULL on invalid cast

<a id="op-0e0e3b2342e5fcb7f2d3d11c"></a>
## cast_type

`function` · `datafusion_physical_expr::expressions::try_cast::TryCastExpr::cast_type` · datafusion-physical-expr 55.1.0

```rust
fn cast_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::try_cast::TryCastExpr", "path": "TryCastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [139, 2], "filename": "src/expressions/try_cast.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/try_cast.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

The data type to cast to

<a id="op-db65ea2a47e810f1f24ffbaa"></a>
## children

`function` · `datafusion_physical_expr::expressions::try_cast::TryCastExpr::children` · datafusion-physical-expr 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::try_cast::TryCastExpr", "path": "TryCastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 1], "end": [237, 2], "filename": "src/expressions/try_cast.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/try_cast.rs:199`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a98b179d26d35c9a0fee1b93"></a>
## clone

`function` · `datafusion_physical_expr::expressions::try_cast::TryCastExpr::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> TryCastExpr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::try_cast::TryCastExpr", "path": "TryCastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 17], "end": [36, 22], "filename": "src/expressions/try_cast.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expressions/try_cast.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8cb76d97dea2cffb8982886f"></a>
## data_type

`function` · `datafusion_physical_expr::expressions::try_cast::TryCastExpr::data_type` · datafusion-physical-expr 55.1.0

```rust
fn data_type(&self, _input_schema: &Schema) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::try_cast::TryCastExpr", "path": "TryCastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 1], "end": [237, 2], "filename": "src/expressions/try_cast.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/try_cast.rs:148`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ecdf7de3554d6a33718beb6"></a>
## eq

`function` · `datafusion_physical_expr::expressions::try_cast::TryCastExpr::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::try_cast::TryCastExpr", "path": "TryCastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [63, 2], "filename": "src/expressions/try_cast.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expressions/try_cast.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5435359bd6cfe1bd57afd92e"></a>
## evaluate

`function` · `datafusion_physical_expr::expressions::try_cast::TryCastExpr::evaluate` · datafusion-physical-expr 55.1.0

```rust
fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::try_cast::TryCastExpr", "path": "TryCastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 1], "end": [237, 2], "filename": "src/expressions/try_cast.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/try_cast.rs:156`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b71193d4c1e8abe8cbfaec94"></a>
## expr

`function` · `datafusion_physical_expr::expressions::try_cast::TryCastExpr::expr` · datafusion-physical-expr 55.1.0

```rust
fn expr(&self) -> &Arc<dyn PhysicalExpr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::try_cast::TryCastExpr", "path": "TryCastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [139, 2], "filename": "src/expressions/try_cast.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/try_cast.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

The expression to cast

<a id="op-23fa4685dcf52c01bc31f3cc"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::try_cast::TryCastExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::try_cast::TryCastExpr", "path": "TryCastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 10], "end": [36, 15], "filename": "src/expressions/try_cast.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expressions/try_cast.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9dc2d6cff17cde30527e9ae6"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::try_cast::TryCastExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::try_cast::TryCastExpr", "path": "TryCastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [145, 2], "filename": "src/expressions/try_cast.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/expressions/try_cast.rs:142`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-788860aa1deda334ce37f5c5"></a>
## fmt_sql

`function` · `datafusion_physical_expr::expressions::try_cast::TryCastExpr::fmt_sql` · datafusion-physical-expr 55.1.0

```rust
fn fmt_sql(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::try_cast::TryCastExpr", "path": "TryCastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 1], "end": [237, 2], "filename": "src/expressions/try_cast.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/try_cast.rs:214`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd78111f30576c6be8eea1b5"></a>
## hash

`function` · `datafusion_physical_expr::expressions::try_cast::TryCastExpr::hash` · datafusion-physical-expr 55.1.0

```rust
fn hash<H: std::hash::Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::try_cast::TryCastExpr", "path": "TryCastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [79, 2], "filename": "src/expressions/try_cast.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expressions/try_cast.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5539f6114f9430b83c943f0e"></a>
## new

`function` · `datafusion_physical_expr::expressions::try_cast::TryCastExpr::new` · datafusion-physical-expr 55.1.0

```rust
fn new(expr: Arc<dyn PhysicalExpr>, cast_type: DataType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::try_cast::TryCastExpr", "path": "TryCastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [139, 2], "filename": "src/expressions/try_cast.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/try_cast.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a new `TryCastExpr` using only a `DataType`.

This constructor creates a type-only cast where metadata is passed through
from the source expression (with extension type keys stripped).
TRY_CAST results are always nullable since failed casts return NULL.

<a id="op-832f8f8f8525b06d5b3ed4ad"></a>
## new_with_target_field

`function` · `datafusion_physical_expr::expressions::try_cast::TryCastExpr::new_with_target_field` · datafusion-physical-expr 55.1.0

```rust
fn new_with_target_field(expr: Arc<dyn PhysicalExpr>, target_field: FieldRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::try_cast::TryCastExpr", "path": "TryCastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [139, 2], "filename": "src/expressions/try_cast.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/try_cast.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a new `TryCastExpr` with an explicit target `FieldRef`.

The provided `target_field` determines the output characteristics:
- The field's data type becomes the cast target type
- The field's metadata is used exactly as provided

TRY_CAST results are always nullable since failed casts return NULL.

See [`TryCastExpr::new`](../operations/datafusion_physical_expr.expressions.try_cast.TryCastExpr.md#op-5539f6114f9430b83c943f0e) for type-only casts where source metadata should
pass through.

<a id="op-cfe6a6939e05cade4f5e9f85"></a>
## nullable

`function` · `datafusion_physical_expr::expressions::try_cast::TryCastExpr::nullable` · datafusion-physical-expr 55.1.0

```rust
fn nullable(&self, _input_schema: &Schema) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::try_cast::TryCastExpr", "path": "TryCastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 1], "end": [237, 2], "filename": "src/expressions/try_cast.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/try_cast.rs:152`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f0ec73d3e78c03234348e66"></a>
## return_field

`function` · `datafusion_physical_expr::expressions::try_cast::TryCastExpr::return_field` · datafusion-physical-expr 55.1.0

```rust
fn return_field(&self, input_schema: &Schema) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::try_cast::TryCastExpr", "path": "TryCastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 1], "end": [237, 2], "filename": "src/expressions/try_cast.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/try_cast.rs:165`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-77c2a762bbdcd4b797d2c1de"></a>
## target_field

`function` · `datafusion_physical_expr::expressions::try_cast::TryCastExpr::target_field` · datafusion-physical-expr 55.1.0

```rust
fn target_field(&self) -> &FieldRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::try_cast::TryCastExpr", "path": "TryCastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [139, 2], "filename": "src/expressions/try_cast.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/try_cast.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

The target field this cast was constructed with.

For a type-only cast this is a field synthesized from the target data
type alone; only its data type is meaningful. TRY_CAST results are
always nullable regardless of the target field's nullability.

<a id="op-7fb21b4896064f26e229f5ae"></a>
## target_metadata

`function` · `datafusion_physical_expr::expressions::try_cast::TryCastExpr::target_metadata` · datafusion-physical-expr 55.1.0

```rust
fn target_metadata(&self) -> Option<&HashMap<String, String>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::try_cast::TryCastExpr", "path": "TryCastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [139, 2], "filename": "src/expressions/try_cast.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/try_cast.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Explicit metadata for the output field, or `None` to pass through source metadata.

<a id="op-27a833dde48f136d956c827d"></a>
## try_from_proto

`function` · `datafusion_physical_expr::expressions::try_cast::TryCastExpr::try_from_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalExprNode, ctx: &datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::try_cast::TryCastExpr", "path": "TryCastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [240, 1], "end": [269, 2], "filename": "src/expressions/try_cast.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/try_cast.rs:242`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Reconstruct a [`TryCastExpr`](../operations/datafusion_physical_expr.expressions.try_cast.TryCastExpr.md#op-cea674f9bf9c028f5652a987) from its protobuf representation.

<a id="op-7b84c827583d1b65d8847b81"></a>
## try_to_proto

`function` · `datafusion_physical_expr::expressions::try_cast::TryCastExpr::try_to_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_to_proto(&self, ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::try_cast::TryCastExpr", "path": "TryCastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 1], "end": [237, 2], "filename": "src/expressions/try_cast.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/try_cast.rs:221`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-808aef8a75e5f1750e4d7a59"></a>
## with_new_children

`function` · `datafusion_physical_expr::expressions::try_cast::TryCastExpr::with_new_children` · datafusion-physical-expr 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::try_cast::TryCastExpr", "path": "TryCastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 1], "end": [237, 2], "filename": "src/expressions/try_cast.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/try_cast.rs:203`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
