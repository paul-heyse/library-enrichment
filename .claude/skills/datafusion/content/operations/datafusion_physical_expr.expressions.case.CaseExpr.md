# `datafusion_physical_expr::expressions::case::CaseExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.expressions.case.CaseExpr.json).

<a id="op-7104eccb9cc9970bca456849"></a>
## CaseExpr

`struct` · `datafusion_physical_expr::expressions::case::CaseExpr` · datafusion-physical-expr 55.1.0

```rust
struct CaseExpr
```

Source: `src/expressions/case.rs:274`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

The CASE expression is similar to a series of nested if/else and there are two forms that
can be used. The first form consists of a series of boolean "when" expressions with
corresponding "then" expressions, and an optional "else" expression.

CASE WHEN condition THEN result
     [WHEN ...]
     [ELSE result]
END

The second form uses a base expression and then a series of "when" clauses that match on a
literal value.

CASE expression
    WHEN value THEN result
    [WHEN ...]
    [ELSE result]
END

<a id="op-0f69d7a3dbc129c1d6636b28"></a>
## children

`function` · `datafusion_physical_expr::expressions::case::CaseExpr::children` · datafusion-physical-expr 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::case::CaseExpr", "path": "CaseExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1250, 1], "end": [1453, 2], "filename": "src/expressions/case.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/case.rs:1348`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2e3cdf81343a853da6968f0"></a>
## data_type

`function` · `datafusion_physical_expr::expressions::case::CaseExpr::data_type` · datafusion-physical-expr 55.1.0

```rust
fn data_type(&self, input_schema: &Schema) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::case::CaseExpr", "path": "CaseExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1250, 1], "end": [1453, 2], "filename": "src/expressions/case.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/case.rs:1251`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e227038af2115abd08f42ddc"></a>
## else_expr

`function` · `datafusion_physical_expr::expressions::case::CaseExpr::else_expr` · datafusion-physical-expr 55.1.0

```rust
fn else_expr(&self) -> Option<&Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::case::CaseExpr", "path": "CaseExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [650, 1], "end": [725, 2], "filename": "src/expressions/case.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/case.rs:722`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Optional "else" expression

<a id="op-8697143e693ca66fdfe052a7"></a>
## eq

`function` · `datafusion_physical_expr::expressions::case::CaseExpr::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::case::CaseExpr", "path": "CaseExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [290, 1], "end": [294, 2], "filename": "src/expressions/case.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expressions/case.rs:291`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35dd3637bd81d038c620da6b"></a>
## evaluate

`function` · `datafusion_physical_expr::expressions::case::CaseExpr::evaluate` · datafusion-physical-expr 55.1.0

```rust
fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::case::CaseExpr", "path": "CaseExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1250, 1], "end": [1453, 2], "filename": "src/expressions/case.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/case.rs:1324`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9eb93305e8f94b883dddce86"></a>
## expr

`function` · `datafusion_physical_expr::expressions::case::CaseExpr::expr` · datafusion-physical-expr 55.1.0

```rust
fn expr(&self) -> Option<&Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::case::CaseExpr", "path": "CaseExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [650, 1], "end": [725, 2], "filename": "src/expressions/case.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/case.rs:712`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Optional base expression that can be compared to literal values in the "when" expressions

<a id="op-062bd454e09519cea0eb5504"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::case::CaseExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::case::CaseExpr", "path": "CaseExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 10], "end": [273, 15], "filename": "src/expressions/case.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expressions/case.rs:273`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27a1301ff018a8b271d7468c"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::case::CaseExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::case::CaseExpr", "path": "CaseExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [298, 1], "end": [312, 2], "filename": "src/expressions/case.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/expressions/case.rs:299`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b6e3ddf335136c9156afcf3"></a>
## fmt_sql

`function` · `datafusion_physical_expr::expressions::case::CaseExpr::fmt_sql` · datafusion-physical-expr 55.1.0

```rust
fn fmt_sql(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::case::CaseExpr", "path": "CaseExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1250, 1], "end": [1453, 2], "filename": "src/expressions/case.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/case.rs:1397`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b43752de025b0d588b8e397e"></a>
## hash

`function` · `datafusion_physical_expr::expressions::case::CaseExpr::hash` · datafusion-physical-expr 55.1.0

```rust
fn hash<H: std::hash::Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::case::CaseExpr", "path": "CaseExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [284, 1], "end": [288, 2], "filename": "src/expressions/case.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expressions/case.rs:285`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-337d2f5116ee159b5e4bc73c"></a>
## nullable

`function` · `datafusion_physical_expr::expressions::case::CaseExpr::nullable` · datafusion-physical-expr 55.1.0

```rust
fn nullable(&self, input_schema: &Schema) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::case::CaseExpr", "path": "CaseExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1250, 1], "end": [1453, 2], "filename": "src/expressions/case.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/case.rs:1255`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f4e954db79d43f5058bcc61"></a>
## try_from_proto

`function` · `datafusion_physical_expr::expressions::case::CaseExpr::try_from_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalExprNode, ctx: &datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::case::CaseExpr", "path": "CaseExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1456, 1], "end": [1499, 2], "filename": "src/expressions/case.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/case.rs:1458`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Reconstruct a [`CaseExpr`](../operations/datafusion_physical_expr.expressions.case.CaseExpr.md#op-7104eccb9cc9970bca456849) from its protobuf representation.

<a id="op-82b1d77627b4b73c3c2a0dea"></a>
## try_new

`function` · `datafusion_physical_expr::expressions::case::CaseExpr::try_new` · datafusion-physical-expr 55.1.0

```rust
fn try_new(expr: Option<Arc<dyn PhysicalExpr>>, when_then_expr: Vec<(std::sync::Arc<dyn PhysicalExpr>, std::sync::Arc<dyn PhysicalExpr>)>, else_expr: Option<Arc<dyn PhysicalExpr>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::case::CaseExpr", "path": "CaseExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [650, 1], "end": [725, 2], "filename": "src/expressions/case.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/case.rs:652`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a new CASE WHEN expression

<a id="op-9964a9f05c4c87479e6e01f5"></a>
## try_to_proto

`function` · `datafusion_physical_expr::expressions::case::CaseExpr::try_to_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_to_proto(&self, ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::case::CaseExpr", "path": "CaseExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1250, 1], "end": [1453, 2], "filename": "src/expressions/case.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/case.rs:1421`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-690c200ab037d510ad8b865c"></a>
## when_then_expr

`function` · `datafusion_physical_expr::expressions::case::CaseExpr::when_then_expr` · datafusion-physical-expr 55.1.0

```rust
fn when_then_expr(&self) -> &[(std::sync::Arc<dyn PhysicalExpr>, std::sync::Arc<dyn PhysicalExpr>)]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::case::CaseExpr", "path": "CaseExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [650, 1], "end": [725, 2], "filename": "src/expressions/case.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/case.rs:717`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

One or more when/then expressions

<a id="op-44aab656e9999fb5d5f7d432"></a>
## with_new_children

`function` · `datafusion_physical_expr::expressions::case::CaseExpr::with_new_children` · datafusion-physical-expr 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::case::CaseExpr", "path": "CaseExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1250, 1], "end": [1453, 2], "filename": "src/expressions/case.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/case.rs:1365`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
