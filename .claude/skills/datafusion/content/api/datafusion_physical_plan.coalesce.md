# `datafusion_physical_plan::coalesce`

Crate `datafusion-physical-plan` · 2 public items · structured records in [`model/datafusion_physical_plan.coalesce.json`](../model/datafusion_physical_plan.coalesce.json)

## PushBatchStatus

`enum` · `datafusion_physical_plan::coalesce::PushBatchStatus`

```rust
enum PushBatchStatus
```

**Variants**: `Continue`, `LimitReached`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

Status returned by [`LimitedBatchCoalescer::push_batch`]

---

## LimitedBatchCoalescer

`struct` · `datafusion_physical_plan::coalesce::LimitedBatchCoalescer`

```rust
struct LimitedBatchCoalescer
```

**Derives**: Debug

**Methods** (6)

```rust
fn finish(&mut self) -> Result<()>
fn is_empty(&self) -> bool
fn new(schema: SchemaRef, target_batch_size: usize, fetch: Option<usize>) -> Self
fn next_completed_batch(&mut self) -> Option<RecordBatch>
fn push_batch(&mut self, batch: RecordBatch) -> Result<PushBatchStatus>
fn schema(&self) -> SchemaRef
```

Concatenate multiple [`RecordBatch`]es and apply a limit

See [`BatchCoalescer`] for more details on how this works.

---
