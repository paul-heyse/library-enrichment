# `datafusion_expr::udaf`

Crate `datafusion-expr` · 11 public items · structured records in [`model/datafusion_expr.udaf.json`](../model/datafusion_expr.udaf.json)

## ReversedUDAF

`enum` · `datafusion_expr::udaf::ReversedUDAF`

Also reachable as `datafusion::logical_expr::ReversedUDAF`, `datafusion_expr::ReversedUDAF`

```rust
enum ReversedUDAF
```

**Variants**: `Identical`, `NotSupported`, `Reversed`

---

## SetMonotonicity

`enum` · `datafusion_expr::udaf::SetMonotonicity`

Also reachable as `datafusion::logical_expr::SetMonotonicity`, `datafusion_expr::SetMonotonicity`

```rust
enum SetMonotonicity
```

**Variants**: `Increasing`, `Decreasing`, `NotMonotonic`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

Indicates whether an aggregation function is monotonic as a set
function. A set function is monotonically increasing if its value
increases as its argument grows (as a set). Formally, `f` is a
monotonically increasing set function if `f(S) >= f(T)` whenever `S`
is a superset of `T`.

For example `COUNT` and `MAX` are monotonically increasing as their
values always increase (or stay the same) as new values are seen. On
the other hand, `MIN` is monotonically decreasing as its value always
decreases or stays the same as new values are seen.

---

## udaf_default_display_name

`function` · `datafusion_expr::udaf::udaf_default_display_name`

Also reachable as `datafusion::logical_expr::udaf_default_display_name`, `datafusion_expr::udaf_default_display_name`

```rust
fn udaf_default_display_name<F: AggregateUDFImpl + ?Sized>(func: &F, params: &expr::AggregateFunctionParams) -> datafusion_common::Result<String>
```

Encapsulates default implementation of [`AggregateUDFImpl::display_name`].

---

## udaf_default_human_display

`function` · `datafusion_expr::udaf::udaf_default_human_display`

Also reachable as `datafusion::logical_expr::udaf_default_human_display`, `datafusion_expr::udaf_default_human_display`

```rust
fn udaf_default_human_display<F: AggregateUDFImpl + ?Sized>(func: &F, params: &expr::AggregateFunctionParams) -> datafusion_common::Result<String>
```

Encapsulates default implementation of [`AggregateUDFImpl::human_display`].

---

## udaf_default_return_field

`function` · `datafusion_expr::udaf::udaf_default_return_field`

Also reachable as `datafusion::logical_expr::udaf_default_return_field`, `datafusion_expr::udaf_default_return_field`

```rust
fn udaf_default_return_field<F: AggregateUDFImpl + ?Sized>(func: &F, arg_fields: &[arrow::datatypes::FieldRef]) -> datafusion_common::Result<arrow::datatypes::FieldRef>
```

Encapsulates default implementation of [`AggregateUDFImpl::return_field`].

---

## udaf_default_schema_name

`function` · `datafusion_expr::udaf::udaf_default_schema_name`

Also reachable as `datafusion::logical_expr::udaf_default_schema_name`, `datafusion_expr::udaf_default_schema_name`

```rust
fn udaf_default_schema_name<F: AggregateUDFImpl + ?Sized>(func: &F, params: &expr::AggregateFunctionParams) -> datafusion_common::Result<String>
```

Encapsulates default implementation of [`AggregateUDFImpl::schema_name`].

---

## udaf_default_window_function_display_name

`function` · `datafusion_expr::udaf::udaf_default_window_function_display_name`

Also reachable as `datafusion::logical_expr::udaf_default_window_function_display_name`, `datafusion_expr::udaf_default_window_function_display_name`

```rust
fn udaf_default_window_function_display_name<F: AggregateUDFImpl + ?Sized>(func: &F, params: &expr::WindowFunctionParams) -> datafusion_common::Result<String>
```

Encapsulates default implementation of [`AggregateUDFImpl::window_function_display_name`].

---

## udaf_default_window_function_schema_name

`function` · `datafusion_expr::udaf::udaf_default_window_function_schema_name`

Also reachable as `datafusion::logical_expr::udaf_default_window_function_schema_name`, `datafusion_expr::udaf_default_window_function_schema_name`

