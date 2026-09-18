# `buoyant_kernel::transaction::stats_verifier::verify_num_records_present`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transaction.stats_verifier.verify_num_records_present.json).

<a id="op-ec61488705a4043b7d68ec0a"></a>
## verify_num_records_present

`function` · `buoyant_kernel::transaction::stats_verifier::verify_num_records_present` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn verify_num_records_present(add_files: &[Box<dyn EngineData>]) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/stats_verifier.rs#L302).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/stats_verifier.rs:302`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Verify that every `add` action has `stats.numRecords` populated. Short-circuits on the first
violation and returns an error containing the `add.path`.
