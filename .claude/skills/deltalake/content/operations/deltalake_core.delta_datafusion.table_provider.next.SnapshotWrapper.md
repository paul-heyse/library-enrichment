# `deltalake_core::delta_datafusion::table_provider::next::SnapshotWrapper`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.table_provider.next.SnapshotWrapper.json).

<a id="op-d0b7682a953b910c30b6d5ba"></a>
## SnapshotWrapper

`enum` · `deltalake_core::delta_datafusion::table_provider::next::SnapshotWrapper` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
enum SnapshotWrapper
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L436).

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:436`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5acf4e2436b4d03629f57ac"></a>
## EagerSnapshot

`variant` · `deltalake_core::delta_datafusion::table_provider::next::SnapshotWrapper::EagerSnapshot` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
EagerSnapshot
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L438).

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:438`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a29b454d09400b5cae689324"></a>
## Snapshot

`variant` · `deltalake_core::delta_datafusion::table_provider::next::SnapshotWrapper::Snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
Snapshot
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L437).

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:437`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-405c73f7f46644dd1dde4be7"></a>
## clone

`function` · `deltalake_core::delta_datafusion::table_provider::next::SnapshotWrapper::clone` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> SnapshotWrapper
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L435).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::SnapshotWrapper", "path": "SnapshotWrapper"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [435, 10], "end": [435, 15], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:435`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb7c9d5b17ce294a87f3cd9f"></a>
## deserialize

`function` · `deltalake_core::delta_datafusion::table_provider::next::SnapshotWrapper::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L435).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::SnapshotWrapper", "path": "SnapshotWrapper"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [435, 35], "end": [435, 46], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:435`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22f81deb62b917fdc84f3b76"></a>
## fmt

`function` · `deltalake_core::delta_datafusion::table_provider::next::SnapshotWrapper::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L435).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::SnapshotWrapper", "path": "SnapshotWrapper"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [435, 17], "end": [435, 22], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:435`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1774cb8d5b790097c2029212"></a>
## from

`function` · `deltalake_core::delta_datafusion::table_provider::next::SnapshotWrapper::from` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn from(snap: Arc<Snapshot>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L442).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::SnapshotWrapper", "path": "SnapshotWrapper"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [441, 1], "end": [445, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::Snapshot", "path": "Snapshot"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:442`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c4d2f50f4ddc0d43b81ba00"></a>
## from

`function` · `deltalake_core::delta_datafusion::table_provider::next::SnapshotWrapper::from` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn from(esnap: Arc<EagerSnapshot>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L454).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::SnapshotWrapper", "path": "SnapshotWrapper"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [453, 1], "end": [457, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "EagerSnapshot"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:454`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80ef878b8c3029e49b170a3c"></a>
## from

`function` · `deltalake_core::delta_datafusion::table_provider::next::SnapshotWrapper::from` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn from(snap: Snapshot) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L448).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::SnapshotWrapper", "path": "SnapshotWrapper"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [447, 1], "end": [451, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::Snapshot", "path": "Snapshot"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:448`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4031143895e9bc19c37b70b"></a>
## from

`function` · `deltalake_core::delta_datafusion::table_provider::next::SnapshotWrapper::from` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn from(esnap: EagerSnapshot) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L460).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::SnapshotWrapper", "path": "SnapshotWrapper"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [459, 1], "end": [463, 2], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::snapshot::EagerSnapshot", "path": "EagerSnapshot"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:460`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5fb8241559fe66de76a0d8f6"></a>
## serialize

`function` · `deltalake_core::delta_datafusion::table_provider::next::SnapshotWrapper::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L435).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::SnapshotWrapper", "path": "SnapshotWrapper"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [435, 24], "end": [435, 33], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:435`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
