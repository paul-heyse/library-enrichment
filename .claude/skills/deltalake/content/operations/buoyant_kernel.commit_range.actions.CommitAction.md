# `buoyant_kernel::commit_range::actions::CommitAction`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.commit_range.actions.CommitAction.json).

<a id="op-1330eec83dea824eb882ec82"></a>
## CommitAction

`struct` · `buoyant_kernel::commit_range::actions::CommitAction` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct CommitAction
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/actions.rs#L48).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/actions.rs:48`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Per-commit handle returned by [`super::CommitRange::commits`](../operations/buoyant_kernel.commit_range.CommitRange.md#op-82385bf443cf1811dcd55fe8).

Carries the commit's version, timestamp, and the effective (extracted from this
commit overlaid onto the iterator's accumulated state) `Protocol` / `Metadata`.
Reading the commit's action batches is lazy and re-buildable via
[`Self::get_actions`](../operations/buoyant_kernel.commit_range.actions.CommitAction.md#op-c6fa11b4411635ac9f30de57), which issues a fresh JSON read on every call.

<a id="op-c6fa11b4411635ac9f30de57"></a>
## get_actions

`function` · `buoyant_kernel::commit_range::actions::CommitAction::get_actions` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_actions(&self, engine: &dyn Engine) -> DeltaResult<FileDataReadResultIterator>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/actions.rs#L220).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::commit_range::actions::CommitAction", "path": "CommitAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [227, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/actions.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/actions.rs:220`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Return an iterator over the commit's action batches projected to the
caller-requested `read_schema`.

Batches contain raw actions exactly as recorded in the commit JSON; no column-mapping
translation is applied.

<a id="op-6752a4cc5ac37fdba5257275"></a>
## timestamp

`function` · `buoyant_kernel::commit_range::actions::CommitAction::timestamp` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn timestamp(&self) -> i64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/actions.rs#L114).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::commit_range::actions::CommitAction", "path": "CommitAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [227, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/actions.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/actions.rs:114`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Commit timestamp in milliseconds since epoch.

For tables with in-commit timestamps enabled, this is the commit's
`commitInfo.inCommitTimestamp` (for versions at or after the enablement version);
otherwise it is the commit file's `last_modified` time. When the effective table
configuration cannot be determined (e.g. a snapshot-less range that has not yet observed
a `Metadata` action), the timestamp is best-effort: the in-commit timestamp if physically
present, else `last_modified`. The value is not guaranteed monotonic across the
enablement boundary or in such best-effort ranges.

<a id="op-2a1daee37d295b59d3123bf3"></a>
## version

`function` · `buoyant_kernel::commit_range::actions::CommitAction::version` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn version(&self) -> Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/actions.rs#L101).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::commit_range::actions::CommitAction", "path": "CommitAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [227, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/actions.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/actions.rs:101`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Commit version of this commit.

<a id="op-237ef13d89399bf7ae203dc0"></a>
## log_path

`struct_field` · `buoyant_kernel::commit_range::actions::CommitAction::log_path` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
log_path: path::ParsedLogPath
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/actions.rs#L50).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/actions.rs:50`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2648c3981dd1c8d62a3a0810"></a>
## metadata

`struct_field` · `buoyant_kernel::commit_range::actions::CommitAction::metadata` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
metadata: Option<actions::Metadata>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/actions.rs#L53).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/actions.rs:53`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd0604cd0b12ae576e591bce"></a>
## protocol

`struct_field` · `buoyant_kernel::commit_range::actions::CommitAction::protocol` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
protocol: Option<actions::Protocol>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/actions.rs#L52).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/actions.rs:52`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18834cd68f5745d5d3c90cad"></a>
## read_schema

`struct_field` · `buoyant_kernel::commit_range::actions::CommitAction::read_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
read_schema: schema::SchemaRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/actions.rs#L51).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/actions.rs:51`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f9391db99f95222bce01510"></a>
## table_root

`struct_field` · `buoyant_kernel::commit_range::actions::CommitAction::table_root` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
table_root: url::Url
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/actions.rs#L49).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/actions.rs:49`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e24b8116a92f1cff14c156c4"></a>
## timestamp

`struct_field` · `buoyant_kernel::commit_range::actions::CommitAction::timestamp` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
timestamp: i64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/actions.rs#L55).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/actions.rs:55`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Resolved commit timestamp (in-commit timestamp or file `last_modified`).
