# `buoyant_kernel::log_compaction::writer::LogCompactionWriter`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.log_compaction.writer.LogCompactionWriter.json).

<a id="op-17a0145eb5a991e6c3d78695"></a>
## LogCompactionWriter

`struct` · `buoyant_kernel::log_compaction::writer::LogCompactionWriter` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct LogCompactionWriter
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_compaction/writer.rs#L28).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_compaction/writer.rs:28`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Writer for log compaction files

This writer provides an API for creating log compaction files that aggregate actions
from multiple commit files.

<a id="op-22e016133c1adf88567054e3"></a>
## compaction_data

`function` · `buoyant_kernel::log_compaction::writer::LogCompactionWriter::compaction_data` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn compaction_data(&mut self, engine: &dyn Engine) -> DeltaResult<ActionReconciliationIterator>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_compaction/writer.rs#L88).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::log_compaction::writer::LogCompactionWriter", "path": "LogCompactionWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [130, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_compaction/writer.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_compaction/writer.rs:88`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Get an iterator over the compaction data to be written

Performs action reconciliation for the version range specified in the constructor

<a id="op-399f44d6661e48504b066ee8"></a>
## compaction_path

`function` · `buoyant_kernel::log_compaction::writer::LogCompactionWriter::compaction_path` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn compaction_path(&self) -> &url::Url
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_compaction/writer.rs#L81).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::log_compaction::writer::LogCompactionWriter", "path": "LogCompactionWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [130, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_compaction/writer.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_compaction/writer.rs:81`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Get the path where the compaction file will be written

<a id="op-0ad317f942e492fb7081432d"></a>
## fmt

`function` · `buoyant_kernel::log_compaction::writer::LogCompactionWriter::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_compaction/writer.rs#L27).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::log_compaction::writer::LogCompactionWriter", "path": "LogCompactionWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 10], "end": [27, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_compaction/writer.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_compaction/writer.rs:27`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-912bbfe809d7ff9700198dfb"></a>
## compaction_path

`struct_field` · `buoyant_kernel::log_compaction::writer::LogCompactionWriter::compaction_path` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
compaction_path: url::Url
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_compaction/writer.rs#L39).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_compaction/writer.rs:39`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Cached compaction file path

<a id="op-c6bde6a077fcb37c0b4c58cb"></a>
## end_version

`struct_field` · `buoyant_kernel::log_compaction::writer::LogCompactionWriter::end_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
end_version: Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_compaction/writer.rs#L36).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_compaction/writer.rs:36`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

End version of commits included in this compaction (exclusive)

<a id="op-e5b5b96a45043f2187498086"></a>
## snapshot

`struct_field` · `buoyant_kernel::log_compaction::writer::LogCompactionWriter::snapshot` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
snapshot: SnapshotRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_compaction/writer.rs#L30).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_compaction/writer.rs:30`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Reference to the snapshot of the table being compacted

<a id="op-38813886b06f0d8c35efe47d"></a>
## start_version

`struct_field` · `buoyant_kernel::log_compaction::writer::LogCompactionWriter::start_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
start_version: Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_compaction/writer.rs#L33).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_compaction/writer.rs:33`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Starting version of commits included in this compaction (inclusive)

<a id="op-9c2a30baeb7016d39308b027"></a>
## table_properties

`function` · `buoyant_kernel::log_compaction::writer::LogCompactionWriter::table_properties` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn table_properties(&self) -> &TableProperties
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_compaction/writer.rs#L43).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::log_compaction::writer::LogCompactionWriter", "path": "LogCompactionWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [46, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_compaction/writer.rs"}, "trait": {"args": null, "id": "buoyant_kernel::action_reconciliation::RetentionCalculator", "path": "RetentionCalculator"}, "trait_path": "buoyant_kernel::action_reconciliation::RetentionCalculator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_compaction/writer.rs:43`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
