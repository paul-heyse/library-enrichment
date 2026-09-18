# `parquet::arrow::arrow_reader::filter::ArrowPredicate`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.arrow_reader.filter.ArrowPredicate.json).

<a id="op-bf093cc5f111fb8b2d44eb01"></a>
## ArrowPredicate

`trait` · `parquet::arrow::arrow_reader::filter::ArrowPredicate` · parquet 59.3.0

```rust
trait ArrowPredicate: Send + 'static
```

Source: `src/arrow/arrow_reader/filter.rs:29`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A predicate operating on [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)

See also:
* [`RowFilter`](../operations/parquet.arrow.arrow_reader.filter.RowFilter.md#op-a310d73e2b8ee5aca56c0c1f) for more information  on applying filters during the
  Parquet decoding process.
* [`ArrowPredicateFn`](../operations/parquet.arrow.arrow_reader.filter.ArrowPredicateFn.md#op-6817a0a6bc2078986ae8a2d4) for a concrete implementation based on a function

<a id="op-5bc1d02594b2f31305ee52ca"></a>
## evaluate

`function` · `parquet::arrow::arrow_reader::filter::ArrowPredicate::evaluate` · parquet 59.3.0

```rust
fn evaluate(&mut self, batch: RecordBatch) -> Result<BooleanArray, ArrowError>
```

Source: `src/arrow/arrow_reader/filter.rs:46`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Evaluate this predicate for the given [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) containing the columns
identified by [`Self::projection`](../operations/parquet.arrow.arrow_reader.filter.ArrowPredicate.md#op-c80615c0ff782ac56d7f5515)

Must return a [`BooleanArray`](../operations/arrow_array.array.boolean_array.BooleanArray.md#op-da9041df4f2e5d0ab93f8505) that has the same length as the input
`batch` where each row indicates whether the row should be returned:
* `true`:the row should be returned
* `false` or `null`: the row should not be returned

<a id="op-c80615c0ff782ac56d7f5515"></a>
## projection

`function` · `parquet::arrow::arrow_reader::filter::ArrowPredicate::projection` · parquet 59.3.0

```rust
fn projection(&self) -> &ProjectionMask
```

Source: `src/arrow/arrow_reader/filter.rs:37`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the [`ProjectionMask`](../operations/parquet.arrow.ProjectionMask.md#op-28a259558ea082dc20ece1ec) that describes the columns required
to evaluate this predicate.

All projected columns will be provided in the `batch` passed to
[`evaluate`](Self::evaluate). The projection mask should be as small as
possible because any columns needed for the overall projection mask are
decoded again after a predicate is applied.
