# `parquet::arrow::arrow_reader::filter::RowFilter`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.arrow_reader.filter.RowFilter.json).

<a id="op-a310d73e2b8ee5aca56c0c1f"></a>
## RowFilter

`struct` · `parquet::arrow::arrow_reader::filter::RowFilter` · parquet 59.3.0

```rust
struct RowFilter
```

Source: `src/arrow/arrow_reader/filter.rs:177`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Filter applied *during* the parquet read process

See example on [`ArrowReaderBuilder::with_row_filter`]

[`RowFilter`](../operations/parquet.arrow.arrow_reader.filter.RowFilter.md#op-a310d73e2b8ee5aca56c0c1f) applies predicates in order, after decoding only the columns
required. As predicates eliminate rows, fewer rows from subsequent columns
may be required, thus potentially reducing IO and decode. This process is
also known as *push down* filtering and  *late materialization*.

A `RowFilter` consists of a list of [`ArrowPredicate`](../operations/parquet.arrow.arrow_reader.filter.ArrowPredicate.md#op-bf093cc5f111fb8b2d44eb01)s. Only the rows for which
all the predicates evaluate to `true` will be returned.
Any [`RowSelection`] provided to the reader will be applied prior
to the first predicate, and each predicate in turn will then be used to compute
a more refined [`RowSelection`] used when evaluating the subsequent predicates.

Once all predicates have been evaluated, the final [`RowSelection`] is applied
to the top-level [`ProjectionMask`](../operations/parquet.arrow.ProjectionMask.md#op-28a259558ea082dc20ece1ec) to produce the final output [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34).

This design has a couple of implications:

* [`RowFilter`](../operations/parquet.arrow.arrow_reader.filter.RowFilter.md#op-a310d73e2b8ee5aca56c0c1f) can be used to skip entire pages, and thus IO, in addition to CPU decode overheads
* Columns may be decoded multiple times if they appear in multiple [`ProjectionMask`](../operations/parquet.arrow.ProjectionMask.md#op-28a259558ea082dc20ece1ec)
* IO will be deferred until needed by a [`ProjectionMask`](../operations/parquet.arrow.ProjectionMask.md#op-28a259558ea082dc20ece1ec)

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

<a id="op-764ef138690bbeb54c9b2022"></a>
## fmt

`function` · `parquet::arrow::arrow_reader::filter::RowFilter::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::filter::RowFilter", "path": "RowFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 1], "end": [186, 2], "filename": "src/arrow/arrow_reader/filter.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow/arrow_reader/filter.rs:183`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d247d4f961d14cc56850515"></a>
## into_predicates

`function` · `parquet::arrow::arrow_reader::filter::RowFilter::into_predicates` · parquet 59.3.0

```rust
fn into_predicates(self) -> Vec<Box<dyn ArrowPredicate>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::filter::RowFilter", "path": "RowFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [201, 2], "filename": "src/arrow/arrow_reader/filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/filter.rs:198`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the inner predicates, consuming self

<a id="op-c0671d08d67368c4dca0f3de"></a>
## new

`function` · `parquet::arrow::arrow_reader::filter::RowFilter::new` · parquet 59.3.0

```rust
fn new(predicates: Vec<Box<dyn ArrowPredicate>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::filter::RowFilter", "path": "RowFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [201, 2], "filename": "src/arrow/arrow_reader/filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/filter.rs:190`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new [`RowFilter`](../operations/parquet.arrow.arrow_reader.filter.RowFilter.md#op-a310d73e2b8ee5aca56c0c1f) from an array of [`ArrowPredicate`](../operations/parquet.arrow.arrow_reader.filter.ArrowPredicate.md#op-bf093cc5f111fb8b2d44eb01)

<a id="op-87a9fa59ca89e93d6fd9b644"></a>
## predicates

`function` · `parquet::arrow::arrow_reader::filter::RowFilter::predicates` · parquet 59.3.0

```rust
fn predicates(&self) -> &Vec<Box<dyn ArrowPredicate>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::filter::RowFilter", "path": "RowFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [201, 2], "filename": "src/arrow/arrow_reader/filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/filter.rs:194`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the inner predicates
