# `buoyant_kernel::transaction::stats_verifier`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transaction.stats_verifier.json).

<a id="op-0bd9419186a7a77f422fa674"></a>
## stats_verifier

`module` · `buoyant_kernel::transaction::stats_verifier` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod stats_verifier
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/stats_verifier.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/stats_verifier.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Validates that add file statistics contain required columns.

Per the Delta protocol, writers MUST write per-file statistics (nullCount, minValues,
maxValues) for certain required columns. For example, clustering columns require stats when
the `ClusteredTable` feature is enabled. This module validates that those stat entries
exist for each required column.
