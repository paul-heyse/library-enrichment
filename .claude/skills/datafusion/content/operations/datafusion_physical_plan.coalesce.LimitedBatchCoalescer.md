# `datafusion_physical_plan::coalesce::LimitedBatchCoalescer`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.coalesce.LimitedBatchCoalescer.json).

<a id="op-3f1aa30591797055625df08b"></a>
## LimitedBatchCoalescer

`struct` · `datafusion_physical_plan::coalesce::LimitedBatchCoalescer` · datafusion-physical-plan 55.1.0

```rust
struct LimitedBatchCoalescer
```

Source: `src/coalesce/mod.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Concatenate multiple [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es and apply a limit

See [`BatchCoalescer`](../operations/arrow_select.coalesce.BatchCoalescer.md#op-76e0a8f9b55b024998e5f424) for more details on how this works.

<a id="op-e7ce23d8851b2fd194c5d485"></a>
## finish

`function` · `datafusion_physical_plan::coalesce::LimitedBatchCoalescer::finish` · datafusion-physical-plan 55.1.0

```rust
fn finish(&mut self) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce::LimitedBatchCoalescer", "path": "LimitedBatchCoalescer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [145, 2], "filename": "src/coalesce/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/coalesce/mod.rs:131`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Complete the current buffered batch and finish the coalescer

Any subsequent calls to `push_batch()` will return an Err

<a id="op-a87432d5038095a1e0fbb166"></a>
## fmt

`function` · `datafusion_physical_plan::coalesce::LimitedBatchCoalescer::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce::LimitedBatchCoalescer", "path": "LimitedBatchCoalescer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 10], "end": [26, 15], "filename": "src/coalesce/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/coalesce/mod.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-154ee8bdb8e4fe9c871ea144"></a>
## is_empty

`function` · `datafusion_physical_plan::coalesce::LimitedBatchCoalescer::is_empty` · datafusion-physical-plan 55.1.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce::LimitedBatchCoalescer", "path": "LimitedBatchCoalescer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [145, 2], "filename": "src/coalesce/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/coalesce/mod.rs:124`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return true if there is no data buffered

<a id="op-2cb54f754c4e9dcc44953fb2"></a>
## new

`function` · `datafusion_physical_plan::coalesce::LimitedBatchCoalescer::new` · datafusion-physical-plan 55.1.0

```rust
fn new(schema: SchemaRef, target_batch_size: usize, fetch: Option<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce::LimitedBatchCoalescer", "path": "LimitedBatchCoalescer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [145, 2], "filename": "src/coalesce/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/coalesce/mod.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new `BatchCoalescer`

# Arguments
- `schema` - the schema of the output batches
- `target_batch_size` - the minimum number of rows for each
  output batch (until limit reached)
- `fetch` - the maximum number of rows to fetch, `None` means fetch all rows

<a id="op-6c83e483e883a4055d137768"></a>
## next_completed_batch

`function` · `datafusion_physical_plan::coalesce::LimitedBatchCoalescer::next_completed_batch` · datafusion-physical-plan 55.1.0

```rust
fn next_completed_batch(&mut self) -> Option<RecordBatch>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce::LimitedBatchCoalescer", "path": "LimitedBatchCoalescer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [145, 2], "filename": "src/coalesce/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/coalesce/mod.rs:142`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return the next completed batch, if any

<a id="op-a99d2c42cb02132ba137f798"></a>
## push_batch

`function` · `datafusion_physical_plan::coalesce::LimitedBatchCoalescer::push_batch` · datafusion-physical-plan 55.1.0

```rust
fn push_batch(&mut self, batch: RecordBatch) -> Result<PushBatchStatus>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce::LimitedBatchCoalescer", "path": "LimitedBatchCoalescer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [145, 2], "filename": "src/coalesce/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/coalesce/mod.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Pushes the next [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) into the coalescer and returns its status.

# Arguments
* `batch` - The [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) to append.

# Returns
* [`PushBatchStatus::Continue`](../operations/datafusion_physical_plan.coalesce.PushBatchStatus.md#op-99f58d86ebf0ee9c894a7a47) - More batches can still be pushed.
* [`PushBatchStatus::LimitReached`](../operations/datafusion_physical_plan.coalesce.PushBatchStatus.md#op-6dd7d3b1f7b8bcff05ad63d6) - The row limit was reached after processing
  this batch. The caller should call [`Self::finish`](../operations/datafusion_physical_plan.coalesce.LimitedBatchCoalescer.md#op-e7ce23d8851b2fd194c5d485) before retrieving the
  remaining buffered batches.

# Errors
Returns an error if called after [`Self::finish`](../operations/datafusion_physical_plan.coalesce.LimitedBatchCoalescer.md#op-e7ce23d8851b2fd194c5d485) or if the internal push
operation fails.

<a id="op-bb4b657866ef8bec9eb2594f"></a>
## schema

`function` · `datafusion_physical_plan::coalesce::LimitedBatchCoalescer::schema` · datafusion-physical-plan 55.1.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce::LimitedBatchCoalescer", "path": "LimitedBatchCoalescer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [145, 2], "filename": "src/coalesce/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/coalesce/mod.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return the schema of the output batches
