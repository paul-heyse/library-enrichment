# `arrow_select::coalesce::BatchCoalescer`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_select.coalesce.BatchCoalescer.json).

<a id="op-76e0a8f9b55b024998e5f424"></a>
## BatchCoalescer

`struct` · `arrow_select::coalesce::BatchCoalescer` · arrow-select 59.3.0

```rust
struct BatchCoalescer
```

Source: `src/coalesce.rs:148`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Concatenate multiple [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es

Implements the common pattern of incrementally creating output
[`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es of a specific size from an input stream of
[`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es.

This is useful after operations such as [`filter`] and [`take`] that produce
smaller batches, and we want to coalesce them into larger batches for
further processing.

# Motivation

If we use [`concat_batches`] to implement the same functionality, there are 2 potential issues:
1. At least 2x peak memory (holding the input and output of concat)
2. 2 copies of the data (to create the output of filter and then create the output of concat)

See: <https://github.com/apache/arrow-rs/issues/6692> for more discussions
about the motivation.

[`filter`]: crate::filter::filter
[`take`]: crate::take::take
[`concat_batches`]: crate::concat::concat_batches

# Example
```
use arrow_array::record_batch;
use arrow_select::coalesce::{BatchCoalescer};
let batch1 = record_batch!(("a", Int32, [1, 2, 3])).unwrap();
let batch2 = record_batch!(("a", Int32, [4, 5])).unwrap();

// Create a `BatchCoalescer` that will produce batches with at least 4 rows
let target_batch_size = 4;
let mut coalescer = BatchCoalescer::new(batch1.schema(), 4);

// push the batches
coalescer.push_batch(batch1).unwrap();
// only pushed 3 rows (not yet 4, enough to produce a batch)
assert!(coalescer.next_completed_batch().is_none());
coalescer.push_batch(batch2).unwrap();
// now we have 5 rows, so we can produce a batch
let finished = coalescer.next_completed_batch().unwrap();
// 4 rows came out (target batch size is 4)
let expected = record_batch!(("a", Int32, [1, 2, 3, 4])).unwrap();
assert_eq!(finished, expected);

// Have no more input, but still have an in-progress batch
assert!(coalescer.next_completed_batch().is_none());
// We can finish the batch, which will produce the remaining rows
coalescer.finish_buffered_batch().unwrap();
let expected = record_batch!(("a", Int32, [5])).unwrap();
assert_eq!(coalescer.next_completed_batch().unwrap(), expected);

// The coalescer is now empty
assert!(coalescer.next_completed_batch().is_none());
```

# Background

Generally speaking, larger [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es are more efficient to process
than smaller [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es (until the CPU cache is exceeded) because
there is fixed processing overhead per batch. This coalescer builds up these
larger batches incrementally.

```text
┌────────────────────┐
│    RecordBatch     │
│   num_rows = 100   │
└────────────────────┘                 ┌────────────────────┐
                                       │                    │
┌────────────────────┐     Coalesce    │                    │
│                    │      Batches    │                    │
│    RecordBatch     │                 │                    │
│   num_rows = 200   │  ─ ─ ─ ─ ─ ─ ▶  │                    │
│                    │                 │    RecordBatch     │
│                    │                 │   num_rows = 400   │
└────────────────────┘                 │                    │
                                       │                    │
┌────────────────────┐                 │                    │
│                    │                 │                    │
│    RecordBatch     │                 │                    │
│   num_rows = 100   │                 └────────────────────┘
│                    │
└────────────────────┘
```

# Notes:

1. Output rows are produced in the same order as the input rows

2. The output is a sequence of batches, with all but the last being at exactly
   `target_batch_size` rows.

<a id="op-5725127b1365f20ac91e5a7f"></a>
## biggest_coalesce_batch_size

`function` · `arrow_select::coalesce::BatchCoalescer::biggest_coalesce_batch_size` · arrow-select 59.3.0

```rust
fn biggest_coalesce_batch_size(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_select::coalesce::BatchCoalescer", "path": "BatchCoalescer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 1], "end": [602, 2], "filename": "src/coalesce.rs"}, "trait": null, "trait_path": null}`

Source: `src/coalesce.rs:218`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Get the current biggest coalesce batch size limit

See [`Self::with_biggest_coalesce_batch_size`](../operations/arrow_select.coalesce.BatchCoalescer.md#op-124f677e969ea01083b6ec07) for details

<a id="op-22ca9b3fc4d9885cb81352e3"></a>
## finish_buffered_batch

`function` · `arrow_select::coalesce::BatchCoalescer::finish_buffered_batch` · arrow-select 59.3.0

```rust
fn finish_buffered_batch(&mut self) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_select::coalesce::BatchCoalescer", "path": "BatchCoalescer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 1], "end": [602, 2], "filename": "src/coalesce.rs"}, "trait": null, "trait_path": null}`

Source: `src/coalesce.rs:547`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Concatenates any buffered batches into a single `RecordBatch` and
clears any output buffers

Normally this is called when the input stream is exhausted, and
we want to finalize the last batch of rows.

See [`Self::next_completed_batch()`](../operations/arrow_select.coalesce.BatchCoalescer.md#op-c748cb59ec3643dfcc8168aa) for the completed batches.

<a id="op-4dab6dc8e0b5b31e5bfc17af"></a>
## fmt

`function` · `arrow_select::coalesce::BatchCoalescer::fmt` · arrow-select 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_select::coalesce::BatchCoalescer", "path": "BatchCoalescer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 10], "end": [147, 15], "filename": "src/coalesce.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/coalesce.rs:147`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-325ab46095b8898fdbac0967"></a>
## get_buffered_rows

`function` · `arrow_select::coalesce::BatchCoalescer::get_buffered_rows` · arrow-select 59.3.0

```rust
fn get_buffered_rows(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_select::coalesce::BatchCoalescer", "path": "BatchCoalescer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 1], "end": [602, 2], "filename": "src/coalesce.rs"}, "trait": null, "trait_path": null}`

Source: `src/coalesce.rs:536`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Returns the number of buffered rows

<a id="op-fe39a436fe71a600d14ec02f"></a>
## has_completed_batch

`function` · `arrow_select::coalesce::BatchCoalescer::has_completed_batch` · arrow-select 59.3.0

```rust
fn has_completed_batch(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_select::coalesce::BatchCoalescer", "path": "BatchCoalescer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 1], "end": [602, 2], "filename": "src/coalesce.rs"}, "trait": null, "trait_path": null}`

Source: `src/coalesce.rs:578`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Returns true if there are any completed batches

<a id="op-448c9c81b5254293e4833023"></a>
## is_empty

`function` · `arrow_select::coalesce::BatchCoalescer::is_empty` · arrow-select 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_select::coalesce::BatchCoalescer", "path": "BatchCoalescer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 1], "end": [602, 2], "filename": "src/coalesce.rs"}, "trait": null, "trait_path": null}`

Source: `src/coalesce.rs:573`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Returns true if there is any buffered data

<a id="op-c4a2e06d08ad8443d55dc03b"></a>
## new

`function` · `arrow_select::coalesce::BatchCoalescer::new` · arrow-select 59.3.0

```rust
fn new(schema: SchemaRef, target_batch_size: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_select::coalesce::BatchCoalescer", "path": "BatchCoalescer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 1], "end": [602, 2], "filename": "src/coalesce.rs"}, "trait": null, "trait_path": null}`

Source: `src/coalesce.rs:175`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Create a new `BatchCoalescer`

# Arguments
- `schema` - the schema of the output batches
- `target_batch_size` - the number of rows in each output batch.
  Typical values are `4096` or `8192` rows.


<a id="op-c748cb59ec3643dfcc8168aa"></a>
## next_completed_batch

`function` · `arrow_select::coalesce::BatchCoalescer::next_completed_batch` · arrow-select 59.3.0

```rust
fn next_completed_batch(&mut self) -> Option<RecordBatch>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_select::coalesce::BatchCoalescer", "path": "BatchCoalescer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 1], "end": [602, 2], "filename": "src/coalesce.rs"}, "trait": null, "trait_path": null}`

Source: `src/coalesce.rs:583`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Removes and returns the next completed batch, if any.

<a id="op-075c8b56053a62116f4064fb"></a>
## push_batch

`function` · `arrow_select::coalesce::BatchCoalescer::push_batch` · arrow-select 59.3.0

```rust
fn push_batch(&mut self, batch: RecordBatch) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_select::coalesce::BatchCoalescer", "path": "BatchCoalescer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 1], "end": [602, 2], "filename": "src/coalesce.rs"}, "trait": null, "trait_path": null}`

Source: `src/coalesce.rs:325`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Push all the rows from `batch` into the Coalescer

When buffered data plus incoming rows reach `target_batch_size` ,
completed batches are generated eagerly and can be retrieved via
[`Self::next_completed_batch()`](../operations/arrow_select.coalesce.BatchCoalescer.md#op-c748cb59ec3643dfcc8168aa).
Output batches contain exactly `target_batch_size` rows, so the tail of
the input batch may remain buffered.
Remaining partial data either waits for future input batches or can be
materialized immediately by calling [`Self::finish_buffered_batch()`](../operations/arrow_select.coalesce.BatchCoalescer.md#op-22ca9b3fc4d9885cb81352e3).

# Example
```
# use arrow_array::record_batch;
# use arrow_select::coalesce::BatchCoalescer;
let batch1 = record_batch!(("a", Int32, [1, 2, 3])).unwrap();
let batch2 = record_batch!(("a", Int32, [4, 5, 6])).unwrap();
// create a new Coalescer that targets creating 1000 row batches
let mut coalescer = BatchCoalescer::new(batch1.schema(), 1000);
coalescer.push_batch(batch1);
coalescer.push_batch(batch2);
// finsh and retrieve the created batch
coalescer.finish_buffered_batch().unwrap();
let completed_batch = coalescer.next_completed_batch().unwrap();
let expected_batch = record_batch!(("a", Int32, [1, 2, 3, 4, 5, 6])).unwrap();
assert_eq!(completed_batch, expected_batch);
```

<a id="op-6b6f2aefd75314df7d9112ca"></a>
## push_batch_with_filter

`function` · `arrow_select::coalesce::BatchCoalescer::push_batch_with_filter` · arrow-select 59.3.0

```rust
fn push_batch_with_filter(&mut self, batch: RecordBatch, filter: &BooleanArray) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_select::coalesce::BatchCoalescer", "path": "BatchCoalescer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 1], "end": [602, 2], "filename": "src/coalesce.rs"}, "trait": null, "trait_path": null}`

Source: `src/coalesce.rs:258`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Push a batch into the Coalescer after applying a filter

This is semantically equivalent of calling [`Self::push_batch`](../operations/arrow_select.coalesce.BatchCoalescer.md#op-075c8b56053a62116f4064fb)
with the results from [`crate::filter::filter_record_batch`](../operations/arrow_select.filter.filter_record_batch.md#op-50a068de42a854681b747251)

# Example
```
# use arrow_array::{record_batch, BooleanArray};
# use arrow_select::coalesce::BatchCoalescer;
let batch1 = record_batch!(("a", Int32, [1, 2, 3])).unwrap();
let batch2 = record_batch!(("a", Int32, [4, 5, 6])).unwrap();
// Apply a filter to each batch to pick the first and last row
let filter = BooleanArray::from(vec![true, false, true]);
// create a new Coalescer that targets creating 1000 row batches
let mut coalescer = BatchCoalescer::new(batch1.schema(), 1000);
coalescer.push_batch_with_filter(batch1, &filter);
coalescer.push_batch_with_filter(batch2, &filter);
// finsh and retrieve the created batch
coalescer.finish_buffered_batch().unwrap();
let completed_batch = coalescer.next_completed_batch().unwrap();
// filtered out 2 and 5:
let expected_batch = record_batch!(("a", Int32, [1, 3, 4, 6])).unwrap();
assert_eq!(completed_batch, expected_batch);
```

<a id="op-81e7a18488f3d01eff93e1c9"></a>
## push_batch_with_indices

`function` · `arrow_select::coalesce::BatchCoalescer::push_batch_with_indices` · arrow-select 59.3.0

```rust
fn push_batch_with_indices(&mut self, batch: RecordBatch, indices: &dyn Array) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_select::coalesce::BatchCoalescer", "path": "BatchCoalescer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 1], "end": [602, 2], "filename": "src/coalesce.rs"}, "trait": null, "trait_path": null}`

Source: `src/coalesce.rs:289`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Push a batch into the Coalescer after applying a set of indices
This is semantically equivalent of calling [`Self::push_batch`](../operations/arrow_select.coalesce.BatchCoalescer.md#op-075c8b56053a62116f4064fb)
with the results from  [`take_record_batch`](../operations/arrow_select.take.take_record_batch.md#op-929c00ffe22d86d208c23db7)

# Example
```
# use arrow_array::{record_batch, UInt64Array};
# use arrow_select::coalesce::BatchCoalescer;
let batch1 = record_batch!(("a", Int32, [0, 0, 0])).unwrap();
let batch2 = record_batch!(("a", Int32, [1, 1, 4, 5, 1, 4])).unwrap();
// Sorted indices to create a sorted output, this can be obtained with
// `arrow-ord`'s sort_to_indices operation
let indices = UInt64Array::from(vec![0, 1, 4, 2, 5, 3]);
// create a new Coalescer that targets creating 1000 row batches
let mut coalescer = BatchCoalescer::new(batch1.schema(), 1000);
coalescer.push_batch(batch1);
coalescer.push_batch_with_indices(batch2, &indices);
// finsh and retrieve the created batch
coalescer.finish_buffered_batch().unwrap();
let completed_batch = coalescer.next_completed_batch().unwrap();
let expected_batch = record_batch!(("a", Int32, [0, 0, 0, 1, 1, 1, 4, 4, 5])).unwrap();
assert_eq!(completed_batch, expected_batch);
```

<a id="op-dba930bd745879b559bcbfe5"></a>
## schema

`function` · `arrow_select::coalesce::BatchCoalescer::schema` · arrow-select 59.3.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_select::coalesce::BatchCoalescer", "path": "BatchCoalescer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 1], "end": [602, 2], "filename": "src/coalesce.rs"}, "trait": null, "trait_path": null}`

