# `buoyant_kernel::transaction::stats_verifier::StatsColumnVerifier`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transaction.stats_verifier.StatsColumnVerifier.json).

<a id="op-62af8afbd47df55192fc1aa0"></a>
## StatsColumnVerifier

`struct` · `buoyant_kernel::transaction::stats_verifier::StatsColumnVerifier` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct StatsColumnVerifier
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/stats_verifier.rs#L23).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/stats_verifier.rs:23`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Verifies that add file statistics contain required columns.

For each required column, validates that `nullCount` is present (non-null) and that
`minValues` and `maxValues` are present unless the column is all-null
(`nullCount == numRecords`).

<a id="op-57cafa23e4ed3cacc09f560f"></a>
## new

`function` · `buoyant_kernel::transaction::stats_verifier::StatsColumnVerifier::new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(required_columns: Vec<(ColumnName, DataType)>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/stats_verifier.rs#L29).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::stats_verifier::StatsColumnVerifier", "path": "StatsColumnVerifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [101, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/stats_verifier.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/stats_verifier.rs:29`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new verifier that checks statistics for the given required columns and types.

<a id="op-64b2e2a2efb670fcc74e33af"></a>
## verify

`function` · `buoyant_kernel::transaction::stats_verifier::StatsColumnVerifier::verify` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn verify(&self, add_files: &[Box<dyn EngineData>]) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/stats_verifier.rs#L37).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::stats_verifier::StatsColumnVerifier", "path": "StatsColumnVerifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [101, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/stats_verifier.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/stats_verifier.rs:37`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Verify that all files in the provided batches have required statistics.

For each required column, extracts all three stat columns (nullCount, minValues,
maxValues) in a single `visit_rows` call per batch.

<a id="op-da1db3b2ab4c06f76f4d55aa"></a>
## required_columns

`struct_field` · `buoyant_kernel::transaction::stats_verifier::StatsColumnVerifier::required_columns` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
required_columns: Vec<(expressions::ColumnName, schema::DataType)>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/stats_verifier.rs#L24).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/stats_verifier.rs:24`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
