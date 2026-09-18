# `datafusion_expr::partition_evaluator::PartitionEvaluator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.partition_evaluator.PartitionEvaluator.json).

<a id="op-28021de2fba820ec392e9ef2"></a>
## PartitionEvaluator

`trait` · `datafusion_expr::partition_evaluator::PartitionEvaluator` · datafusion-expr 55.1.0

```rust
trait PartitionEvaluator: Debug + Send + std::any::Any
```

Source: `src/partition_evaluator.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

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

<a id="op-677473c31d382476473f452a"></a>
## evaluate

`function` · `datafusion_expr::partition_evaluator::PartitionEvaluator::evaluate` · datafusion-expr 55.1.0

```rust
fn evaluate(&mut self, _values: &[ArrayRef], _range: &Range<usize>) -> Result<ScalarValue>
```

Source: `src/partition_evaluator.rs:195`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Evaluate window function on a range of rows in an input
partition.

This is the simplest and most general function to implement
but also the least performant as it creates output one row at
a time. It is typically much faster to implement stateful
evaluation using one of the other specialized methods on this
trait.

Returns a [`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b) that is the value of the window
function within `range` for the entire partition. Argument
`values` contains the evaluation result of function arguments
and evaluation results of ORDER BY expressions. If function has a
single argument, `values[1..]` will contain ORDER BY expression results.

<a id="op-2b7c471ccdb273bd19e1a13a"></a>
## evaluate_all

`function` · `datafusion_expr::partition_evaluator::PartitionEvaluator::evaluate_all` · datafusion-expr 55.1.0

```rust
fn evaluate_all(&mut self, values: &[ArrayRef], num_rows: usize) -> Result<ArrayRef>
```

Source: `src/partition_evaluator.rs:166`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Evaluate a window function on an entire input partition.

This function is called once per input *partition* for window
functions that *do not use* values from the window frame,
such as `ROW_NUMBER`, `RANK`, `DENSE_RANK`, `PERCENT_RANK`,
`CUME_DIST`, `LEAD`, `LAG`).

It produces the result of all rows in a single pass. It
expects to receive the entire partition as the `value` and
must produce an output column with one output row for every
input row.

`num_rows` is required to correctly compute the output in case
`values.len() == 0`

Implementing this function is an optimization: certain window
functions are not affected by the window frame definition or
the query doesn't have a frame, and `evaluate` skips the
(costly) window frame boundary calculation and the overhead of
calling `evaluate` for each output row.

For example, the `LAG` built in window function does not use
the values of its window frame (it can be computed in one shot
on the entire partition with `Self::evaluate_all` regardless of the
window defined in the `OVER` clause)

```sql
lag(x, 1) OVER (ORDER BY z ROWS BETWEEN 2 PRECEDING AND 3 FOLLOWING)
```

However, `avg()` computes the average in the window and thus
does use its window frame

```sql
avg(x) OVER (PARTITION BY y ORDER BY z ROWS BETWEEN 2 PRECEDING AND 3 FOLLOWING)
```

<a id="op-74222397d87d3d3ff34325f3"></a>
## evaluate_all_with_rank

`function` · `datafusion_expr::partition_evaluator::PartitionEvaluator::evaluate_all_with_rank` · datafusion-expr 55.1.0

```rust
fn evaluate_all_with_rank(&self, _num_rows: usize, _ranks_in_partition: &[Range<usize>]) -> Result<ArrayRef>
```

Source: `src/partition_evaluator.rs:230`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

[`PartitionEvaluator::evaluate_all_with_rank`](../operations/datafusion_expr.partition_evaluator.PartitionEvaluator.md#op-74222397d87d3d3ff34325f3) is called for window
functions that only need the rank of a row within its window
frame.

Evaluate the partition evaluator against the partition using
the row ranks. For example, `RANK(col)` produces

```text
col | rank
--- + ----
 A  | 1
 A  | 1
 C  | 3
 D  | 4
 D  | 4
```

For this case, `num_rows` would be `5` and the
`ranks_in_partition` would be called with

```text
[
  (0,1),
  (2,2),
  (3,4),
]
```

<a id="op-9cb571cf41f2d605287c7817"></a>
## get_range

`function` · `datafusion_expr::partition_evaluator::PartitionEvaluator::get_range` · datafusion-expr 55.1.0

```rust
fn get_range(&self, idx: usize, _n_rows: usize) -> Result<Range<usize>>
```

Source: `src/partition_evaluator.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

If `uses_window_frame` flag is `false`. This method is used to
calculate required range for the window function during
stateful execution.

Generally there is no required range, hence by default this
returns smallest range(current row). e.g seeing current row is
enough to calculate window result (such as row_number, rank,
etc)

<a id="op-0aeef2fe6b407e26dceb7a35"></a>
## include_rank

`function` · `datafusion_expr::partition_evaluator::PartitionEvaluator::include_rank` · datafusion-expr 55.1.0

```rust
fn include_rank(&self) -> bool
```

Source: `src/partition_evaluator.rs:257`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Can this function be evaluated with (only) rank

See the table on [`Self`](../operations/datafusion_expr.partition_evaluator.PartitionEvaluator.md#op-28021de2fba820ec392e9ef2) for what functions to implement

<a id="op-85f9cec84ce90d6ab3b4e6b2"></a>
## is_causal

`function` · `datafusion_expr::partition_evaluator::PartitionEvaluator::is_causal` · datafusion-expr 55.1.0

```rust
fn is_causal(&self) -> bool
```

Source: `src/partition_evaluator.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Get whether evaluator needs future data for its result (if so returns `false`) or not

<a id="op-2b8620b8bb1839ce41bc872f"></a>
## memoize

`function` · `datafusion_expr::partition_evaluator::PartitionEvaluator::memoize` · datafusion-expr 55.1.0

```rust
fn memoize(&mut self, _state: &mut WindowAggState) -> Result<()>
```

Source: `src/partition_evaluator.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

When the window frame has a fixed beginning (e.g UNBOUNDED
PRECEDING), some functions such as FIRST_VALUE, LAST_VALUE and
NTH_VALUE do not need the (unbounded) input once they have
seen a certain amount of input.

`memoize` is called after each input batch is processed, and
such functions can save whatever they need and modify
[`WindowAggState`](../operations/datafusion_expr.window_state.WindowAggState.md#op-f28e52b20ee037aad942edd8) appropriately to allow rows to be pruned

<a id="op-406b7e5a18bc62b1828a694f"></a>
## supports_bounded_execution

`function` · `datafusion_expr::partition_evaluator::PartitionEvaluator::supports_bounded_execution` · datafusion-expr 55.1.0

```rust
fn supports_bounded_execution(&self) -> bool
```

Source: `src/partition_evaluator.rs:242`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Can the window function be incrementally computed using
bounded memory?

See the table on [`Self`](../operations/datafusion_expr.partition_evaluator.PartitionEvaluator.md#op-28021de2fba820ec392e9ef2) for what functions to implement

<a id="op-059ce0e02968bcd6f1dea033"></a>
## uses_window_frame

`function` · `datafusion_expr::partition_evaluator::PartitionEvaluator::uses_window_frame` · datafusion-expr 55.1.0

```rust
fn uses_window_frame(&self) -> bool
```

Source: `src/partition_evaluator.rs:250`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Does the window function use the values from the window frame,
if one is specified?

See the table on [`Self`](../operations/datafusion_expr.partition_evaluator.PartitionEvaluator.md#op-28021de2fba820ec392e9ef2) for what functions to implement
