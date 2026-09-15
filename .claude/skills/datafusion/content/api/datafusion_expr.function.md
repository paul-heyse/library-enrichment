# `datafusion_expr::function`

Crate `datafusion-expr` · 7 public items · structured records in [`model/datafusion_expr.function.json`](../model/datafusion_expr.function.json)

## Hint

`enum` · `datafusion_expr::function::Hint`

```rust
enum Hint
```

**Variants**: `Pad`, `AcceptsSingular`

**Derives**: Clone, Copy, Debug

---

## AggregateFunctionSimplification

`type_alias` · `datafusion_expr::function::AggregateFunctionSimplification`

```rust
type AggregateFunctionSimplification = Box<dyn Fn(expr::AggregateFunction, &simplify::SimplifyContext) -> datafusion_common::Result<Expr>>
```

Type alias for [crate::udaf::AggregateUDFImpl::simplify].

This closure is invoked with:
* `aggregate_function`: [AggregateFunction] with already simplified arguments
* `info`: [SimplifyContext]

It returns a simplified [Expr] or an error.

---

## PartitionEvaluatorFactory

`type_alias` · `datafusion_expr::function::PartitionEvaluatorFactory`

Also reachable as `datafusion::logical_expr::PartitionEvaluatorFactory`, `datafusion_expr::PartitionEvaluatorFactory`

```rust
type PartitionEvaluatorFactory = std::sync::Arc<dyn Fn() -> datafusion_common::Result<Box<dyn PartitionEvaluator>> + Send + Sync>
```

Factory that creates a PartitionEvaluator for the given window
function

---

## ReturnTypeFunction

`type_alias` · `datafusion_expr::function::ReturnTypeFunction`

Also reachable as `datafusion::logical_expr::ReturnTypeFunction`, `datafusion_expr::ReturnTypeFunction`

```rust
type ReturnTypeFunction = std::sync::Arc<dyn Fn(&[arrow::datatypes::DataType]) -> datafusion_common::Result<std::sync::Arc<arrow::datatypes::DataType>> + Send + Sync>
```

Factory that returns the functions's return type given the input argument types

---

## ScalarFunctionImplementation

`type_alias` · `datafusion_expr::function::ScalarFunctionImplementation`

Also reachable as `datafusion::logical_expr::ScalarFunctionImplementation`, `datafusion_expr::ScalarFunctionImplementation`

```rust
type ScalarFunctionImplementation = std::sync::Arc<dyn Fn(&[ColumnarValue]) -> datafusion_common::Result<ColumnarValue> + Send + Sync>
```

Scalar function

The Fn param is the wrapped function but be aware that the function will
be passed with the slice / vec of columnar values (either scalar or array)
with the exception of zero param function, where a singular element vec
will be passed. In that case the single element is a null array to indicate
the batch's row count (so that the generative zero-argument function can know
the result array size).

---

## StateTypeFunction

`type_alias` · `datafusion_expr::function::StateTypeFunction`

Also reachable as `datafusion::logical_expr::StateTypeFunction`, `datafusion_expr::StateTypeFunction`

```rust
type StateTypeFunction = std::sync::Arc<dyn Fn(&arrow::datatypes::DataType) -> datafusion_common::Result<std::sync::Arc<Vec<arrow::datatypes::DataType>>> + Send + Sync>
```

Factory that returns the types used by an aggregator to serialize
its state, given its return datatype.

---

## WindowFunctionSimplification

`type_alias` · `datafusion_expr::function::WindowFunctionSimplification`

```rust
type WindowFunctionSimplification = Box<dyn Fn(expr::WindowFunction, &simplify::SimplifyContext) -> datafusion_common::Result<Expr>>
```

Type alias for [crate::udwf::WindowUDFImpl::simplify].

This closure is invoked with:
* `window_function`: [WindowFunction] with already simplified arguments
* `info`: [SimplifyContext]

It returns a simplified [Expr] or an error.

---
