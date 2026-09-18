# `deltalake_core::kernel::snapshot::iterators::tombstones::TombstoneView`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.snapshot.iterators.tombstones.TombstoneView.json).

<a id="op-6266c9e7448abc20956a3c00"></a>
## TombstoneView

`struct` · `deltalake_core::kernel::snapshot::iterators::tombstones::TombstoneView` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct TombstoneView
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/iterators/tombstones.rs#L17).

Source: `crates/core/src/kernel/snapshot/iterators/tombstones.rs:17`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A lightweight, cloneable view over a single tombstone (`Remove` action) row.

Rather than materializing a `Remove` struct, this borrows into the backing
[`RecordBatch`] and decodes individual fields on demand.

Unresolved upstream links (retained, not inferred): ``RecordBatch``.

<a id="op-aeb96684b779bbe51f247841"></a>
## clone

`function` · `deltalake_core::kernel::snapshot::iterators::tombstones::TombstoneView::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> TombstoneView
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/iterators/tombstones.rs#L16).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::iterators::tombstones::TombstoneView", "path": "TombstoneView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [16, 10], "end": [16, 15], "filename": "crates/core/src/kernel/snapshot/iterators/tombstones.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/kernel/snapshot/iterators/tombstones.rs:16`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca0554b51dd8f8216dd5867b"></a>
## data_change

`function` · `deltalake_core::kernel::snapshot::iterators::tombstones::TombstoneView::data_change` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn data_change(&self) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/iterators/tombstones.rs#L52).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::iterators::tombstones::TombstoneView", "path": "TombstoneView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 1], "end": [74, 2], "filename": "crates/core/src/kernel/snapshot/iterators/tombstones.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/iterators/tombstones.rs:52`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns whether removing this file represents a data change (vs. a compaction-style rewrite).

<a id="op-3b1b9e6dce82ecccf8733daa"></a>
## deletion_timestamp

`function` · `deltalake_core::kernel::snapshot::iterators::tombstones::TombstoneView::deletion_timestamp` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deletion_timestamp(&self) -> Option<i64>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/iterators/tombstones.rs#L38).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::iterators::tombstones::TombstoneView", "path": "TombstoneView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 1], "end": [74, 2], "filename": "crates/core/src/kernel/snapshot/iterators/tombstones.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/iterators/tombstones.rs:38`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns the deletion timestamp (milliseconds since epoch), if recorded.

<a id="op-836249faabf7bf85650b7e84"></a>
## path

`function` · `deltalake_core::kernel::snapshot::iterators::tombstones::TombstoneView::path` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn path(&self) -> Cow<'_, str>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/iterators/tombstones.rs#L29).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::iterators::tombstones::TombstoneView", "path": "TombstoneView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 1], "end": [74, 2], "filename": "crates/core/src/kernel/snapshot/iterators/tombstones.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/iterators/tombstones.rs:29`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns the file path with URL decoding applied.

<a id="op-042ed282c1d404dc79949b20"></a>
## size

`function` · `deltalake_core::kernel::snapshot::iterators::tombstones::TombstoneView::size` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn size(&self) -> Option<i64>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/iterators/tombstones.rs#L66).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::iterators::tombstones::TombstoneView", "path": "TombstoneView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 1], "end": [74, 2], "filename": "crates/core/src/kernel/snapshot/iterators/tombstones.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/iterators/tombstones.rs:66`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns the size of the removed file in bytes, if recorded.

<a id="op-b526452b339b0a9d0c0b1cfe"></a>
## data

`struct_field` · `deltalake_core::kernel::snapshot::iterators::tombstones::TombstoneView::data` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
data: arrow::array::RecordBatch
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/iterators/tombstones.rs#L18).

Source: `crates/core/src/kernel/snapshot/iterators/tombstones.rs:18`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e049e437f320fa1af8e3815"></a>
## index

`struct_field` · `deltalake_core::kernel::snapshot::iterators::tombstones::TombstoneView::index` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
index: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/iterators/tombstones.rs#L19).

Source: `crates/core/src/kernel/snapshot/iterators/tombstones.rs:19`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
