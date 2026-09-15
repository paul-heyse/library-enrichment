# AggregateUDFImpl

`datafusion_expr::udaf::AggregateUDFImpl`

```rust
trait AggregateUDFImpl: Debug + DynEq + DynHash + Send + Sync + Any
```

Also reachable as `datafusion::logical_expr::AggregateUDFImpl`, `datafusion_expr::AggregateUDFImpl`

Prose: [`api/datafusion_expr.udaf.md`](../api/datafusion_expr.udaf.md#aggregateudfimpl) · records: [`model/datafusion_expr.udaf.json`](../model/datafusion_expr.udaf.json)

## Required

Every implementation must supply these.

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
fn name(&self) -> &str
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

## Provided

Defaulted, and this is where the capability hides. The default is the conservative answer -- no pushdown, no statistics, no specialization -- so an implementation that overrides none of these works correctly and performs badly.

```rust
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
fn order_sensitivity(&self) -> AggregateOrderSensitivity
fn return_field(&self, arg_fields: &[FieldRef]) -> Result<FieldRef>
fn reverse_expr(&self) -> ReversedUDAF
fn schema_name(&self, params: &AggregateFunctionParams) -> Result<String>
fn set_monotonicity(&self, _data_type: &DataType) -> SetMonotonicity
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

## Implementors (39)

Read one before writing your own.

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

## Demonstrated by 1 upstream example(s)

- [`corpus/examples/udf/advanced_udaf.rs`](../corpus/examples/udf/advanced_udaf.rs)

## Documentation

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
