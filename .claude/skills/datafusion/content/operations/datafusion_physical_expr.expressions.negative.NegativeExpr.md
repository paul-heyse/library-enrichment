# `datafusion_physical_expr::expressions::negative::NegativeExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.expressions.negative.NegativeExpr.json).

<a id="op-22e93e9f4f08b744a39b0c5d"></a>
## NegativeExpr

`struct` · `datafusion_physical_expr::expressions::negative::NegativeExpr` · datafusion-physical-expr 55.1.0

```rust
struct NegativeExpr
```

Source: `src/expressions/negative.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Negative expression

<a id="op-a8c3cf2add1058f39809cd75"></a>
## arg

`function` · `datafusion_physical_expr::expressions::negative::NegativeExpr::arg` · datafusion-physical-expr 55.1.0

```rust
fn arg(&self) -> &Arc<dyn PhysicalExpr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::negative::NegativeExpr", "path": "NegativeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [73, 2], "filename": "src/expressions/negative.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/negative.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Get the input expression

<a id="op-71bb752316f8f4aced38346a"></a>
## children

`function` · `datafusion_physical_expr::expressions::negative::NegativeExpr::children` · datafusion-physical-expr 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::negative::NegativeExpr", "path": "NegativeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [196, 2], "filename": "src/expressions/negative.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/negative.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-367fb7b6f3486635f9441e03"></a>
## data_type

`function` · `datafusion_physical_expr::expressions::negative::NegativeExpr::data_type` · datafusion-physical-expr 55.1.0

```rust
fn data_type(&self, input_schema: &Schema) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::negative::NegativeExpr", "path": "NegativeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [196, 2], "filename": "src/expressions/negative.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/negative.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6cbc29a2917fc68d88a4ec24"></a>
## eq

`function` · `datafusion_physical_expr::expressions::negative::NegativeExpr::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::negative::NegativeExpr", "path": "NegativeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [55, 2], "filename": "src/expressions/negative.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expressions/negative.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0b754717b12a36605de7863"></a>
## evaluate

`function` · `datafusion_physical_expr::expressions::negative::NegativeExpr::evaluate` · datafusion-physical-expr 55.1.0

```rust
fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::negative::NegativeExpr", "path": "NegativeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [196, 2], "filename": "src/expressions/negative.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/negative.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d8b5bf39bc6970a6511dec5"></a>
## evaluate_bounds

`function` · `datafusion_physical_expr::expressions::negative::NegativeExpr::evaluate_bounds` · datafusion-physical-expr 55.1.0

```rust
fn evaluate_bounds(&self, children: &[&Interval]) -> Result<Interval>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::negative::NegativeExpr", "path": "NegativeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [196, 2], "filename": "src/expressions/negative.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/negative.rs:120`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Given the child interval of a NegativeExpr, it calculates the NegativeExpr's interval.
It replaces the upper and lower bounds after multiplying them with -1.
Ex: `(a, b]` => `[-b, -a)`

<a id="op-d6cf12bc134ff629d4c0990f"></a>
## evaluate_statistics

`function` · `datafusion_physical_expr::expressions::negative::NegativeExpr::evaluate_statistics` · datafusion-physical-expr 55.1.0

```rust
fn evaluate_statistics(&self, children: &[&Distribution]) -> Result<Distribution>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::negative::NegativeExpr", "path": "NegativeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [196, 2], "filename": "src/expressions/negative.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/negative.rs:139`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d6823c0de543c132e79837f"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::negative::NegativeExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::negative::NegativeExpr", "path": "NegativeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 1], "end": [79, 2], "filename": "src/expressions/negative.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/expressions/negative.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f74f923fdf82fb2376da4a7"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::negative::NegativeExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::negative::NegativeExpr", "path": "NegativeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 10], "end": [44, 15], "filename": "src/expressions/negative.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expressions/negative.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f4eab6eeacb0eeefba317e4a"></a>
## fmt_sql

`function` · `datafusion_physical_expr::expressions::negative::NegativeExpr::fmt_sql` · datafusion-physical-expr 55.1.0

```rust
fn fmt_sql(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::negative::NegativeExpr", "path": "NegativeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [196, 2], "filename": "src/expressions/negative.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/negative.rs:174`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5ec9b12efa0ad9579f3af77"></a>
## get_properties

