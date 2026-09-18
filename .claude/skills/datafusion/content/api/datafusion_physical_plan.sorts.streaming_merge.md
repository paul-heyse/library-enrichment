# `datafusion_physical_plan::sorts::streaming_merge`

Crate `datafusion-physical-plan` · 2 public items · structured records in [`model/datafusion_physical_plan.sorts.streaming_merge.json`](../model/datafusion_physical_plan.sorts.streaming_merge.json)

## SortedSpillFile

`struct` · `datafusion_physical_plan::sorts::streaming_merge::SortedSpillFile`

```rust
struct SortedSpillFile
```

**Fields**: `file`, `max_record_batch_memory`

**Derives**: Debug

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.sorts.streaming_merge.SortedSpillFile.md).


---

## StreamingMergeBuilder

`struct` · `datafusion_physical_plan::sorts::streaming_merge::StreamingMergeBuilder`

```rust
struct StreamingMergeBuilder<'a>
```

**Derives**: Default

**Methods** (12)

```rust
fn build(self) -> Result<SendableRecordBatchStream>
fn new() -> Self
fn with_batch_size(self, batch_size: usize) -> Self
fn with_expressions(self, expressions: &'a LexOrdering) -> Self
fn with_fetch(self, fetch: Option<usize>) -> Self
fn with_metrics(self, metrics: BaselineMetrics) -> Self
fn with_reservation(self, reservation: MemoryReservation) -> Self
fn with_round_robin_tie_breaker(self, enable_round_robin_tie_breaker: bool) -> Self
fn with_schema(self, schema: SchemaRef) -> Self
fn with_sorted_spill_files(self, sorted_spill_files: Vec<SortedSpillFile>) -> Self
fn with_spill_manager(self, spill_manager: SpillManager) -> Self
fn with_streams(self, streams: Vec<SendableRecordBatchStream>) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.sorts.streaming_merge.StreamingMergeBuilder.md).


---
