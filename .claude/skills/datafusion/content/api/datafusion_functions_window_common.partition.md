# `datafusion_functions_window_common::partition`

Crate `datafusion-functions-window-common` · 1 public items · structured records in [`model/datafusion_functions_window_common.partition.json`](../model/datafusion_functions_window_common.partition.json)

## PartitionEvaluatorArgs

`struct` · `datafusion_functions_window_common::partition::PartitionEvaluatorArgs`

Also reachable as `datafusion_expr::function::PartitionEvaluatorArgs`

```rust
struct PartitionEvaluatorArgs<'a>
```

**Derives**: Debug, Default

**Methods** (5)

```rust
fn ignore_nulls(&self) -> bool
fn input_exprs(&self) -> &'a [Arc<dyn PhysicalExpr>]
fn input_fields(&self) -> &'a [FieldRef]
fn is_reversed(&self) -> bool
fn new(input_exprs: &'a [Arc<dyn PhysicalExpr>], input_fields: &'a [FieldRef], is_reversed: bool, ignore_nulls: bool) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_window_common.partition.PartitionEvaluatorArgs.md).


Arguments passed to created user-defined window function state
during physical execution.

---