`function` · `datafusion_physical_expr::expressions::negative::NegativeExpr::get_properties` · datafusion-physical-expr 55.1.0

```rust
fn get_properties(&self, children: &[ExprProperties]) -> Result<ExprProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::negative::NegativeExpr", "path": "NegativeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [196, 2], "filename": "src/expressions/negative.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/negative.rs:164`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

The ordering of a [`NegativeExpr`](../operations/datafusion_physical_expr.expressions.negative.NegativeExpr.md#op-22e93e9f4f08b744a39b0c5d) is simply the reverse of its child.

<a id="op-f1df6791d20dc585208491fc"></a>
## hash

`function` · `datafusion_physical_expr::expressions::negative::NegativeExpr::hash` · datafusion-physical-expr 55.1.0

```rust
fn hash<H: std::hash::Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::negative::NegativeExpr", "path": "NegativeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [61, 2], "filename": "src/expressions/negative.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expressions/negative.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6797a0b9963b8ba2d12613c"></a>
## new

`function` · `datafusion_physical_expr::expressions::negative::NegativeExpr::new` · datafusion-physical-expr 55.1.0

```rust
fn new(arg: Arc<dyn PhysicalExpr>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::negative::NegativeExpr", "path": "NegativeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [73, 2], "filename": "src/expressions/negative.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/negative.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create new not expression

<a id="op-82b2386ed3423b9981961648"></a>
## nullable

`function` · `datafusion_physical_expr::expressions::negative::NegativeExpr::nullable` · datafusion-physical-expr 55.1.0

```rust
fn nullable(&self, input_schema: &Schema) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::negative::NegativeExpr", "path": "NegativeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [196, 2], "filename": "src/expressions/negative.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/negative.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a00c7848d68882ba1fe1d31f"></a>
## propagate_constraints

`function` · `datafusion_physical_expr::expressions::negative::NegativeExpr::propagate_constraints` · datafusion-physical-expr 55.1.0

```rust
fn propagate_constraints(&self, interval: &Interval, children: &[&Interval]) -> Result<Option<Vec<Interval>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::negative::NegativeExpr", "path": "NegativeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [196, 2], "filename": "src/expressions/negative.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/negative.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Returns a new [`Interval`](../operations/datafusion_expr_common.interval_arithmetic.Interval.md#op-0e8ed6898e67ba9321c8f53e) of a NegativeExpr  that has the existing `interval` given that
given the input interval is known to be `children`.

<a id="op-0dd984aab4e1d3f1530d3b34"></a>
## return_field

`function` · `datafusion_physical_expr::expressions::negative::NegativeExpr::return_field` · datafusion-physical-expr 55.1.0

```rust
fn return_field(&self, input_schema: &Schema) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::negative::NegativeExpr", "path": "NegativeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [196, 2], "filename": "src/expressions/negative.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/negative.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b2fd7a604f4fdf82829d8fa"></a>
## try_from_proto

`function` · `datafusion_physical_expr::expressions::negative::NegativeExpr::try_from_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalExprNode, ctx: &datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::negative::NegativeExpr", "path": "NegativeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [199, 1], "end": [218, 2], "filename": "src/expressions/negative.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/negative.rs:201`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Reconstruct a [`NegativeExpr`](../operations/datafusion_physical_expr.expressions.negative.NegativeExpr.md#op-22e93e9f4f08b744a39b0c5d) from its protobuf representation.

<a id="op-ffd23ffee5bffdbe181b2b7b"></a>
## try_to_proto

`function` · `datafusion_physical_expr::expressions::negative::NegativeExpr::try_to_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_to_proto(&self, ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::negative::NegativeExpr", "path": "NegativeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [196, 2], "filename": "src/expressions/negative.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/negative.rs:181`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf32cd57c8cf5f37313ceb77"></a>
## with_new_children

`function` · `datafusion_physical_expr::expressions::negative::NegativeExpr::with_new_children` · datafusion-physical-expr 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::negative::NegativeExpr", "path": "NegativeExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [196, 2], "filename": "src/expressions/negative.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/negative.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
