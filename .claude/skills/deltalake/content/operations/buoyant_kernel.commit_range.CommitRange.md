# `buoyant_kernel::commit_range::CommitRange`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.commit_range.CommitRange.json).

<a id="op-7308777d4b0c84353f5470a4"></a>
## CommitRange

`struct` · `buoyant_kernel::commit_range::CommitRange` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct CommitRange
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/mod.rs#L75).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/mod.rs:75`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A contiguous range of Delta commits, holding resolved `[start_version, end_version]` bounds
plus the materialized commit-file pointers in `commit_files`.

The pointer order matches the [`CommitOrdering`](../operations/buoyant_kernel.commit_range.builder.CommitOrdering.md#op-406de1722baf5b7d3ee90ca9) requested at build time (ascending by
default; reversed if [`CommitOrdering::DescendingOrder`](../operations/buoyant_kernel.commit_range.builder.CommitOrdering.md#op-75145b6f3d62cc4e2e0418a5)). Reading the underlying actions is
lazy via [`CommitRange::commits`](../operations/buoyant_kernel.commit_range.CommitRange.md#op-82385bf443cf1811dcd55fe8).

<a id="op-d558fffe48e24faa5c16db01"></a>
## builder_for

`function` · `buoyant_kernel::commit_range::CommitRange::builder_for` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn builder_for(table_root: impl AsRef<str>, start_version: Version) -> CommitRangeBuilder
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/mod.rs#L85).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::commit_range::CommitRange", "path": "CommitRange"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [181, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/mod.rs:85`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Begin building a [`CommitRange`](../operations/buoyant_kernel.commit_range.CommitRange.md#op-7308777d4b0c84353f5470a4) rooted at `table_root`, starting at `start_version`.

<a id="op-b85017bbbf1059bb4f0f92d0"></a>
## builder_from

`function` · `buoyant_kernel::commit_range::CommitRange::builder_from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn builder_from(snapshot: SnapshotRef, start_version: Version) -> CommitRangeBuilder
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/mod.rs#L94).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::commit_range::CommitRange", "path": "CommitRange"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [181, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/mod.rs:94`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Begin building a [`CommitRange`](../operations/buoyant_kernel.commit_range.CommitRange.md#op-7308777d4b0c84353f5470a4) derived from an existing snapshot, starting at
`start_version`.

The snapshot's table root anchors the range, and the snapshot's `LogSegment` is
reused to enumerate commits, avoiding an extra delta-log listing.

<a id="op-82385bf443cf1811dcd55fe8"></a>
## commits

`function` · `buoyant_kernel::commit_range::CommitRange::commits` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn commits(&self, engine: Arc<dyn Engine>, start_snapshot: Option<SnapshotRef>, actions: &[DeltaAction]) -> DeltaResult<impl Iterator<Item = DeltaResult<CommitAction>> + Send>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/mod.rs#L130).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::commit_range::CommitRange", "path": "CommitRange"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [181, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/mod.rs:130`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Iterator over the commits in the range, yielding one [`CommitAction`](../operations/buoyant_kernel.commit_range.actions.CommitAction.md#op-1330eec83dea824eb882ec82) per commit.

- `engine`: performs the per-commit JSON reads.
- `start_snapshot`: optional snapshot whose version anchors the range and seeds
  protocol/metadata validation; `None` validates from the commits alone.
- `actions`: the action kinds to project into each commit's read schema.

Actions are returned raw, exactly as recorded in the commit JSON; no column-mapping
translation is applied.

This is operation-agnostic: requesting [`DeltaAction::Cdc`](../operations/buoyant_kernel.commit_range.actions.DeltaAction.md#op-46d91699ede50509e4df89d4) returns the raw `cdc` action
records and does NOT impose change-data-feed support (`Operation::Cdf`). It does not
materialize a change data feed.

Returns `Err` if `actions` is empty or contains duplicate kinds, or if `start_snapshot`
belongs to a different table, its version does not match the range anchor, or its table
does not support scanning.

<a id="op-7540c6258f735bb27cb4b3d2"></a>
## end_version

`function` · `buoyant_kernel::commit_range::CommitRange::end_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn end_version(&self) -> Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/mod.rs#L104).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::commit_range::CommitRange", "path": "CommitRange"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [181, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/mod.rs:104`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Last version (inclusive) in the range.

<a id="op-f049a63be61a922bdf96e192"></a>
## fmt

`function` · `buoyant_kernel::commit_range::CommitRange::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/mod.rs#L74).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::commit_range::CommitRange", "path": "CommitRange"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 10], "end": [74, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/mod.rs:74`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32fc69cb41fbc4abed7429fb"></a>
## start_version

`function` · `buoyant_kernel::commit_range::CommitRange::start_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn start_version(&self) -> Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/mod.rs#L99).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::commit_range::CommitRange", "path": "CommitRange"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [181, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/mod.rs:99`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

First version (inclusive) in the range.

<a id="op-c37b1a47f20ab14a48ac22b1"></a>
## table_root

`function` · `buoyant_kernel::commit_range::CommitRange::table_root` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn table_root(&self) -> &Url
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/mod.rs#L109).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::commit_range::CommitRange", "path": "CommitRange"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [181, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/mod.rs:109`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The table root URL this range was built from.

<a id="op-45b5b05be741a1148387421b"></a>
## commit_files

`struct_field` · `buoyant_kernel::commit_range::CommitRange::commit_files` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
commit_files: Vec<path::ParsedLogPath>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/mod.rs#L77).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/mod.rs:77`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9412f9cc912085d8ce28beaf"></a>
## commit_ordering

`struct_field` · `buoyant_kernel::commit_range::CommitRange::commit_ordering` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
commit_ordering: CommitOrdering
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/mod.rs#L80).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/mod.rs:80`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a031449bffda313e6d4cacef"></a>
## end_version

`struct_field` · `buoyant_kernel::commit_range::CommitRange::end_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
end_version: Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/mod.rs#L79).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/mod.rs:79`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed563c9cf6f70970ba917361"></a>
## start_version

`struct_field` · `buoyant_kernel::commit_range::CommitRange::start_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
start_version: Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/mod.rs#L78).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/mod.rs:78`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-921d235d8e25fa2fcb796fbb"></a>
## table_root

`struct_field` · `buoyant_kernel::commit_range::CommitRange::table_root` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
table_root: url::Url
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/mod.rs#L76).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/mod.rs:76`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
