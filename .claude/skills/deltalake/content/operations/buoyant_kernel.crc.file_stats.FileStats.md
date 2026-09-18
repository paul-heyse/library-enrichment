# `buoyant_kernel::crc::file_stats::FileStats`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.crc.file_stats.FileStats.json).

<a id="op-0ae222f09a8ef0e02c9e6fa6"></a>
## FileStats

`struct` · `buoyant_kernel::crc::file_stats::FileStats` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct FileStats
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/file_stats.rs#L28).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_stats.rs:28`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

File-level statistics for a table version: total file count, size, and histogram.

Obtained via [`Snapshot::get_file_stats_if_present`] or [`Crc::file_stats()`]. Returns
`None` when the source CRC's `file_stats_state` is not `Complete`.

[`Snapshot::get_file_stats_if_present`]: crate::snapshot::Snapshot::get_file_stats_if_present
[`Crc::file_stats()`]: super::Crc::file_stats

<a id="op-376ee1523d8018a397eae481"></a>
## clone

`function` · `buoyant_kernel::crc::file_stats::FileStats::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> FileStats
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/file_stats.rs#L27).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::file_stats::FileStats", "path": "FileStats"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 17], "end": [27, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_stats.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_stats.rs:27`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d7b5eaf182d197083e1578d2"></a>
## default

`function` · `buoyant_kernel::crc::file_stats::FileStats::default` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> FileStats
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/file_stats.rs#L27).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::file_stats::FileStats", "path": "FileStats"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 24], "end": [27, 31], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_stats.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_stats.rs:27`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-65825d27a4d148a7453fed04"></a>
## eq

`function` · `buoyant_kernel::crc::file_stats::FileStats::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &FileStats) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/file_stats.rs#L27).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::file_stats::FileStats", "path": "FileStats"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 33], "end": [27, 42], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_stats.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_stats.rs:27`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e786a3f827dc99ad6ac0f9d1"></a>
## file_size_histogram

`function` · `buoyant_kernel::crc::file_stats::FileStats::file_size_histogram` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn file_size_histogram(&self) -> Option<&FileSizeHistogram>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/file_stats.rs#L52).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::file_stats::FileStats", "path": "FileStats"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [55, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_stats.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_stats.rs:52`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the size distribution of active files, if available.

<a id="op-1685382437fccfd83dc83d8b"></a>
## fmt

`function` · `buoyant_kernel::crc::file_stats::FileStats::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/file_stats.rs#L27).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::file_stats::FileStats", "path": "FileStats"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 10], "end": [27, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_stats.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_stats.rs:27`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47391c727509f29ba34195c0"></a>
## num_files

`function` · `buoyant_kernel::crc::file_stats::FileStats::num_files` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn num_files(&self) -> i64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/file_stats.rs#L41).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::file_stats::FileStats", "path": "FileStats"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [55, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_stats.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_stats.rs:41`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the number of active [`Add`](crate::actions::Add) file actions in this table
version.

<a id="op-09960a936cd6ed15e7987c38"></a>
## table_size_bytes

`function` · `buoyant_kernel::crc::file_stats::FileStats::table_size_bytes` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn table_size_bytes(&self) -> i64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/file_stats.rs#L47).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::file_stats::FileStats", "path": "FileStats"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [55, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_stats.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_stats.rs:47`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the total size of the table in bytes (sum of all active
[`Add`](crate::actions::Add) file sizes).

<a id="op-a6afeea3a9432843c1fd280c"></a>
## file_size_histogram

`struct_field` · `buoyant_kernel::crc::file_stats::FileStats::file_size_histogram` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
file_size_histogram: Option<super::FileSizeHistogram>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/file_stats.rs#L35).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_stats.rs:35`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Size distribution of active files, if available.

<a id="op-f09b8ba77ce3a2b54ad8eb4b"></a>
## num_files

`struct_field` · `buoyant_kernel::crc::file_stats::FileStats::num_files` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
num_files: i64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/file_stats.rs#L30).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_stats.rs:30`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Number of active [`Add`](crate::actions::Add) file actions in this table version.

<a id="op-198f36d8324464eb9fdc89c5"></a>
## table_size_bytes

`struct_field` · `buoyant_kernel::crc::file_stats::FileStats::table_size_bytes` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
table_size_bytes: i64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/file_stats.rs#L33).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_stats.rs:33`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Total size of the table in bytes (sum of all active
[`Add`](crate::actions::Add) file sizes).
