# `datafusion_physical_expr::expressions::in_list::InListExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.expressions.in_list.InListExpr.json).

<a id="op-44801613aedf41888de4d10c"></a>
## InListExpr

`struct` · `datafusion_physical_expr::expressions::in_list::InListExpr` · datafusion-physical-expr 55.1.0

```rust
struct InListExpr
```

Source: `src/expressions/in_list.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

InList

<a id="op-36dd7094a0dc18f694fb299e"></a>
## children

`function` · `datafusion_physical_expr::expressions::in_list::InListExpr::children` · datafusion-physical-expr 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::in_list::InListExpr", "path": "InListExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [296, 1], "end": [492, 2], "filename": "src/expressions/in_list.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/in_list.rs:439`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf16ff13d3b1202d863dd72f"></a>
## data_type

`function` · `datafusion_physical_expr::expressions::in_list::InListExpr::data_type` · datafusion-physical-expr 55.1.0

```rust
fn data_type(&self, _input_schema: &Schema) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::in_list::InListExpr", "path": "InListExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [296, 1], "end": [492, 2], "filename": "src/expressions/in_list.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/in_list.rs:297`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c37792aaa2ce5908b189e2b"></a>
## eq

`function` · `datafusion_physical_expr::expressions::in_list::InListExpr::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::in_list::InListExpr", "path": "InListExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [494, 1], "end": [500, 2], "filename": "src/expressions/in_list.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expressions/in_list.rs:495`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-495f4cbdae059fb505b4d3df"></a>
## evaluate

`function` · `datafusion_physical_expr::expressions::in_list::InListExpr::evaluate` · datafusion-physical-expr 55.1.0

```rust
fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::in_list::InListExpr", "path": "InListExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [296, 1], "end": [492, 2], "filename": "src/expressions/in_list.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/in_list.rs:318`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f36878d6e5d9365641ae59bb"></a>
## expr

`function` · `datafusion_physical_expr::expressions::in_list::InListExpr::expr` · datafusion-physical-expr 55.1.0

```rust
fn expr(&self) -> &Arc<dyn PhysicalExpr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::in_list::InListExpr", "path": "InListExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 1], "end": [277, 2], "filename": "src/expressions/in_list.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/in_list.rs:162`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Input expression

<a id="op-02011129c8c1ea5b2f2d98c7"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::in_list::InListExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::in_list::InListExpr", "path": "InListExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [278, 1], "end": [294, 2], "filename": "src/expressions/in_list.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/expressions/in_list.rs:279`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb4099c72710ed39feaa24d1"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::in_list::InListExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::in_list::InListExpr", "path": "InListExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [65, 2], "filename": "src/expressions/in_list.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expressions/in_list.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3c65414b48e3a4e9bb53b8d"></a>
## fmt_sql

`function` · `datafusion_physical_expr::expressions::in_list::InListExpr::fmt_sql` · datafusion-physical-expr 55.1.0

```rust
fn fmt_sql(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::in_list::InListExpr", "path": "InListExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [296, 1], "end": [492, 2], "filename": "src/expressions/in_list.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/in_list.rs:458`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a2ee46a9df87f66cb980034c"></a>
## hash

`function` · `datafusion_physical_expr::expressions::in_list::InListExpr::hash` · datafusion-physical-expr 55.1.0

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::in_list::InListExpr", "path": "InListExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [504, 1], "end": [511, 2], "filename": "src/expressions/in_list.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expressions/in_list.rs:505`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f6ec71cabd285e4de729ff54"></a>
## is_empty

`function` · `datafusion_physical_expr::expressions::in_list::InListExpr::is_empty` · datafusion-physical-expr 55.1.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::in_list::InListExpr", "path": "InListExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 1], "end": [277, 2], "filename": "src/expressions/in_list.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/in_list.rs:171`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-96d73fc36647682557f1c86c"></a>
## len

`function` · `datafusion_physical_expr::expressions::in_list::InListExpr::len` · datafusion-physical-expr 55.1.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::in_list::InListExpr", "path": "InListExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 1], "end": [277, 2], "filename": "src/expressions/in_list.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/in_list.rs:175`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff7f876c73a793d2e3f6ff5d"></a>
## list

