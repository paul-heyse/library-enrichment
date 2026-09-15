# `datafusion_functions_window::rank`

Crate `datafusion-functions-window` · 9 public items · structured records in [`model/datafusion_functions_window.rank.json`](../model/datafusion_functions_window.rank.json)

## RankType

`enum` · `datafusion_functions_window::rank::RankType`

```rust
enum RankType
```

**Variants**: `Basic`, `Dense`, `Percent`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## dense_rank

`function` · `datafusion_functions_window::rank::dense_rank`

Also reachable as `datafusion_functions_window::expr_fn::dense_rank`

```rust
fn dense_rank() -> datafusion_expr::Expr
```

Create a [`WindowFunction`](datafusion_expr::Expr::WindowFunction) expression for
`DenseRank` user-defined window function.

Returns rank of the current row without gaps. This function counts peer groups

---

## dense_rank_udwf

`function` · `datafusion_functions_window::rank::dense_rank_udwf`

```rust
fn dense_rank_udwf() -> std::sync::Arc<datafusion_expr::WindowUDF>
```

Returns a [`WindowUDF`](datafusion_expr::WindowUDF) for [`dense_rank`].

Returns rank of the current row without gaps. This function counts peer groups

---

## percent_rank

`function` · `datafusion_functions_window::rank::percent_rank`

Also reachable as `datafusion_functions_window::expr_fn::percent_rank`

```rust
fn percent_rank() -> datafusion_expr::Expr
```

Create a [`WindowFunction`](datafusion_expr::Expr::WindowFunction) expression for
`PercentRank` user-defined window function.

Returns the relative rank of the current row: (rank - 1) / (total rows - 1)

---

## percent_rank_udwf

`function` · `datafusion_functions_window::rank::percent_rank_udwf`

```rust
fn percent_rank_udwf() -> std::sync::Arc<datafusion_expr::WindowUDF>
```

Returns a [`WindowUDF`](datafusion_expr::WindowUDF) for [`percent_rank`].

Returns the relative rank of the current row: (rank - 1) / (total rows - 1)

---

## rank

`function` · `datafusion_functions_window::rank::rank`

Also reachable as `datafusion_functions_window::expr_fn::rank`

```rust
fn rank() -> datafusion_expr::Expr
```

Create a [`WindowFunction`](datafusion_expr::Expr::WindowFunction) expression for
`Rank` user-defined window function.

Returns rank of the current row with gaps. Same as `row_number` of its first peer

---

## rank_udwf

`function` · `datafusion_functions_window::rank::rank_udwf`

```rust
fn rank_udwf() -> std::sync::Arc<datafusion_expr::WindowUDF>
```

Returns a [`WindowUDF`](datafusion_expr::WindowUDF) for [`rank`].

Returns rank of the current row with gaps. Same as `row_number` of its first peer

---

## Rank

`struct` · `datafusion_functions_window::rank::Rank`

```rust
struct Rank
```

**Implements**: `datafusion_expr::udwf::WindowUDFImpl`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn basic() -> Self
fn dense_rank() -> Self
fn new(name: String, rank_type: RankType) -> Self
fn percent_rank() -> Self
```

**via `datafusion_expr::udwf::WindowUDFImpl`**

```rust
fn documentation(&self) -> Option<&Documentation>
fn field(&self, field_args: WindowUDFFieldArgs<'_>) -> Result<FieldRef>
fn limit_effect(&self, _args: &[Arc<dyn PhysicalExpr>]) -> LimitEffect
fn name(&self) -> &str
fn partition_evaluator(&self, _partition_evaluator_args: PartitionEvaluatorArgs<'_>) -> Result<Box<dyn PartitionEvaluator>>
fn signature(&self) -> &Signature
fn sort_options(&self) -> Option<SortOptions>
```

Rank calculates the rank in the window function with order by

---

## RankState

`struct` · `datafusion_functions_window::rank::RankState`

```rust
struct RankState
```

**Fields**: `last_rank_data`, `last_rank_boundary`, `current_group_count`, `n_rank`

**Derives**: Clone, Debug, Default

State for the RANK(rank) built-in window function.

---
