# `datafusion_functions_aggregate::median`

Crate `datafusion-functions-aggregate` · 3 public items · structured records in [`model/datafusion_functions_aggregate.median.json`](../model/datafusion_functions_aggregate.median.json)

## median

`function` · `datafusion_functions_aggregate::median::median`

Also reachable as `datafusion_functions_aggregate::expr_fn::median`

```rust
fn median(expression: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Computes the median of a set of numbers

---

## median_udaf

`function` · `datafusion_functions_aggregate::median::median_udaf`

```rust
fn median_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`Median`]

---

## Median

`struct` · `datafusion_functions_aggregate::median::Median`

```rust
struct Median
```

**Implements**: `datafusion_expr::udaf::AggregateUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::udaf::AggregateUDFImpl`**

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
fn create_groups_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
fn documentation(&self) -> Option<&Documentation>
fn groups_accumulator_supported(&self, args: AccumulatorArgs<'_>) -> bool
fn name(&self) -> &str
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

MEDIAN aggregate expression. If using the non-distinct variation, then this uses a
lot of memory because all values need to be stored in memory before a result can be
computed. If an approximation is sufficient then APPROX_MEDIAN provides a much more
efficient solution.

If using the distinct variation, the memory usage will be similarly high if the
cardinality is high as it stores all distinct values in memory before computing the
result, but if cardinality is low then memory usage will also be lower.

---
