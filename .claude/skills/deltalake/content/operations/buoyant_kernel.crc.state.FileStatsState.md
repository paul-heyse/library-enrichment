# `buoyant_kernel::crc::state::FileStatsState`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.crc.state.FileStatsState.json).

<a id="op-fa49cf56a10f2e93f21b3b8a"></a>
## FileStatsState

`enum` · `buoyant_kernel::crc::state::FileStatsState` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum FileStatsState
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/state.rs#L26).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs:26`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The state of file statistics for a CRC.

# State transitions during `Crc::apply`

| Current       | + safe op     | + unsafe op or missing remove.size |
|---------------|---------------|------------------------------------|
| Complete      | Complete      | Indeterminate                      |
| Indeterminate | Indeterminate | Indeterminate                      |

<a id="op-41986e83f36889560d6d4947"></a>
## Complete

`variant` · `buoyant_kernel::crc::state::FileStatsState::Complete` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Complete
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/state.rs#L31).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs:31`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

File stats are known-correct absolute totals. The carried [`FileStats`](../operations/buoyant_kernel.crc.file_stats.FileStats.md#op-0ae222f09a8ef0e02c9e6fa6)'s
`file_size_histogram` is optional: `Some` when the CRC source had a histogram (or full
replay produced one), `None` when the source lacked one. Safe to write to disk (with or
without histogram).

<a id="op-8d68b6ac1e47495676458ff4"></a>
## Indeterminate

`variant` · `buoyant_kernel::crc::state::FileStatsState::Indeterminate` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Indeterminate
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/state.rs#L36).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs:36`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

File stats cannot be determined incrementally. Reasons include a non-incremental
operation (like `ANALYZE STATS`) that re-adds files without corresponding removes,
or a remove action with a missing `size` field. A full add/remove reconciliation pass
can recover `Complete`.

<a id="op-a8527eeee3456cd45b3606d0"></a>
## clone

`function` · `buoyant_kernel::crc::state::FileStatsState::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> FileStatsState
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/state.rs#L25).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::state::FileStatsState", "path": "FileStatsState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 17], "end": [25, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs:25`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9fb391a7476e0dfa31b149ee"></a>
## default

`function` · `buoyant_kernel::crc::state::FileStatsState::default` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/state.rs#L66).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::state::FileStatsState", "path": "FileStatsState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [69, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs:66`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-76258c165aa9cd2f6d9faa3f"></a>
## eq

`function` · `buoyant_kernel::crc::state::FileStatsState::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &FileStatsState) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/state.rs#L25).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::state::FileStatsState", "path": "FileStatsState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 24], "end": [25, 33], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs:25`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0ca8275f5b0cbf8c8dfbebd"></a>
## file_stats

`function` · `buoyant_kernel::crc::state::FileStatsState::file_stats` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn file_stats(&self) -> Option<&FileStats>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/state.rs#L41).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::state::FileStatsState", "path": "FileStatsState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [61, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs:41`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns absolute file stats only when `Complete`. Returns `None` for `Indeterminate`.

<a id="op-34d16cc6726f1a8072a110f7"></a>
## fmt

`function` · `buoyant_kernel::crc::state::FileStatsState::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/state.rs#L25).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::state::FileStatsState", "path": "FileStatsState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 10], "end": [25, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs:25`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-963436c71c6186b5adaab834"></a>
## is_complete

`function` · `buoyant_kernel::crc::state::FileStatsState::is_complete` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn is_complete(&self) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/state.rs#L51).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::state::FileStatsState", "path": "FileStatsState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [61, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs:51`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns `true` if file stats are known-correct absolute totals. Also gates whether
the CRC is safe to write to disk: only `Complete` CRCs have well-defined on-disk
representations.
