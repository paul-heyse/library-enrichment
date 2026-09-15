# `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval`

Crate `datafusion-physical-plan` · 3 public items · structured records in [`model/datafusion_physical_plan.joins.hash_join.partitioned_hash_eval.json`](../model/datafusion_physical_plan.joins.hash_join.partitioned_hash_eval.json)

## HashExpr

`struct` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashExpr`

Also reachable as `datafusion_physical_plan::joins::HashExpr`

```rust
struct HashExpr
```

**Implements**: `core::fmt::Display`, `datafusion_physical_expr_common::physical_expr::PhysicalExpr`

**Derives**: Debug, Eq, Hash, PartialEq

**Methods** (5)

```rust
fn description(&self) -> &str
fn new(on_columns: Vec<PhysicalExprRef>, random_state: SeededRandomState, description: String) -> Self
fn on_columns(&self) -> &[PhysicalExprRef]
fn seed(&self) -> u64
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalExprNode, ctx: &datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> Result<Arc<dyn PhysicalExpr>>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_physical_expr_common::physical_expr::PhysicalExpr`**

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
fn data_type(&self, _input_schema: &Schema) -> Result<DataType>
fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue>
fn fmt_sql(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
fn nullable(&self, _input_schema: &Schema) -> Result<bool>
fn try_to_proto(&self, ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

Physical expression that computes hash values for a set of columns

This expression computes the hash of join key columns using a specific RandomState.
It returns a UInt64Array containing the hash values.

This is used for:
- Computing routing hashes (with RepartitionExec's 0,0,0,0 seeds)
- Computing lookup hashes (with HashJoin's 'J','O','I','N' seeds)

---

## HashTableLookupExpr

`struct` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::HashTableLookupExpr`

Also reachable as `datafusion_physical_plan::joins::HashTableLookupExpr`

```rust
struct HashTableLookupExpr
```

**Implements**: `core::fmt::Display`, `datafusion_physical_expr_common::physical_expr::PhysicalExpr`

**Derives**: Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn new(on_columns: Vec<PhysicalExprRef>, random_state: SeededRandomState, map: Arc<Map>, description: String) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_physical_expr_common::physical_expr::PhysicalExpr`**

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
fn data_type(&self, _input_schema: &Schema) -> Result<DataType>
fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue>
fn fmt_sql(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
fn nullable(&self, _input_schema: &Schema) -> Result<bool>
fn try_to_proto(&self, _ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

Physical expression that checks join keys in a [`Map`] (hash table or array map).

Returns a [`BooleanArray`](arrow::array::BooleanArray) indicating if join keys (from `on_columns`) exist in the map.

---

## SeededRandomState

`struct` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::SeededRandomState`

Also reachable as `datafusion_physical_plan::joins::SeededRandomState`

```rust
struct SeededRandomState
```

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn random_state(&self) -> &RandomState
fn seed(&self) -> u64
const fn with_seed(k: u64) -> Self
```

RandomState wrapper that preserves the seed used to create it.

This is needed because `RandomState` doesn't expose its seed after creation,
but we need them for serialization (e.g., protobuf serde).

---
