# `buoyant_kernel::log_path::LogPath`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.log_path.LogPath.json).

<a id="op-9cfc97ab009a43d5f58bc93a"></a>
## LogPath

`struct` · `buoyant_kernel::log_path::LogPath` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct LogPath
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_path.rs#L15).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_path.rs:15`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A path to a valid delta log file. You can parse a given `FileMeta` into a `LogPath` using
[`LogPath::try_new`](../operations/buoyant_kernel.log_path.LogPath.md#op-17b967a5d7feb2857679f6b2).

Today, a `LogPath` is a file in the `_delta_log` directory of a Delta table; in the future,
this will expand to support providing inline data in the log path itself.

<a id="op-d6fb4dae6b5f3386cee3ab10"></a>
## clone

`function` · `buoyant_kernel::log_path::LogPath::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> LogPath
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_path.rs#L14).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::log_path::LogPath", "path": "LogPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [14, 17], "end": [14, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_path.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_path.rs:14`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8dd2011e551da90a7bbb12c"></a>
## eq

`function` · `buoyant_kernel::log_path::LogPath::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &LogPath) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_path.rs#L14).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::log_path::LogPath", "path": "LogPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [14, 24], "end": [14, 33], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_path.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_path.rs:14`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-452d8ee2bc53afdd242f4f47"></a>
## fmt

`function` · `buoyant_kernel::log_path::LogPath::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_path.rs#L14).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::log_path::LogPath", "path": "LogPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [14, 10], "end": [14, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_path.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_path.rs:14`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb02b3272dc86f8378433f69"></a>
## staged_commit

`function` · `buoyant_kernel::log_path::LogPath::staged_commit` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn staged_commit(table_root: Url, filename: &str, last_modified: i64, size: FileSize) -> DeltaResult<LogPath>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_path.rs#L41).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::log_path::LogPath", "path": "LogPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 1], "end": [69, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_path.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_path.rs:41`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new staged commit log path given the table root and filename and metadata. The
table_root must point to the root of the table and end with a '/'.

<a id="op-4720e1030dcdc77e68e81b8c"></a>
## staged_commit_url

`function` · `buoyant_kernel::log_path::LogPath::staged_commit_url` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn staged_commit_url(table_root: Url, filename: &str) -> DeltaResult<Url>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_path.rs#L58).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::log_path::LogPath", "path": "LogPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 1], "end": [69, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_path.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_path.rs:58`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create the URL for a staged commit file given the table root and filename. The table_root
must point to the root of the table and end with a '/'.

<a id="op-17b967a5d7feb2857679f6b2"></a>
## try_new

`function` · `buoyant_kernel::log_path::LogPath::try_new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_new(file_meta: FileMeta) -> DeltaResult<Self>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_path.rs#L26).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::log_path::LogPath", "path": "LogPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 1], "end": [69, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_path.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_path.rs:26`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Attempt to create a `LogPath` from `FileMeta`. This returns an error if the path isn't a
valid log path.

<a id="op-e442c4a9c52229e26f7d3846"></a>
## 0

`struct_field` · `buoyant_kernel::log_path::LogPath::0` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
0: path::ParsedLogPath
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_path.rs#L15).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_path.rs:15`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
