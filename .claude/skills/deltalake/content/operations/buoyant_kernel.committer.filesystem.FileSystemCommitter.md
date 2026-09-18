# `buoyant_kernel::committer::filesystem::FileSystemCommitter`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.committer.filesystem.FileSystemCommitter.json).

<a id="op-c9d7ce0544dbb4d345b6296d"></a>
## FileSystemCommitter

`struct` · `buoyant_kernel::committer::filesystem::FileSystemCommitter` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct FileSystemCommitter
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/filesystem.rs#L16).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/filesystem.rs:16`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The `FileSystemCommitter` is an internal implementation of the `Committer` trait which
commits to a file system directly via `Engine::json_handler().write_json_file` for
non-catalog-managed tables.

SAFETY: it is _incorrect_ to use this committer for catalog-managed tables.

<a id="op-ad0a30afd61082119cf0087f"></a>
## commit

`function` · `buoyant_kernel::committer::filesystem::FileSystemCommitter::commit` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn commit(&self, engine: &dyn Engine, actions: DeltaResultIterator<'_, FilteredEngineData>, commit_metadata: CommitMetadata) -> DeltaResult<CommitResponse>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/filesystem.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::filesystem::FileSystemCommitter", "path": "FileSystemCommitter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 1], "end": [84, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/filesystem.rs"}, "trait": {"args": null, "id": "buoyant_kernel::committer::Committer", "path": "Committer"}, "trait_path": "buoyant_kernel::committer::Committer"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/filesystem.rs:31`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b30cf99919373b4d89b6e222"></a>
## default

`function` · `buoyant_kernel::committer::filesystem::FileSystemCommitter::default` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> FileSystemCommitter
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/filesystem.rs#L15).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::filesystem::FileSystemCommitter", "path": "FileSystemCommitter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [15, 17], "end": [15, 24], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/filesystem.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/filesystem.rs:15`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc3177cca02e686de7f52a6a"></a>
## fmt

`function` · `buoyant_kernel::committer::filesystem::FileSystemCommitter::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/filesystem.rs#L15).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::filesystem::FileSystemCommitter", "path": "FileSystemCommitter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [15, 10], "end": [15, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/filesystem.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/filesystem.rs:15`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6f9adc1445a5e729132318f9"></a>
## is_catalog_committer

`function` · `buoyant_kernel::committer::filesystem::FileSystemCommitter::is_catalog_committer` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn is_catalog_committer(&self) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/filesystem.rs#L70).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::filesystem::FileSystemCommitter", "path": "FileSystemCommitter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 1], "end": [84, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/filesystem.rs"}, "trait": {"args": null, "id": "buoyant_kernel::committer::Committer", "path": "Committer"}, "trait_path": "buoyant_kernel::committer::Committer"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/filesystem.rs:70`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba64438886df0c3400f2c08e"></a>
## new

`function` · `buoyant_kernel::committer::filesystem::FileSystemCommitter::new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new() -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/filesystem.rs#L19).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::filesystem::FileSystemCommitter", "path": "FileSystemCommitter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [18, 1], "end": [22, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/filesystem.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/filesystem.rs:19`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-917ee3cf2a104157a58fb4ab"></a>
## publish

`function` · `buoyant_kernel::committer::filesystem::FileSystemCommitter::publish` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn publish(&self, _engine: &dyn Engine, publish_metadata: PublishMetadata) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/filesystem.rs#L76).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::filesystem::FileSystemCommitter", "path": "FileSystemCommitter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 1], "end": [84, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/filesystem.rs"}, "trait": {"args": null, "id": "buoyant_kernel::committer::Committer", "path": "Committer"}, "trait_path": "buoyant_kernel::committer::Committer"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/filesystem.rs:76`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The FileSystemCommitter should never be invoked to publish catalog commits. If it is,
something has gone wrong upstream.
