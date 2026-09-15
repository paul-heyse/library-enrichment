# `arrow_select::coalesce`

Crate `arrow-select` · 1 public items · structured records in [`model/arrow_select.coalesce.json`](../model/arrow_select.coalesce.json)

## BatchCoalescer

`struct` · `arrow_select::coalesce::BatchCoalescer`

Also reachable as `arrow::compute::BatchCoalescer`, `arrow::compute::kernels::coalesce::BatchCoalescer`

```rust
struct BatchCoalescer
```

**Derives**: Debug

**Methods** (14)

```rust
fn biggest_coalesce_batch_size(&self) -> Option<usize>
fn finish_buffered_batch(&mut self) -> Result<(), ArrowError>
fn get_buffered_rows(&self) -> usize
fn has_completed_batch(&self) -> bool
fn is_empty(&self) -> bool
fn new(schema: SchemaRef, target_batch_size: usize) -> Self
fn next_completed_batch(&mut self) -> Option<RecordBatch>
fn push_batch(&mut self, batch: RecordBatch) -> Result<(), ArrowError>
fn push_batch_with_filter(&mut self, batch: RecordBatch, filter: &BooleanArray) -> Result<(), ArrowError>
fn push_batch_with_indices(&mut self, batch: RecordBatch, indices: &dyn Array) -> Result<(), ArrowError>
fn schema(&self) -> SchemaRef
fn set_biggest_coalesce_batch_size(&mut self, limit: Option<usize>)
fn size(&self) -> usize
fn with_biggest_coalesce_batch_size(self, limit: Option<usize>) -> Self
```

Concatenate multiple [`RecordBatch`]es

Implements the common pattern of incrementally creating output
[`RecordBatch`]es of a specific size from an input stream of
[`RecordBatch`]es.

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

Generally speaking, larger [`RecordBatch`]es are more efficient to process
than smaller [`RecordBatch`]es (until the CPU cache is exceeded) because
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

---