Source: `src/coalesce.rs:230`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Return the schema of the output batches

<a id="op-d99429526c0e50377d4825a6"></a>
## set_biggest_coalesce_batch_size

`function` · `arrow_select::coalesce::BatchCoalescer::set_biggest_coalesce_batch_size` · arrow-select 59.3.0

```rust
fn set_biggest_coalesce_batch_size(&mut self, limit: Option<usize>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_select::coalesce::BatchCoalescer", "path": "BatchCoalescer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 1], "end": [602, 2], "filename": "src/coalesce.rs"}, "trait": null, "trait_path": null}`

Source: `src/coalesce.rs:225`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Set the biggest coalesce batch size limit

See [`Self::with_biggest_coalesce_batch_size`](../operations/arrow_select.coalesce.BatchCoalescer.md#op-124f677e969ea01083b6ec07) for details

<a id="op-fb8b80005228a272df0210b7"></a>
## size

`function` · `arrow_select::coalesce::BatchCoalescer::size` · arrow-select 59.3.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_select::coalesce::BatchCoalescer", "path": "BatchCoalescer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 1], "end": [602, 2], "filename": "src/coalesce.rs"}, "trait": null, "trait_path": null}`

Source: `src/coalesce.rs:588`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Returns the number of bytes used by this data structure.

<a id="op-124f677e969ea01083b6ec07"></a>
## with_biggest_coalesce_batch_size

`function` · `arrow_select::coalesce::BatchCoalescer::with_biggest_coalesce_batch_size` · arrow-select 59.3.0

```rust
fn with_biggest_coalesce_batch_size(self, limit: Option<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_select::coalesce::BatchCoalescer", "path": "BatchCoalescer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 1], "end": [602, 2], "filename": "src/coalesce.rs"}, "trait": null, "trait_path": null}`

Source: `src/coalesce.rs:210`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Set the coalesce batch size limit (default `None`)

This limit determine when batches should bypass coalescing. Intuitively,
batches that are already large are costly to coalesce and are efficient
enough to process directly without coalescing.

If `Some(limit)`, batches larger than this limit will bypass coalescing
when there is no buffered data, or when the previously buffered data
already exceeds this limit.

If `None`, all batches will be coalesced according to the
target_batch_size.
