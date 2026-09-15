# `parquet::arrow::arrow_reader::filter`

Crate `parquet` · 3 public items · structured records in [`model/parquet.arrow.arrow_reader.filter.json`](../model/parquet.arrow.arrow_reader.filter.json)

## ArrowPredicateFn

`struct` · `parquet::arrow::arrow_reader::filter::ArrowPredicateFn`

Also reachable as `parquet::arrow::arrow_reader::ArrowPredicateFn`

```rust
struct ArrowPredicateFn<F>
```

**Implements**: `parquet::arrow::arrow_reader::filter::ArrowPredicate`

**Methods** (1)

```rust
fn new(projection: ProjectionMask, f: F) -> Self
```

**via `parquet::arrow::arrow_reader::filter::ArrowPredicate`**

```rust
fn evaluate(&mut self, batch: RecordBatch) -> Result<BooleanArray, ArrowError>
fn projection(&self) -> &ProjectionMask
```

An [`ArrowPredicate`] created from an [`FnMut`] and a [`ProjectionMask`]

See [`RowFilter`] for more information on applying filters during the
Parquet decoding process.

The function is passed `RecordBatch`es with only the columns specified in
the [`ProjectionMask`].

The function must return a [`BooleanArray`] that has the same length as the
input `batch` where each row indicates whether the row should be returned:
* `true`: the row should be returned
* `false` or `null`: the row should not be returned

# Example:

Given an input schema: `"a:int64", "b:int64"`, you can create a predicate that
evaluates `b > 0` like this:

```
# use std::sync::Arc;
# use arrow::compute::kernels::cmp::gt;
# use arrow_array::{BooleanArray, Int64Array, RecordBatch};
# use arrow_array::cast::AsArray;
# use arrow_array::types::Int64Type;
# use parquet::arrow::arrow_reader::ArrowPredicateFn;
# use parquet::arrow::ProjectionMask;
# use parquet::schema::types::{SchemaDescriptor, Type};
# use parquet::basic; // note there are two `Type`s that are different
# // Schema for a table with one columns: "a" (int64) and "b" (int64)
# let descriptor = SchemaDescriptor::new(
#  Arc::new(
#    Type::group_type_builder("my_schema")
#      .with_fields(vec![
#        Arc::new(
#         Type::primitive_type_builder("a", basic::Type::INT64)
#          .build().unwrap()
#        ),
#        Arc::new(
#         Type::primitive_type_builder("b", basic::Type::INT64)
#          .build().unwrap()
#        ),
#     ])
#     .build().unwrap()
#  )
# );
// Create a mask for selecting only the second column "b" (index 1)
let projection_mask = ProjectionMask::leaves(&descriptor, [1]);
// Closure that evaluates "b > 0"
let predicate = |batch: RecordBatch| {
   let scalar_0 = Int64Array::new_scalar(0);
   let column = batch.column(0).as_primitive::<Int64Type>();
   // call the gt kernel to compute `>` which returns a BooleanArray
   gt(column, &scalar_0)
 };
// Create ArrowPredicateFn that can be passed to RowFilter
let arrow_predicate = ArrowPredicateFn::new(projection_mask, predicate);
```

---

## RowFilter

`struct` · `parquet::arrow::arrow_reader::filter::RowFilter`

Also reachable as `parquet::arrow::arrow_reader::RowFilter`

```rust
struct RowFilter
```

**Derives**: Debug

**Methods** (3)

```rust
fn into_predicates(self) -> Vec<Box<dyn ArrowPredicate>>
fn new(predicates: Vec<Box<dyn ArrowPredicate>>) -> Self
fn predicates(&self) -> &Vec<Box<dyn ArrowPredicate>>
```

Filter applied *during* the parquet read process

See example on [`ArrowReaderBuilder::with_row_filter`]

[`RowFilter`] applies predicates in order, after decoding only the columns
required. As predicates eliminate rows, fewer rows from subsequent columns
may be required, thus potentially reducing IO and decode. This process is
also known as *push down* filtering and  *late materialization*.

A `RowFilter` consists of a list of [`ArrowPredicate`]s. Only the rows for which
all the predicates evaluate to `true` will be returned.
Any [`RowSelection`] provided to the reader will be applied prior
to the first predicate, and each predicate in turn will then be used to compute
a more refined [`RowSelection`] used when evaluating the subsequent predicates.

Once all predicates have been evaluated, the final [`RowSelection`] is applied
to the top-level [`ProjectionMask`] to produce the final output [`RecordBatch`].

This design has a couple of implications:

* [`RowFilter`] can be used to skip entire pages, and thus IO, in addition to CPU decode overheads
* Columns may be decoded multiple times if they appear in multiple [`ProjectionMask`]
* IO will be deferred until needed by a [`ProjectionMask`]

As such there is a trade-off between a single large predicate, or multiple predicates,
that will depend on the shape of the data. Whilst multiple smaller predicates may
minimise the amount of data scanned/decoded, it may not be faster overall.

For example, if a predicate that needs a single column of data filters out all but
1% of the rows, applying it as one of the early `ArrowPredicateFn` will likely significantly
improve performance.

As a counter example, if a predicate needs several columns of data to evaluate but
leaves 99% of the rows, it may be better to not filter the data from parquet and
apply the filter after the RecordBatch has been fully decoded.

Additionally, even if a predicate eliminates a moderate number of rows, it may still be faster
to filter the data after the RecordBatch has been fully decoded, if the eliminated rows are
not contiguous.

[`RowSelection`]: crate::arrow::arrow_reader::RowSelection
[`ArrowReaderBuilder::with_row_filter`]: crate::arrow::arrow_reader::ArrowReaderBuilder::with_row_filter

---

## ArrowPredicate

`trait` · `parquet::arrow::arrow_reader::filter::ArrowPredicate`

Also reachable as `parquet::arrow::arrow_reader::ArrowPredicate`

```rust
trait ArrowPredicate: Send + 'static
```

**Implementors** (1)

- `parquet::arrow::arrow_reader::filter::ArrowPredicateFn`

**Methods** (2)

```rust
fn evaluate(&mut self, batch: RecordBatch) -> Result<BooleanArray, ArrowError>
fn projection(&self) -> &ProjectionMask
```

A predicate operating on [`RecordBatch`]

See also:
* [`RowFilter`] for more information  on applying filters during the
  Parquet decoding process.
* [`ArrowPredicateFn`] for a concrete implementation based on a function

---
