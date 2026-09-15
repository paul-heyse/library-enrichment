# PartitionEvaluator

`datafusion_expr::partition_evaluator::PartitionEvaluator`

```rust
trait PartitionEvaluator: Debug + Send + std::any::Any
```

Also reachable as `datafusion::logical_expr::PartitionEvaluator`, `datafusion_expr::PartitionEvaluator`

Prose: [`api/datafusion_expr.partition_evaluator.md`](../api/datafusion_expr.partition_evaluator.md#partitionevaluator) · records: [`model/datafusion_expr.partition_evaluator.json`](../model/datafusion_expr.partition_evaluator.json)

## Provided

Defaulted, and this is where the capability hides. The default is the conservative answer -- no pushdown, no statistics, no specialization -- so an implementation that overrides none of these works correctly and performs badly.

```rust
fn evaluate(&mut self, _values: &[ArrayRef], _range: &Range<usize>) -> Result<ScalarValue>
fn evaluate_all(&mut self, values: &[ArrayRef], num_rows: usize) -> Result<ArrayRef>
fn evaluate_all_with_rank(&self, _num_rows: usize, _ranks_in_partition: &[Range<usize>]) -> Result<ArrayRef>
fn get_range(&self, idx: usize, _n_rows: usize) -> Result<Range<usize>>
fn include_rank(&self) -> bool
fn is_causal(&self) -> bool
fn memoize(&mut self, _state: &mut WindowAggState) -> Result<()>
fn supports_bounded_execution(&self) -> bool
fn uses_window_frame(&self) -> bool
```

## Demonstrated by 2 upstream example(s)

- [`corpus/examples/udf/advanced_udwf.rs`](../corpus/examples/udf/advanced_udwf.rs)
- [`corpus/examples/udf/simple_udwf.rs`](../corpus/examples/udf/simple_udwf.rs)

## Documentation

Partition evaluator for Window Functions

# Background

An implementation of this trait is created and used for each
partition defined by an `OVER` clause and is instantiated by
the DataFusion runtime.

For example, evaluating `window_func(val) OVER (PARTITION BY col)`
on the following data:

```text
col | val
--- + ----
 A  | 10
 A  | 10
 C  | 20
 D  | 30
 D  | 30
```

Will instantiate three `PartitionEvaluator`s, one each for the
partitions defined by `col=A`, `col=B`, and `col=C`.

```text
col | val
--- + ----
 A  | 10     <--- partition 1
 A  | 10

col | val
--- + ----
 C  | 20     <--- partition 2

col | val
--- + ----
 D  | 30     <--- partition 3
 D  | 30
```

Different methods on this trait will be called depending on the
capabilities described by [`supports_bounded_execution`],
[`uses_window_frame`], and [`include_rank`],

When implementing a new `PartitionEvaluator`, implement
corresponding evaluator according to table below.

# Implementation Table

|[`uses_window_frame`]|[`supports_bounded_execution`]|[`include_rank`]|function_to_implement|
|---|---|----|----|
|false (default)      |false (default)               |false (default)   | [`evaluate_all`]           |
|false                |true                          |false             | [`evaluate`]               |
|false                |true/false                    |true              | [`evaluate_all_with_rank`] |
|true                 |true/false                    |true/false        | [`evaluate`]               |

[`evaluate`]: Self::evaluate
[`evaluate_all`]: Self::evaluate_all
[`evaluate_all_with_rank`]: Self::evaluate_all_with_rank
[`uses_window_frame`]: Self::uses_window_frame
[`include_rank`]: Self::include_rank
[`supports_bounded_execution`]: Self::supports_bounded_execution

For more background, please also see the [User defined Window Functions in DataFusion blog]

[User defined Window Functions in DataFusion blog]: https://datafusion.apache.org/blog/2025/04/19/user-defined-window-functions