`function` · `datafusion_physical_expr::expressions::in_list::InListExpr::list` · datafusion-physical-expr 55.1.0

```rust
fn list(&self) -> &[Arc<dyn PhysicalExpr>]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::in_list::InListExpr", "path": "InListExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 1], "end": [277, 2], "filename": "src/expressions/in_list.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/in_list.rs:167`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

List to search in

<a id="op-2399f1567ad46360281e6574"></a>
## negated

`function` · `datafusion_physical_expr::expressions::in_list::InListExpr::negated` · datafusion-physical-expr 55.1.0

```rust
fn negated(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::in_list::InListExpr", "path": "InListExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 1], "end": [277, 2], "filename": "src/expressions/in_list.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/in_list.rs:180`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Is this negated e.g. NOT IN LIST

<a id="op-a4f0fa60d28e029b2a1807a5"></a>
## nullable

`function` · `datafusion_physical_expr::expressions::in_list::InListExpr::nullable` · datafusion-physical-expr 55.1.0

```rust
fn nullable(&self, input_schema: &Schema) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::in_list::InListExpr", "path": "InListExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [296, 1], "end": [492, 2], "filename": "src/expressions/in_list.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/in_list.rs:301`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bfccbddb02ba0c7a3dbfd794"></a>
## try_from_proto

`function` · `datafusion_physical_expr::expressions::in_list::InListExpr::try_from_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalExprNode, ctx: &datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::in_list::InListExpr", "path": "InListExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 1], "end": [277, 2], "filename": "src/expressions/in_list.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/in_list.rs:253`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9893e2db115c0f13c1f7cb2"></a>
## try_new

`function` · `datafusion_physical_expr::expressions::in_list::InListExpr::try_new` · datafusion-physical-expr 55.1.0

```rust
fn try_new(expr: Arc<dyn PhysicalExpr>, list: Vec<Arc<dyn PhysicalExpr>>, negated: bool, schema: &Schema) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::in_list::InListExpr", "path": "InListExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 1], "end": [277, 2], "filename": "src/expressions/in_list.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/in_list.rs:230`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a new InList expression, using a static filter when possible.

This validates data types and attempts to create a static filter for constant
list expressions. Uses specialized StaticFilter implementations for better
performance (e.g., Int32StaticFilter for Int32).

Returns an error if data types don't match. If the list contains non-constant
expressions, falls back to dynamic evaluation at runtime.

<a id="op-530925ab44c8b1ed3f1e7134"></a>
## try_new_from_array

`function` · `datafusion_physical_expr::expressions::in_list::InListExpr::try_new_from_array` · datafusion-physical-expr 55.1.0

```rust
fn try_new_from_array(expr: Arc<dyn PhysicalExpr>, array: ArrayRef, negated: bool, schema: &Schema) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::in_list::InListExpr", "path": "InListExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 1], "end": [277, 2], "filename": "src/expressions/in_list.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/in_list.rs:199`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a new InList expression directly from an array, bypassing expression evaluation.

This is more efficient than [`InListExpr::try_new`](../operations/datafusion_physical_expr.expressions.in_list.InListExpr.md#op-d9893e2db115c0f13c1f7cb2) when you already have the list
as an array, as it builds the static filter directly from the array instead of
reconstructing an intermediate array from literal expressions.

The `list` field is populated with literal expressions extracted from
the array, and the array is used to build a static filter for
efficient set membership evaluation.

The `array` may be dictionary-encoded — it will be flattened to its
value type such that specialized filters are used.

Returns an error if the expression's data type and the array's data type
are not logically equal. Null arrays are always accepted.

<a id="op-3968e48098bfde2e208f8d9f"></a>
## try_to_proto

`function` · `datafusion_physical_expr::expressions::in_list::InListExpr::try_to_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_to_proto(&self, ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::in_list::InListExpr", "path": "InListExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [296, 1], "end": [492, 2], "filename": "src/expressions/in_list.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/in_list.rs:475`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ada3ac59be870c0b8353718f"></a>
## with_new_children

`function` · `datafusion_physical_expr::expressions::in_list::InListExpr::with_new_children` · datafusion-physical-expr 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::in_list::InListExpr", "path": "InListExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [296, 1], "end": [492, 2], "filename": "src/expressions/in_list.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/in_list.rs:445`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
