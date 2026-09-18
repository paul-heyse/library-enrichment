# `buoyant_kernel::history_manager::error::LogHistoryError`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.history_manager.error.LogHistoryError.json).

<a id="op-4cdbaf4e3692f236131c959f"></a>
## LogHistoryError

`enum` · `buoyant_kernel::history_manager::error::LogHistoryError` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum LogHistoryError
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/error.rs#L41).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs:41`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Represents errors that can occur when converting commit timestamps to versions.

<a id="op-02a6d6d959b277e9030bcdab"></a>
## EmptyTimestampRange

`variant` · `buoyant_kernel::history_manager::error::LogHistoryError::EmptyTimestampRange` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
EmptyTimestampRange
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/error.rs#L63).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs:63`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The timestamp range contains no commits - the entire range falls between two adjacent
versions.

<a id="op-8c14377c5095e8c6f43671a4"></a>
## Internal

`variant` · `buoyant_kernel::history_manager::error::LogHistoryError::Internal` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Internal
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/error.rs#L97).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs:97`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

An internal error occurred during timestamp conversion.

<a id="op-daccd834fdc4daa2e9470543"></a>
## InvalidTimestampRange

`variant` · `buoyant_kernel::history_manager::error::LogHistoryError::InvalidTimestampRange` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
InvalidTimestampRange
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/error.rs#L50).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs:50`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The provided timestamp range is invalid (start > end).

<a id="op-2a7f0eaf1a66b3e6bdd40163"></a>
## NoCommitsFound

`variant` · `buoyant_kernel::history_manager::error::LogHistoryError::NoCommitsFound` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
NoCommitsFound
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/error.rs#L44).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs:44`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No commit files were found in the log directory.

<a id="op-2a2a6bd69549c86d59a54763"></a>
## NoRecreatableCommit

`variant` · `buoyant_kernel::history_manager::error::LogHistoryError::NoRecreatableCommit` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
NoRecreatableCommit
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/error.rs#L91).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs:91`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Commit files exist in the log but the table cannot be reconstructed: commit version 0
is missing and no complete checkpoint is present to anchor the surviving commits.

<a id="op-d826f69b327b93b6ee4b7767"></a>
## TimestampOutOfRange

`variant` · `buoyant_kernel::history_manager::error::LogHistoryError::TimestampOutOfRange` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
TimestampOutOfRange
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/error.rs#L74).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs:74`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The timestamp is outside the range of available commits.

<a id="op-0c62507d50efe113b923fb9e"></a>
## fmt

`function` · `buoyant_kernel::history_manager::error::LogHistoryError::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/error.rs#L40).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::history_manager::error::LogHistoryError", "path": "LogHistoryError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 17], "end": [40, 33], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs:40`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6262b255d555eae9ed4cd7a1"></a>
## fmt

`function` · `buoyant_kernel::history_manager::error::LogHistoryError::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/error.rs#L40).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::history_manager::error::LogHistoryError", "path": "LogHistoryError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 10], "end": [40, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs:40`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3bbc5e8e07606bdb1238c8e"></a>
## source

`function` · `buoyant_kernel::history_manager::error::LogHistoryError::source` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private20::Error + 'static>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/error.rs#L40).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::history_manager::error::LogHistoryError", "path": "LogHistoryError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 17], "end": [40, 33], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs"}, "trait": {"args": null, "id": "core::error::Error", "path": "Error"}, "trait_path": "core::error::Error"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs:40`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
