# `datafusion_functions_aggregate_common::accumulator`

Crate `datafusion-functions-aggregate-common` · 3 public items · structured records in [`model/datafusion_functions_aggregate_common.accumulator.json`](../model/datafusion_functions_aggregate_common.accumulator.json)

## AccumulatorArgs

`struct` · `datafusion_functions_aggregate_common::accumulator::AccumulatorArgs`

Also reachable as `datafusion_expr::function::AccumulatorArgs`

```rust
struct AccumulatorArgs<'a>
```

**Fields**: `return_field`, `schema`, `ignore_nulls`, `order_bys`, `is_reversed`, `name`, `is_distinct`, `exprs`, `expr_fields`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn return_type(&self) -> &DataType
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.accumulator.AccumulatorArgs.md).


[`AccumulatorArgs`] contains information about how an aggregate
function was called, including the types of its arguments and any optional
ordering expressions.

---

## StateFieldsArgs

`struct` · `datafusion_functions_aggregate_common::accumulator::StateFieldsArgs`

Also reachable as `datafusion_expr::function::StateFieldsArgs`

```rust
struct StateFieldsArgs<'a>
```

**Fields**: `name`, `input_fields`, `return_field`, `ordering_fields`, `is_distinct`

**Methods** (1)

```rust
fn return_type(&self) -> &DataType
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.accumulator.StateFieldsArgs.md).


[`StateFieldsArgs`] contains information about the fields that an
aggregate function's accumulator should have. Used for `AggregateUDFImpl::state_fields`.

---

## AccumulatorFactoryFunction

`type_alias` · `datafusion_functions_aggregate_common::accumulator::AccumulatorFactoryFunction`

Also reachable as `datafusion::logical_expr::AccumulatorFactoryFunction`, `datafusion_expr::AccumulatorFactoryFunction`, `datafusion_expr::function::AccumulatorFactoryFunction`

```rust
type AccumulatorFactoryFunction = std::sync::Arc<dyn Fn(AccumulatorArgs<'_>) -> datafusion_common::Result<Box<dyn Accumulator>> + Send + Sync>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.accumulator.AccumulatorFactoryFunction.md).


Factory that returns an accumulator for the given aggregate function.

---
