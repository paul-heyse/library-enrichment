# `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashTableLookupExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.hash_join.partitioned_hash_eval.HashTableLookupExpr.json).

<a id="op-5c906554f27a7ed5d604fb93"></a>
## HashTableLookupExpr

`struct` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashTableLookupExpr` · datafusion-physical-plan 55.1.0

```rust
struct HashTableLookupExpr
```

Source: `src/joins/hash_join/partitioned_hash_eval.rs:259`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Physical expression that checks join keys in a [`Map`](../operations/datafusion_physical_plan.joins.Map.md#op-019dac099651410387431f3a) (hash table or array map).

Returns a [`BooleanArray`](arrow::array::BooleanArray) indicating if join keys (from `on_columns`) exist in the map.

<a id="op-343e013d2f14cd43386fd860"></a>
## children

`function` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashTableLookupExpr::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashTableLookupExpr", "path": "HashTableLookupExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [347, 1], "end": [424, 2], "filename": "src/joins/hash_join/partitioned_hash_eval.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/joins/hash_join/partitioned_hash_eval.rs:348`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-624d714af9bd887825e6bade"></a>
## data_type

`function` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashTableLookupExpr::data_type` · datafusion-physical-plan 55.1.0

```rust
fn data_type(&self, _input_schema: &Schema) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashTableLookupExpr", "path": "HashTableLookupExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [347, 1], "end": [424, 2], "filename": "src/joins/hash_join/partitioned_hash_eval.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/joins/hash_join/partitioned_hash_eval.rs:364`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28b8327f19b4591df6d96ada"></a>
## eq

`function` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashTableLookupExpr::eq` · datafusion-physical-plan 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashTableLookupExpr", "path": "HashTableLookupExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [323, 1], "end": [337, 2], "filename": "src/joins/hash_join/partitioned_hash_eval.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/joins/hash_join/partitioned_hash_eval.rs:324`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9628abe749f0a5ec36278136"></a>
## evaluate

`function` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashTableLookupExpr::evaluate` · datafusion-physical-plan 55.1.0

```rust
fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashTableLookupExpr", "path": "HashTableLookupExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [347, 1], "end": [424, 2], "filename": "src/joins/hash_join/partitioned_hash_eval.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/joins/hash_join/partitioned_hash_eval.rs:372`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b045e8edbb0512688e46ea7b"></a>
## fmt

`function` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashTableLookupExpr::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashTableLookupExpr", "path": "HashTableLookupExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [294, 1], "end": [305, 2], "filename": "src/joins/hash_join/partitioned_hash_eval.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/joins/hash_join/partitioned_hash_eval.rs:295`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc9353c1ac0043207b636f3a"></a>
## fmt

`function` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashTableLookupExpr::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashTableLookupExpr", "path": "HashTableLookupExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [341, 1], "end": [345, 2], "filename": "src/joins/hash_join/partitioned_hash_eval.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/joins/hash_join/partitioned_hash_eval.rs:342`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b69fc53fd3f01cc919bd594b"></a>
## fmt_sql

`function` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashTableLookupExpr::fmt_sql` · datafusion-physical-plan 55.1.0

```rust
fn fmt_sql(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashTableLookupExpr", "path": "HashTableLookupExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [347, 1], "end": [424, 2], "filename": "src/joins/hash_join/partitioned_hash_eval.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/joins/hash_join/partitioned_hash_eval.rs:421`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3eb0727b4a5a8484a3be2ac4"></a>
## hash

`function` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashTableLookupExpr::hash` · datafusion-physical-plan 55.1.0

```rust
fn hash<H: std::hash::Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashTableLookupExpr", "path": "HashTableLookupExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [307, 1], "end": [321, 2], "filename": "src/joins/hash_join/partitioned_hash_eval.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/joins/hash_join/partitioned_hash_eval.rs:308`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6e8e3af7f4dcf393f63fb8d"></a>
## new

`function` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashTableLookupExpr::new` · datafusion-physical-plan 55.1.0

```rust
fn new(on_columns: Vec<PhysicalExprRef>, random_state: SeededRandomState, map: Arc<Map>, description: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashTableLookupExpr", "path": "HashTableLookupExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [293, 2], "filename": "src/joins/hash_join/partitioned_hash_eval.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/partitioned_hash_eval.rs:280`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new HashTableLookupExpr

# Arguments
* `on_columns` - Columns in the ON clause used to compute the join key
* `random_state` - SeededRandomState for hashing
* `map` - Map to check membership (hash table or array map)
* `description` - Description for debugging
# Note
This is public for internal testing purposes only and is not
guaranteed to be stable across versions.

<a id="op-8dcd8f10daaaa9ab6ac119ad"></a>
## nullable

`function` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashTableLookupExpr::nullable` · datafusion-physical-plan 55.1.0

```rust
fn nullable(&self, _input_schema: &Schema) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashTableLookupExpr", "path": "HashTableLookupExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [347, 1], "end": [424, 2], "filename": "src/joins/hash_join/partitioned_hash_eval.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/joins/hash_join/partitioned_hash_eval.rs:368`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95f9db68b0c1e864bd3ed69e"></a>
## try_to_proto

`function` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashTableLookupExpr::try_to_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_to_proto(&self, _ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashTableLookupExpr", "path": "HashTableLookupExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [347, 1], "end": [424, 2], "filename": "src/joins/hash_join/partitioned_hash_eval.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/joins/hash_join/partitioned_hash_eval.rs:390`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a748ffdf2af3535a276bf2b4"></a>
## with_new_children

`function` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashTableLookupExpr::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashTableLookupExpr", "path": "HashTableLookupExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [347, 1], "end": [424, 2], "filename": "src/joins/hash_join/partitioned_hash_eval.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/joins/hash_join/partitioned_hash_eval.rs:352`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
