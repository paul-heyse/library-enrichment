# `buoyant_kernel::transaction::stats_verifier`

Crate `buoyant_kernel` · 2 public items · structured records in [`model/buoyant_kernel.transaction.stats_verifier.json`](../model/buoyant_kernel.transaction.stats_verifier.json)

## verify_num_records_present

`function` · `buoyant_kernel::transaction::stats_verifier::verify_num_records_present`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.transaction.stats_verifier.verify_num_records_present.md)

Also reachable as `delta_kernel::transaction::stats_verifier::verify_num_records_present`

```rust
fn verify_num_records_present(add_files: &[Box<dyn EngineData>]) -> DeltaResult<()>
```

Verify that every `add` action has `stats.numRecords` populated. Short-circuits on the first
violation and returns an error containing the `add.path`.

---

## StatsColumnVerifier

`struct` · `buoyant_kernel::transaction::stats_verifier::StatsColumnVerifier`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.transaction.stats_verifier.StatsColumnVerifier.md)

Also reachable as `delta_kernel::transaction::stats_verifier::StatsColumnVerifier`

```rust
struct StatsColumnVerifier
```

**Methods** (2)

```rust
fn new(required_columns: Vec<(ColumnName, DataType)>) -> Self
fn verify(&self, add_files: &[Box<dyn EngineData>]) -> DeltaResult<()>
```

Verifies that add file statistics contain required columns.

For each required column, validates that `nullCount` is present (non-null) and that
`minValues` and `maxValues` are present unless the column is all-null
(`nullCount == numRecords`).

---
