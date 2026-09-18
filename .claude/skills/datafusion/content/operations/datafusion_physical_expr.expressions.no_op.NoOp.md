# `datafusion_physical_expr::expressions::no_op::NoOp`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.expressions.no_op.NoOp.json).

<a id="op-b326da3d6bc40fc2326e7b0e"></a>
## NoOp

`struct` · `datafusion_physical_expr::expressions::no_op::NoOp` · datafusion-physical-expr 55.1.0

```rust
struct NoOp
```

Source: `src/expressions/no_op.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

A place holder expression, can not be evaluated.

Used in some cases where an `Arc<dyn PhysicalExpr>` is needed, such as `children()`

<a id="op-9eeaab2949c23c568417fff5"></a>
## children

`function` · `datafusion_physical_expr::expressions::no_op::NoOp::children` · datafusion-physical-expr 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::no_op::NoOp", "path": "NoOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [77, 2], "filename": "src/expressions/no_op.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/no_op.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c94451a0d0f5c4a3bf09a1d"></a>
## data_type

`function` · `datafusion_physical_expr::expressions::no_op::NoOp::data_type` · datafusion-physical-expr 55.1.0

```rust
fn data_type(&self, _input_schema: &Schema) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::no_op::NoOp", "path": "NoOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [77, 2], "filename": "src/expressions/no_op.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/no_op.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-776b0e169f22bc58902c6c65"></a>
## default

`function` · `datafusion_physical_expr::expressions::no_op::NoOp::default` · datafusion-physical-expr 55.1.0

```rust
fn default() -> NoOp
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::no_op::NoOp", "path": "NoOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 32], "end": [34, 39], "filename": "src/expressions/no_op.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/expressions/no_op.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6115840486e030e103ac06c"></a>
## eq

`function` · `datafusion_physical_expr::expressions::no_op::NoOp::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, other: &NoOp) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::no_op::NoOp", "path": "NoOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 17], "end": [34, 26], "filename": "src/expressions/no_op.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expressions/no_op.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79160722919738a8caa855a0"></a>
## evaluate

`function` · `datafusion_physical_expr::expressions::no_op::NoOp::evaluate` · datafusion-physical-expr 55.1.0

```rust
fn evaluate(&self, _batch: &RecordBatch) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::no_op::NoOp", "path": "NoOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [77, 2], "filename": "src/expressions/no_op.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/no_op.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cdb50fb13e8fc86565cb5eb4"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::no_op::NoOp::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::no_op::NoOp", "path": "NoOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 10], "end": [34, 15], "filename": "src/expressions/no_op.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expressions/no_op.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2dba8d80a60bf914e4b3403"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::no_op::NoOp::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::no_op::NoOp", "path": "NoOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [48, 2], "filename": "src/expressions/no_op.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/expressions/no_op.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-243ae3c04570d8da347e6778"></a>
## fmt_sql

`function` · `datafusion_physical_expr::expressions::no_op::NoOp::fmt_sql` · datafusion-physical-expr 55.1.0

```rust
fn fmt_sql(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::no_op::NoOp", "path": "NoOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [77, 2], "filename": "src/expressions/no_op.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/no_op.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2cc2fee7343e0b96cabcda2e"></a>
## hash

`function` · `datafusion_physical_expr::expressions::no_op::NoOp::hash` · datafusion-physical-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::no_op::NoOp", "path": "NoOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 41], "end": [34, 45], "filename": "src/expressions/no_op.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expressions/no_op.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb91886baa5bbc90575f5cad"></a>
## new

`function` · `datafusion_physical_expr::expressions::no_op::NoOp::new` · datafusion-physical-expr 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::no_op::NoOp", "path": "NoOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [42, 2], "filename": "src/expressions/no_op.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/no_op.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a NoOp expression

<a id="op-bddc6c141d4fb8fc268cd276"></a>
## nullable

`function` · `datafusion_physical_expr::expressions::no_op::NoOp::nullable` · datafusion-physical-expr 55.1.0

```rust
fn nullable(&self, _input_schema: &Schema) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::no_op::NoOp", "path": "NoOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [77, 2], "filename": "src/expressions/no_op.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/no_op.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-310a390fc4e7b7eba7964f64"></a>
## with_new_children

`function` · `datafusion_physical_expr::expressions::no_op::NoOp::with_new_children` · datafusion-physical-expr 55.1.0

```rust
fn with_new_children(Arc<self>, _children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::no_op::NoOp", "path": "NoOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [77, 2], "filename": "src/expressions/no_op.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/no_op.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
