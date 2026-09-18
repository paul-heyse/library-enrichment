# `buoyant_kernel::crc::DeletedRecordCountsHistogram`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.crc.DeletedRecordCountsHistogram.json).

<a id="op-1c12178a256bbc6b53217a0d"></a>
## DeletedRecordCountsHistogram

`struct` · `buoyant_kernel::crc::DeletedRecordCountsHistogram` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DeletedRecordCountsHistogram
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/mod.rs#L299).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs:299`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The [DeletedRecordCountsHistogram] object represents a histogram tracking the distribution of
deleted record counts across files in the table. Each bin in the histogram represents a range
of deletion counts and stores the number of files having that many deleted records.

The histogram bins correspond to the following ranges:
Bin 0: [0, 0] (files with no deletions)
Bin 1: [1, 9] (files with 1-9 deleted records)
Bin 2: [10, 99] (files with 10-99 deleted records)
Bin 3: [100, 999] (files with 100-999 deleted records)
Bin 4: [1000, 9999] (files with 1,000-9,999 deleted records)
Bin 5: [10000, 99999] (files with 10,000-99,999 deleted records)
Bin 6: [100000, 999999] (files with 100,000-999,999 deleted records)
Bin 7: [1000000, 9999999] (files with 1,000,000-9,999,999 deleted records)
Bin 8: [10000000, 2147483646] (files with 10,000,000 to 2,147,483,646 deleted records)
Bin 9: [2147483647, inf) (files with 2,147,483,647 or more deleted records)

[DeletedRecordCountsHistogram]: https://github.com/delta-io/delta/blob/master/PROTOCOL.md#deleted-record-counts-histogram-schema

<a id="op-17ad239fa09c08ffb36f51a1"></a>
## clone

`function` · `buoyant_kernel::crc::DeletedRecordCountsHistogram::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> DeletedRecordCountsHistogram
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/mod.rs#L298).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::DeletedRecordCountsHistogram", "path": "DeletedRecordCountsHistogram"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [298, 17], "end": [298, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs:298`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84bc93d5113eddacb17fe255"></a>
## eq

`function` · `buoyant_kernel::crc::DeletedRecordCountsHistogram::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &DeletedRecordCountsHistogram) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/mod.rs#L298).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::DeletedRecordCountsHistogram", "path": "DeletedRecordCountsHistogram"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [298, 24], "end": [298, 33], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs:298`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bcd57f060d38304987e28223"></a>
## fmt

`function` · `buoyant_kernel::crc::DeletedRecordCountsHistogram::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/mod.rs#L298).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::DeletedRecordCountsHistogram", "path": "DeletedRecordCountsHistogram"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [298, 10], "end": [298, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs:298`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b4aefd0be47f0a096f8e1e94"></a>
## deleted_record_counts

`struct_field` · `buoyant_kernel::crc::DeletedRecordCountsHistogram::deleted_record_counts` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
deleted_record_counts: Vec<i64>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/mod.rs#L302).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs:302`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Array of size 10 where each element represents the count of files falling into a specific
deletion count range.