```rust
fn udaf_default_window_function_schema_name<F: AggregateUDFImpl + ?Sized>(func: &F, params: &expr::WindowFunctionParams) -> datafusion_common::Result<String>
```

Encapsulates default implementation of [`AggregateUDFImpl::window_function_schema_name`].

---

## AggregateUDF

`struct` · `datafusion_expr::udaf::AggregateUDF`

Also reachable as `datafusion::logical_expr::AggregateUDF`, `datafusion_expr::AggregateUDF`

```rust
struct AggregateUDF
```

**Implements**: `core::convert::From`, `core::fmt::Display`, `datafusion_expr::type_coercion::functions::UDFCoercionExt`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd

**Methods** (33)

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
fn aliases(&self) -> &[String]
fn call(&self, args: Vec<Expr>) -> Expr
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
fn create_groups_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
fn create_sliding_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
fn default_value(&self, data_type: &DataType) -> Result<ScalarValue>
fn display_name(&self, params: &AggregateFunctionParams) -> Result<String>
fn documentation(&self) -> Option<&Documentation>
fn groups_accumulator_supported(&self, args: AccumulatorArgs<'_>) -> bool
fn human_display(&self, params: &AggregateFunctionParams) -> Result<String>
fn inner(&self) -> &Arc<dyn AggregateUDFImpl>
fn is_descending(&self) -> Option<bool>
fn is_nullable(&self) -> bool
fn name(&self) -> &str
fn new_from_impl<F>(fun: F) -> AggregateUDF where F: AggregateUDFImpl + 'static
fn new_from_shared_impl(fun: Arc<dyn AggregateUDFImpl>) -> AggregateUDF
fn order_sensitivity(&self) -> AggregateOrderSensitivity
fn return_field(&self, args: &[FieldRef]) -> Result<FieldRef>
fn return_type(&self, args: &[DataType]) -> Result<DataType>
fn reverse_udf(&self) -> ReversedUDAF
fn schema_name(&self, params: &AggregateFunctionParams) -> Result<String>
fn signature(&self) -> &Signature
fn simplify(&self) -> Option<AggregateFunctionSimplification>
fn simplify_expr_op_literal(&self, agg_function: &AggregateFunction, arg: &Expr, op: Operator, lit: &Expr, arg_is_left: bool) -> Result<Option<Expr>>
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
fn supports_null_handling_clause(&self) -> bool
fn supports_within_group_clause(&self) -> bool
fn value_from_stats(&self, statistics_args: &StatisticsArgs<'_>) -> Option<ScalarValue>
fn window_function_display_name(&self, params: &WindowFunctionParams) -> Result<String>
fn window_function_schema_name(&self, params: &WindowFunctionParams) -> Result<String>
fn with_aliases(self, aliases: impl IntoIterator<Item = &'static str>) -> Self
fn with_beneficial_ordering(self, beneficial_ordering: bool) -> Result<Option<AggregateUDF>>
```

**via `core::convert::From`**

```rust
fn from(fun: F) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

**via `datafusion_expr::type_coercion::functions::UDFCoercionExt`**

```rust
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
fn name(&self) -> &str
fn signature(&self) -> &Signature
```

Logical representation of a user-defined [aggregate function] (UDAF).

An aggregate function combines the values from multiple input rows
into a single output "aggregate" (summary) row. It is different
from a scalar function because it is stateful across batches. User
defined aggregate functions can be used as normal SQL aggregate
functions (`GROUP BY` clause) as well as window functions (`OVER`
clause).

`AggregateUDF` provides DataFusion the information needed to plan and call
aggregate functions, including name, type information, and a factory
function to create an [`Accumulator`] instance, to perform the actual
aggregation.

For more information, please see [the examples]:

1. For simple use cases, use [`create_udaf`] (examples in [`simple_udaf.rs`]).

2. For advanced use cases, use [`AggregateUDFImpl`] which provides full API
   access (examples in [`advanced_udaf.rs`]).

# API Note
This is a separate struct from `AggregateUDFImpl` to maintain backwards
compatibility with the older API.

[the examples]: https://github.com/apache/datafusion/tree/main/datafusion-examples#single-process
[aggregate function]: https://en.wikipedia.org/wiki/Aggregate_function
[`Accumulator`]: Accumulator
[`create_udaf`]: crate::expr_fn::create_udaf
[`simple_udaf.rs`]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/udf/simple_udaf.rs
[`advanced_udaf.rs`]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/udf/advanced_udaf.rs

---

## StatisticsArgs

`struct` · `datafusion_expr::udaf::StatisticsArgs`

Also reachable as `datafusion::logical_expr::StatisticsArgs`, `datafusion_expr::StatisticsArgs`, `datafusion_physical_plan::udaf::StatisticsArgs`

```rust
struct StatisticsArgs<'a>
```

**Fields**: `statistics`, `return_type`, `is_distinct`, `exprs`

**Derives**: Debug

Arguments passed to [`AggregateUDFImpl::value_from_stats`]

---

## AggregateUDFImpl

`trait` · `datafusion_expr::udaf::AggregateUDFImpl`

Also reachable as `datafusion::logical_expr::AggregateUDFImpl`, `datafusion_expr::AggregateUDFImpl`

```rust
trait AggregateUDFImpl: Debug + DynEq + DynHash + Send + Sync + Any
```

**Implementors** (39)

- `datafusion_expr::expr_fn::SimpleAggregateUDF`
- `datafusion_expr::test::function_stub::Avg`
- `datafusion_expr::test::function_stub::Count`
- `datafusion_expr::test::function_stub::Max`
- `datafusion_expr::test::function_stub::Min`
- `datafusion_expr::test::function_stub::Sum`
- `datafusion_ffi::udaf::ForeignAggregateUDF`
- `datafusion_functions_aggregate::any_value::AnyValue`
- `datafusion_functions_aggregate::approx_distinct::ApproxDistinct`
- `datafusion_functions_aggregate::approx_median::ApproxMedian`
- `datafusion_functions_aggregate::approx_percentile_cont::ApproxPercentileCont`
- `datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileContWithWeight`
- `datafusion_functions_aggregate::array_agg::ArrayAgg`
- `datafusion_functions_aggregate::average::Avg`
- `datafusion_functions_aggregate::bool_and_or::BoolAnd`
- `datafusion_functions_aggregate::bool_and_or::BoolOr`
- `datafusion_functions_aggregate::correlation::Correlation`
- `datafusion_functions_aggregate::count::Count`
- `datafusion_functions_aggregate::covariance::CovariancePopulation`
- `datafusion_functions_aggregate::covariance::CovarianceSample`
- `datafusion_functions_aggregate::first_last::FirstValue`
- `datafusion_functions_aggregate::first_last::LastValue`
- `datafusion_functions_aggregate::grouping::Grouping`
- `datafusion_functions_aggregate::median::Median`
- `datafusion_functions_aggregate::min_max::Max`
- `datafusion_functions_aggregate::min_max::Min`
- `datafusion_functions_aggregate::nth_value::NthValueAgg`
- `datafusion_functions_aggregate::percentile_cont::PercentileCont`
- `datafusion_functions_aggregate::regr::Regr`
- `datafusion_functions_aggregate::stddev::Stddev`
- `datafusion_functions_aggregate::stddev::StddevPop`
- `datafusion_functions_aggregate::string_agg::StringAgg`
- `datafusion_functions_aggregate::sum::Sum`
- `datafusion_functions_aggregate::variance::VariancePopulation`
- `datafusion_functions_aggregate::variance::VarianceSample`
- `datafusion_spark::function::aggregate::avg::SparkAvg`
- `datafusion_spark::function::aggregate::collect::SparkCollectList`
- `datafusion_spark::function::aggregate::collect::SparkCollectSet`
- `datafusion_spark::function::aggregate::try_sum::SparkTrySum`

**Methods** (29)

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
fn aliases(&self) -> &[String]
fn coerce_types(&self, _arg_types: &[DataType]) -> Result<Vec<DataType>>
fn create_groups_accumulator(&self, _args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
fn create_sliding_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
fn default_value(&self, data_type: &DataType) -> Result<ScalarValue>
fn display_name(&self, params: &AggregateFunctionParams) -> Result<String>
fn documentation(&self) -> Option<&Documentation>
fn groups_accumulator_supported(&self, _args: AccumulatorArgs<'_>) -> bool
fn human_display(&self, params: &AggregateFunctionParams) -> Result<String>
fn is_descending(&self) -> Option<bool>
fn is_nullable(&self) -> bool
fn name(&self) -> &str
fn order_sensitivity(&self) -> AggregateOrderSensitivity
fn return_field(&self, arg_fields: &[FieldRef]) -> Result<FieldRef>
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn reverse_expr(&self) -> ReversedUDAF
fn schema_name(&self, params: &AggregateFunctionParams) -> Result<String>
fn set_monotonicity(&self, _data_type: &DataType) -> SetMonotonicity
fn signature(&self) -> &Signature
fn simplify(&self) -> Option<AggregateFunctionSimplification>
fn simplify_expr_op_literal(&self, _agg_function: &AggregateFunction, _arg: &Expr, _op: Operator, _lit: &Expr, _arg_is_left: bool) -> Result<Option<Expr>>
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
fn supports_null_handling_clause(&self) -> bool
fn supports_within_group_clause(&self) -> bool
fn value_from_stats(&self, _statistics_args: &StatisticsArgs<'_>) -> Option<ScalarValue>
fn window_function_display_name(&self, params: &WindowFunctionParams) -> Result<String>
fn window_function_schema_name(&self, params: &WindowFunctionParams) -> Result<String>
fn with_beneficial_ordering(Arc<self>, _beneficial_ordering: bool) -> Result<Option<Arc<dyn AggregateUDFImpl>>>
```

Trait for implementing [`AggregateUDF`].

This trait exposes the full API for implementing user defined aggregate functions and
can be used to implement any function.

See [`advanced_udaf.rs`] for a full example with complete implementation and
[`AggregateUDF`] for other available options.

[`advanced_udaf.rs`]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/udf/advanced_udaf.rs

# Basic Example
```
# use std::any::Any;
# use std::sync::{Arc, LazyLock};
# use arrow::datatypes::{DataType, FieldRef};
# use datafusion_common::{DataFusionError, plan_err, Result};
# use datafusion_expr::{col, ColumnarValue, Signature, Volatility, Expr, Documentation};
# use datafusion_expr::{AggregateUDFImpl, AggregateUDF, Accumulator, function::{AccumulatorArgs, StateFieldsArgs}};
# use datafusion_expr::window_doc_sections::DOC_SECTION_AGGREGATE;
# use arrow::datatypes::Schema;
# use arrow::datatypes::Field;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GeoMeanUdf {
  signature: Signature,
}

impl GeoMeanUdf {
  fn new() -> Self {
    Self {
      signature: Signature::uniform(1, vec![DataType::Float64], Volatility::Immutable),
     }
  }
}

static DOCUMENTATION: LazyLock<Documentation> = LazyLock::new(|| {
        Documentation::builder(DOC_SECTION_AGGREGATE, "calculates a geometric mean", "geo_mean(2.0)")
            .with_argument("arg1", "The Float64 number for the geometric mean")
            .build()
    });

fn get_doc() -> &'static Documentation {
    &DOCUMENTATION
}

/// Implement the AggregateUDFImpl trait for GeoMeanUdf
impl AggregateUDFImpl for GeoMeanUdf {
   fn name(&self) -> &str { "geo_mean" }
   fn signature(&self) -> &Signature { &self.signature }
   fn return_type(&self, args: &[DataType]) -> Result<DataType> {
     if !matches!(args.get(0), Some(&DataType::Float64)) {
       return plan_err!("geo_mean only accepts Float64 arguments");
     }
     Ok(DataType::Float64)
   }
   // This is the accumulator factory; DataFusion uses it to create new accumulators.
   fn accumulator(&self, _acc_args: AccumulatorArgs) -> Result<Box<dyn Accumulator>> { unimplemented!() }
   fn state_fields(&self, args: StateFieldsArgs) -> Result<Vec<FieldRef>> {
       Ok(vec![
            Arc::new(args.return_field.as_ref().clone().with_name("value")),
            Arc::new(Field::new("ordering", DataType::UInt32, true))
       ])
   }
   fn documentation(&self) -> Option<&Documentation> {
       Some(get_doc())
   }
}

// Create a new AggregateUDF from the implementation
let geometric_mean = AggregateUDF::from(GeoMeanUdf::new());

// Call the function `geo_mean(col)`
let expr = geometric_mean.call(vec![col("a")]);
```

---
