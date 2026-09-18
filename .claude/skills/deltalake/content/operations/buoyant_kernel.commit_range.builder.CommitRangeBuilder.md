# `buoyant_kernel::commit_range::builder::CommitRangeBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.commit_range.builder.CommitRangeBuilder.json).

<a id="op-9307a39a35eaca1fd7ec407f"></a>
## CommitRangeBuilder

`struct` · `buoyant_kernel::commit_range::builder::CommitRangeBuilder` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct CommitRangeBuilder
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/builder.rs#L17).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/builder.rs:17`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Builder for a [`CommitRange`](../operations/buoyant_kernel.commit_range.CommitRange.md#op-7308777d4b0c84353f5470a4).

Created via [`CommitRange::builder_for`](../operations/buoyant_kernel.commit_range.CommitRange.md#op-d558fffe48e24faa5c16db01) (path-based) or
[`CommitRange::builder_from`](../operations/buoyant_kernel.commit_range.CommitRange.md#op-b85017bbbf1059bb4f0f92d0) (snapshot-based). Supports configuring an end version
and the commit ordering. [`Self::build`](../operations/buoyant_kernel.commit_range.builder.CommitRangeBuilder.md#op-d5d95e702f615e87e68c4a02) performs delta-log listing and contiguity
validation.

<a id="op-d5d95e702f615e87e68c4a02"></a>
## build

`function` · `buoyant_kernel::commit_range::builder::CommitRangeBuilder::build` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn build(&self, engine: &dyn Engine) -> DeltaResult<CommitRange>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/builder.rs#L66).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::commit_range::builder::CommitRangeBuilder", "path": "CommitRangeBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [124, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/builder.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/builder.rs:66`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

List `_delta_log/`, validate contiguity, and produce a [`CommitRange`](../operations/buoyant_kernel.commit_range.CommitRange.md#op-7308777d4b0c84353f5470a4). Performs
filesystem listing but no JSON reads.

Returns an error if the resolved version range is invalid (start > end), the
listed commits are non-contiguous, or the requested start version is not present
on the filesystem.

<a id="op-dd728e72d06ec51d17d6ac81"></a>
## with_end_version

`function` · `buoyant_kernel::commit_range::builder::CommitRangeBuilder::with_end_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_end_version(self, end_version: Version) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/builder.rs#L48).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::commit_range::builder::CommitRangeBuilder", "path": "CommitRangeBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [124, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/builder.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/builder.rs:48`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Pin the end of the range. Without this, the range extends to the latest committed
version observed at build time.

<a id="op-2dd19700492dff2f7094f14a"></a>
## with_ordering

`function` · `buoyant_kernel::commit_range::builder::CommitRangeBuilder::with_ordering` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_ordering(self, commit_ordering: CommitOrdering) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/builder.rs#L55).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::commit_range::builder::CommitRangeBuilder", "path": "CommitRangeBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [124, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/builder.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/builder.rs:55`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Set the order in which [`CommitRange::commits`](../operations/buoyant_kernel.commit_range.CommitRange.md#op-82385bf443cf1811dcd55fe8) yields commits. Defaults to
[`CommitOrdering::AscendingOrder`](../operations/buoyant_kernel.commit_range.builder.CommitOrdering.md#op-8b295ffda3eae152a8fae3b3).

<a id="op-e354f3df66929bdab5165fbc"></a>
## commit_ordering

`struct_field` · `buoyant_kernel::commit_range::builder::CommitRangeBuilder::commit_ordering` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
commit_ordering: CommitOrdering
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/builder.rs#L22).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/builder.rs:22`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a3d14d8cf5e2f2c3de87997"></a>
## end_version

`struct_field` · `buoyant_kernel::commit_range::builder::CommitRangeBuilder::end_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
end_version: Option<Version>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/builder.rs#L20).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/builder.rs:20`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa599e724a78f3ed160101f7"></a>
## snapshot

`struct_field` · `buoyant_kernel::commit_range::builder::CommitRangeBuilder::snapshot` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
snapshot: Option<snapshot::SnapshotRef>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/builder.rs#L21).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/builder.rs:21`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1bc3921076521dcc09a60214"></a>
## start_version

`struct_field` · `buoyant_kernel::commit_range::builder::CommitRangeBuilder::start_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
start_version: Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/builder.rs#L19).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/builder.rs:19`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f4f9a1621c67eae9277b08b"></a>
## table_root

`struct_field` · `buoyant_kernel::commit_range::builder::CommitRangeBuilder::table_root` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
table_root: String
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/builder.rs#L18).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/builder.rs:18`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
