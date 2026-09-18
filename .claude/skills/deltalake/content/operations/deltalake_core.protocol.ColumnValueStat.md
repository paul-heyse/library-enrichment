# `deltalake_core::protocol::ColumnValueStat`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.protocol.ColumnValueStat.json).

<a id="op-ee05496ed14e81374fc27611"></a>
## ColumnValueStat

`enum` · `deltalake_core::protocol::ColumnValueStat` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum ColumnValueStat
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L34).

Source: `crates/core/src/protocol/mod.rs:34`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Struct used to represent minValues and maxValues in add action statistics.

<a id="op-469f01b36900d80afbef1806"></a>
## Column

`variant` · `deltalake_core::protocol::ColumnValueStat::Column` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Column
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L36).

Source: `crates/core/src/protocol/mod.rs:36`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Composite HashMap representation of statistics.

<a id="op-4fdcb9505a263409e1c96257"></a>
## Value

`variant` · `deltalake_core::protocol::ColumnValueStat::Value` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Value
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L38).

Source: `crates/core/src/protocol/mod.rs:38`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Json representation of statistics.

<a id="op-28ed53d558c2a1142285eae4"></a>
## as_column

`function` · `deltalake_core::protocol::ColumnValueStat::as_column` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn as_column(&self) -> Option<&HashMap<String, ColumnValueStat>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L43).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::ColumnValueStat", "path": "ColumnValueStat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [57, 2], "filename": "crates/core/src/protocol/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/protocol/mod.rs:43`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns the HashMap representation of the ColumnValueStat.

<a id="op-3b561dbd0f9ada2da330912e"></a>
## as_value

`function` · `deltalake_core::protocol::ColumnValueStat::as_value` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn as_value(&self) -> Option<&Value>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L51).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::ColumnValueStat", "path": "ColumnValueStat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [57, 2], "filename": "crates/core/src/protocol/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/protocol/mod.rs:51`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns the serde_json representation of the ColumnValueStat.

<a id="op-5f2e48bdfaa7dc4046307636"></a>
## deserialize

`function` · `deltalake_core::protocol::ColumnValueStat::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L32).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::ColumnValueStat", "path": "ColumnValueStat"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 21], "end": [32, 32], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/protocol/mod.rs:32`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f53177612baf6ad490753c10"></a>
## eq

`function` · `deltalake_core::protocol::ColumnValueStat::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &ColumnValueStat) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L32).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::ColumnValueStat", "path": "ColumnValueStat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 41], "end": [32, 50], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/protocol/mod.rs:32`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab6e81266ebcf983424e56c0"></a>
## fmt

`function` · `deltalake_core::protocol::ColumnValueStat::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L32).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::ColumnValueStat", "path": "ColumnValueStat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 34], "end": [32, 39], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/protocol/mod.rs:32`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6099f1132c6636fac4be248e"></a>
## serialize

`function` · `deltalake_core::protocol::ColumnValueStat::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L32).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::ColumnValueStat", "path": "ColumnValueStat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 10], "end": [32, 19], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/protocol/mod.rs:32`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
