# `datafusion_expr_common::accumulator`

Crate `datafusion-expr-common` · 1 public items · structured records in [`model/datafusion_expr_common.accumulator.json`](../model/datafusion_expr_common.accumulator.json)

## Accumulator

`trait` · `datafusion_expr_common::accumulator::Accumulator`

Also reachable as `datafusion::logical_expr::Accumulator`, `datafusion::physical_plan::Accumulator`, `datafusion_expr::Accumulator`, `datafusion_physical_plan::Accumulator`, `datafusion_physical_plan::execution_plan::Accumulator`

```rust
trait Accumulator: Send + Sync + Debug + std::any::Any
```

**Implementors** (37)

- `datafusion_functions_aggregate::approx_percentile_cont::ApproxPercentileAccumulator`
- `datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileWithWeightAccumulator`
- `datafusion_functions_aggregate::array_agg::ArrayAggAccumulator`
- `datafusion_functions_aggregate::array_agg::DistinctArrayAggAccumulator`
- `datafusion_functions_aggregate::average::AvgAccumulator`
- `datafusion_functions_aggregate::correlation::CorrelationAccumulator`
- `datafusion_functions_aggregate::count::SlidingDistinctCountAccumulator`
- `datafusion_functions_aggregate::covariance::CovarianceAccumulator`
- `datafusion_functions_aggregate::first_last::FirstValueAccumulator`
- `datafusion_functions_aggregate::first_last::TrivialFirstValueAccumulator`
- `datafusion_functions_aggregate::first_last::TrivialLastValueAccumulator`
- `datafusion_functions_aggregate::min_max::SlidingMaxAccumulator`
- `datafusion_functions_aggregate::min_max::SlidingMinAccumulator`
- `datafusion_functions_aggregate::nth_value::NthValueAccumulator`
- `datafusion_functions_aggregate::nth_value::TrivialNthValueAccumulator`
- `datafusion_functions_aggregate::regr::RegrAccumulator`
- `datafusion_functions_aggregate::stddev::StddevAccumulator`
- `datafusion_functions_aggregate::sum::SlidingDistinctSumAccumulator`
- `datafusion_functions_aggregate::variance::DistinctVarianceAccumulator`
- `datafusion_functions_aggregate::variance::VarianceAccumulator`
- `datafusion_functions_aggregate_common::aggregate::avg_distinct::decimal::DecimalDistinctAvgAccumulator`
- `datafusion_functions_aggregate_common::aggregate::avg_distinct::numeric::Float64DistinctAvgAccumulator`
- `datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesDistinctCountAccumulator`
- `datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesViewDistinctCountAccumulator`
- `datafusion_functions_aggregate_common::aggregate::count_distinct::dict::DictionaryCountAccumulator`
- `datafusion_functions_aggregate_common::aggregate::count_distinct::native::Bitmap65536DistinctCountAccumulator`
- `datafusion_functions_aggregate_common::aggregate::count_distinct::native::Bitmap65536DistinctCountAccumulatorI16`
- `datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulator`
- `datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulatorI8`
- `datafusion_functions_aggregate_common::aggregate::count_distinct::native::BooleanDistinctCountAccumulator`
- `datafusion_functions_aggregate_common::aggregate::count_distinct::native::FloatDistinctCountAccumulator`
- `datafusion_functions_aggregate_common::aggregate::count_distinct::native::PrimitiveDistinctCountAccumulator`
- `datafusion_functions_aggregate_common::aggregate::sum_distinct::numeric::DistinctSumAccumulator`
- `datafusion_functions_aggregate_common::min_max::MaxAccumulator`
- `datafusion_functions_aggregate_common::min_max::MinAccumulator`
- `datafusion_functions_aggregate_common::noop_accumulator::NoopAccumulator`
- `datafusion_spark::function::aggregate::avg::AvgAccumulator`

**Methods** (7)

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
fn retract_batch(&mut self, _values: &[ArrayRef]) -> Result<()>
fn size(&self) -> usize
fn state(&mut self) -> Result<Vec<ScalarValue>>
fn supports_retract_batch(&self) -> bool
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Tracks an aggregate function's state.

`Accumulator`s are stateful objects that implement a single group. They
aggregate values from multiple rows together into a final output aggregate.

[`GroupsAccumulator]` is an additional more performant (but also complex) API
that manages state for multiple groups at once.

An accumulator knows how to:
* update its state from inputs via [`update_batch`]

* compute the final value from its internal state via [`evaluate`]

* retract an update to its state from given inputs via
  [`retract_batch`] (when used as a window aggregate [window
  function])

* convert its internal state to a vector of aggregate values via
  [`state`] and combine the state from multiple accumulators
  via [`merge_batch`], as part of efficient multi-phase grouping.

[`update_batch`]: Self::update_batch
[`retract_batch`]: Self::retract_batch
[`state`]: Self::state
[`evaluate`]: Self::evaluate
[`merge_batch`]: Self::merge_batch
[window function]: https://en.wikipedia.org/wiki/Window_function_(SQL)

---
