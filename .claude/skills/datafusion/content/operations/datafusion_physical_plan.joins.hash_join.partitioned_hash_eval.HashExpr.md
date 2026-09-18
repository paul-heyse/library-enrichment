# `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.hash_join.partitioned_hash_eval.HashExpr.json).

<a id="op-5c950665f206d2dde90ca02c"></a>
## HashExpr

`struct` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr` · datafusion-physical-plan 55.1.0

```rust
struct HashExpr
```

Source: `src/joins/hash_join/partitioned_hash_eval.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Physical expression that computes hash values for a set of columns

This expression computes the hash of join key columns using a specific RandomState.
It returns a UInt64Array containing the hash values.

This is used for:
- Computing routing hashes (with RepartitionExec's 0,0,0,0 seeds)
- Computing lookup hashes (with HashJoin's 'J','O','I','N' seeds)

<a id="op-16c8d9bebea67ec8f4c812a3"></a>
## children

`function` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr", "path": "HashExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [158, 1], "end": [223, 2], "filename": "src/joins/hash_join/partitioned_hash_eval.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/joins/hash_join/partitioned_hash_eval.rs:159`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29cb3d29d6235bf408bbbba3"></a>
## data_type

`function` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr::data_type` · datafusion-physical-plan 55.1.0

```rust
fn data_type(&self, _input_schema: &Schema) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr", "path": "HashExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [158, 1], "end": [223, 2], "filename": "src/joins/hash_join/partitioned_hash_eval.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/joins/hash_join/partitioned_hash_eval.rs:174`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ac87cab590cc168cc6dab0a"></a>
## description

`function` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr::description` · datafusion-physical-plan 55.1.0

```rust
fn description(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr", "path": "HashExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [119, 2], "filename": "src/joins/hash_join/partitioned_hash_eval.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/partitioned_hash_eval.rs:116`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Get the description.

<a id="op-cda96985fd47e25422a53880"></a>
## eq

`function` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr::eq` · datafusion-physical-plan 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr", "path": "HashExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [142, 1], "end": [148, 2], "filename": "src/joins/hash_join/partitioned_hash_eval.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/joins/hash_join/partitioned_hash_eval.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10243094618885cbe0f3c865"></a>
## evaluate

`function` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr::evaluate` · datafusion-physical-plan 55.1.0

```rust
fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr", "path": "HashExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [158, 1], "end": [223, 2], "filename": "src/joins/hash_join/partitioned_hash_eval.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/joins/hash_join/partitioned_hash_eval.rs:182`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36bc64526aae62ea49418f4f"></a>
## fmt

`function` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr", "path": "HashExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 1], "end": [156, 2], "filename": "src/joins/hash_join/partitioned_hash_eval.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/joins/hash_join/partitioned_hash_eval.rs:153`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f30ca174af93a13431c56b8b"></a>
## fmt

`function` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr", "path": "HashExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [132, 2], "filename": "src/joins/hash_join/partitioned_hash_eval.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/joins/hash_join/partitioned_hash_eval.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c588fdec07eabc76effca6af"></a>
## fmt_sql

`function` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr::fmt_sql` · datafusion-physical-plan 55.1.0

```rust
fn fmt_sql(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr", "path": "HashExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [158, 1], "end": [223, 2], "filename": "src/joins/hash_join/partitioned_hash_eval.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/joins/hash_join/partitioned_hash_eval.rs:201`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c155c73b4d7843a5aec654e8"></a>
## hash

`function` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr::hash` · datafusion-physical-plan 55.1.0

```rust
fn hash<H: std::hash::Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr", "path": "HashExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [140, 2], "filename": "src/joins/hash_join/partitioned_hash_eval.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/joins/hash_join/partitioned_hash_eval.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71fb01f5b97405e62baa48b4"></a>
## new

`function` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr::new` · datafusion-physical-plan 55.1.0

```rust
fn new(on_columns: Vec<PhysicalExprRef>, random_state: SeededRandomState, description: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr", "path": "HashExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [119, 2], "filename": "src/joins/hash_join/partitioned_hash_eval.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/partitioned_hash_eval.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new HashExpr

# Arguments
* `on_columns` - Columns to hash
* `random_state` - SeededRandomState for hashing
* `description` - Description for debugging (e.g., "hash_repartition", "hash_join")

<a id="op-f35dc4773a9a0ee8ef2af1cd"></a>
## nullable

`function` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr::nullable` · datafusion-physical-plan 55.1.0

```rust
fn nullable(&self, _input_schema: &Schema) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr", "path": "HashExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [158, 1], "end": [223, 2], "filename": "src/joins/hash_join/partitioned_hash_eval.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/joins/hash_join/partitioned_hash_eval.rs:178`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c8153d1878dfca80d1b9fdb"></a>
## on_columns

`function` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr::on_columns` · datafusion-physical-plan 55.1.0

```rust
fn on_columns(&self) -> &[PhysicalExprRef]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr", "path": "HashExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [119, 2], "filename": "src/joins/hash_join/partitioned_hash_eval.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/partitioned_hash_eval.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Get the columns being hashed.

<a id="op-2e7dd4444d5ef5abaeca9cd8"></a>
## seed

`function` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr::seed` · datafusion-physical-plan 55.1.0

```rust
fn seed(&self) -> u64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr", "path": "HashExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [119, 2], "filename": "src/joins/hash_join/partitioned_hash_eval.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/partitioned_hash_eval.rs:111`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Get the seed used for hashing.

<a id="op-e0c57de56807f26a185e4192"></a>
## try_from_proto

`function` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr::try_from_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalExprNode, ctx: &datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr", "path": "HashExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [226, 1], "end": [253, 2], "filename": "src/joins/hash_join/partitioned_hash_eval.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/partitioned_hash_eval.rs:237`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Reconstruct a [`HashExpr`](../operations/datafusion_physical_plan.joins.hash_join.partitioned_hash_eval.HashExpr.md#op-5c950665f206d2dde90ca02c) from its protobuf representation.

Takes the whole [`PhysicalExprNode`], the exact inverse of what
[`PhysicalExpr::try_to_proto`] produces, so every expression's
`try_from_proto` shares one signature. Child sub-expressions are
decoded recursively via [`PhysicalExprDecodeCtx::decode`].

[`PhysicalExprNode`]: datafusion_proto_models::protobuf::PhysicalExprNode
[`PhysicalExpr::try_to_proto`]: datafusion_physical_expr_common::physical_expr::PhysicalExpr::try_to_proto
[`PhysicalExprDecodeCtx::decode`]: datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx::decode

Unresolved upstream links (retained, not inferred): `datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx::decode`.

<a id="op-82e5cf3f65a63b6d00824359"></a>
## try_to_proto

`function` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr::try_to_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_to_proto(&self, ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr", "path": "HashExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [158, 1], "end": [223, 2], "filename": "src/joins/hash_join/partitioned_hash_eval.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/joins/hash_join/partitioned_hash_eval.rs:206`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6dfe00bfa99f4bc86b04485b"></a>
## with_new_children

`function` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr", "path": "HashExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [158, 1], "end": [223, 2], "filename": "src/joins/hash_join/partitioned_hash_eval.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/joins/hash_join/partitioned_hash_eval.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
