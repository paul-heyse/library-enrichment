# `buoyant_kernel::log_segment_files::LogSegmentFiles`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.log_segment_files.LogSegmentFiles.json).

<a id="op-65a62a8e62a765ce9da36e5d"></a>
## LogSegmentFiles

`struct` · `buoyant_kernel::log_segment_files::LogSegmentFiles` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct LogSegmentFiles
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment_files/mod.rs#L51).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment_files/mod.rs:51`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Represents the set of log files found during a listing operation in the Delta log directory.

- `ascending_commit_files`: All commit and staged commit files found, sorted by version. May
  contain gaps.
- `ascending_compaction_files`: All compaction commit files found, sorted by version.
- `checkpoint_parts`: All parts of the most recent complete checkpoint (all same version). Empty
  if no checkpoint found.
- `latest_crc_file`: The CRC file with the highest version, only if version >= checkpoint
  version.
- `latest_commit_file`: The commit file with the highest version, or `None` if no commits were
  found. This field may be present even when `ascending_commit_files` is empty, such as when a
  checkpoint subsumes all commits. In that case, it is retained because downstream code (e.g.
  In-Commit Timestamp reading) needs access to the commit file at the snapshot version.
- `max_published_version`: The highest published commit file version, or `None` if no published
  commits were found.

<a id="op-41bab7caf91fedf2e4cfb644"></a>
## ascending_commit_files

`struct_field` · `buoyant_kernel::log_segment_files::LogSegmentFiles::ascending_commit_files` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ascending_commit_files: Vec<path::ParsedLogPath>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment_files/mod.rs#L52).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment_files/mod.rs:52`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0582f36b1ceb66f2648ba13"></a>
## ascending_compaction_files

`struct_field` · `buoyant_kernel::log_segment_files::LogSegmentFiles::ascending_compaction_files` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ascending_compaction_files: Vec<path::ParsedLogPath>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment_files/mod.rs#L53).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment_files/mod.rs:53`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31bf657466045a589f7a28c9"></a>
## checkpoint_parts

`struct_field` · `buoyant_kernel::log_segment_files::LogSegmentFiles::checkpoint_parts` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
checkpoint_parts: Vec<path::ParsedLogPath>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment_files/mod.rs#L54).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment_files/mod.rs:54`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a94db7d4f8d3dcafdb290b9"></a>
## clone

`function` · `buoyant_kernel::log_segment_files::LogSegmentFiles::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> LogSegmentFiles
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment_files/mod.rs#L49).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::log_segment_files::LogSegmentFiles", "path": "LogSegmentFiles"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 26], "end": [49, 31], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment_files/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment_files/mod.rs:49`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2cf47d46aa8eece97bd0db04"></a>
## default

`function` · `buoyant_kernel::log_segment_files::LogSegmentFiles::default` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> LogSegmentFiles
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment_files/mod.rs#L49).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::log_segment_files::LogSegmentFiles", "path": "LogSegmentFiles"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 17], "end": [49, 24], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment_files/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment_files/mod.rs:49`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1debc1cd45ec0cd3b96435aa"></a>
## eq

`function` · `buoyant_kernel::log_segment_files::LogSegmentFiles::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &LogSegmentFiles) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment_files/mod.rs#L49).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::log_segment_files::LogSegmentFiles", "path": "LogSegmentFiles"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 33], "end": [49, 42], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment_files/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment_files/mod.rs:49`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e01d59b7fbae50828cb0d553"></a>
## fmt

`function` · `buoyant_kernel::log_segment_files::LogSegmentFiles::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment_files/mod.rs#L49).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::log_segment_files::LogSegmentFiles", "path": "LogSegmentFiles"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 10], "end": [49, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment_files/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment_files/mod.rs:49`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d0b70d96d9bdb80937d0afd"></a>
## latest_commit_file

`struct_field` · `buoyant_kernel::log_segment_files::LogSegmentFiles::latest_commit_file` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
latest_commit_file: Option<path::ParsedLogPath>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment_files/mod.rs#L56).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment_files/mod.rs:56`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3564bbcc792b0b41e999db24"></a>
## latest_crc_file

`struct_field` · `buoyant_kernel::log_segment_files::LogSegmentFiles::latest_crc_file` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
latest_crc_file: Option<path::ParsedLogPath>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment_files/mod.rs#L55).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment_files/mod.rs:55`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44d5a78cca1f10356b0abfed"></a>
## max_published_version

`struct_field` · `buoyant_kernel::log_segment_files::LogSegmentFiles::max_published_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
max_published_version: Option<Version>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment_files/mod.rs#L57).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment_files/mod.rs:57`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
