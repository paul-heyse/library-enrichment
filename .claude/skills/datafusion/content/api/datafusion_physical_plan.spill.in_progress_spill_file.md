# `datafusion_physical_plan::spill::in_progress_spill_file`

Crate `datafusion-physical-plan` · 1 public items · structured records in [`model/datafusion_physical_plan.spill.in_progress_spill_file.json`](../model/datafusion_physical_plan.spill.in_progress_spill_file.json)

## InProgressSpillFile

`struct` · `datafusion_physical_plan::spill::in_progress_spill_file::InProgressSpillFile`

```rust
struct InProgressSpillFile
```

Represents an in-progress spill file used for writing `RecordBatch`es to disk, created by `SpillManager`.
Caller is able to use this struct to incrementally append in-memory batches to
the file, and then finalize the file by calling the `finish` method.

---
