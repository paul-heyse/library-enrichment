# `datafusion_physical_plan::spill::in_progress_spill_file::InProgressSpillFile`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.spill.in_progress_spill_file.InProgressSpillFile.json).

<a id="op-5c2e32a41adcc035c9528119"></a>
## InProgressSpillFile

`struct` · `datafusion_physical_plan::spill::in_progress_spill_file::InProgressSpillFile` · datafusion-physical-plan 55.1.0

```rust
struct InProgressSpillFile
```

Source: `src/spill/in_progress_spill_file.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Represents an in-progress spill file used for writing `RecordBatch`es to disk, created by `SpillManager`.
Caller is able to use this struct to incrementally append in-memory batches to
the file, and then finalize the file by calling the `finish` method.
