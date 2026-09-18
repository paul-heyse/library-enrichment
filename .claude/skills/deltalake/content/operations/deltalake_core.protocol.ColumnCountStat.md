# `deltalake_core::protocol::ColumnCountStat`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.protocol.ColumnCountStat.json).

<a id="op-8fe1160b947a340f984446b5"></a>
## ColumnCountStat

`enum` · `deltalake_core::protocol::ColumnCountStat` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum ColumnCountStat
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L62).

Source: `crates/core/src/protocol/mod.rs:62`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Struct used to represent nullCount in add action statistics.

<a id="op-25f304af3ba39926dc343372"></a>
## Column

`variant` · `deltalake_core::protocol::ColumnCountStat::Column` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Column
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L64).

Source: `crates/core/src/protocol/mod.rs:64`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Composite HashMap representation of statistics.

<a id="op-c672fe5f7ecbb91c7943decb"></a>
## Value

`variant` · `deltalake_core::protocol::ColumnCountStat::Value` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Value
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L66).

Source: `crates/core/src/protocol/mod.rs:66`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Json representation of statistics.

<a id="op-fc08a8110e08e8d7e2ecfbcb"></a>
## as_column

`function` · `deltalake_core::protocol::ColumnCountStat::as_column` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn as_column(&self) -> Option<&HashMap<String, ColumnCountStat>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L71).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::ColumnCountStat", "path": "ColumnCountStat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [85, 2], "filename": "crates/core/src/protocol/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/protocol/mod.rs:71`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns the HashMap representation of the ColumnCountStat.

<a id="op-d0e24c4b311c3e21b1af44b3"></a>
## as_value

`function` · `deltalake_core::protocol::ColumnCountStat::as_value` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn as_value(&self) -> Option<i64>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L79).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::ColumnCountStat", "path": "ColumnCountStat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [85, 2], "filename": "crates/core/src/protocol/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/protocol/mod.rs:79`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns the serde_json representation of the ColumnCountStat.

<a id="op-3947c5cc8d1ca6dfd1e4402f"></a>
## deserialize

`function` · `deltalake_core::protocol::ColumnCountStat::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L60).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::ColumnCountStat", "path": "ColumnCountStat"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 21], "end": [60, 32], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/protocol/mod.rs:60`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-316e7ab4f3db54768577397d"></a>
## eq

`function` · `deltalake_core::protocol::ColumnCountStat::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &ColumnCountStat) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L60).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::ColumnCountStat", "path": "ColumnCountStat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 41], "end": [60, 50], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/protocol/mod.rs:60`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf815381945d9f743a15bde1"></a>
## fmt

`function` · `deltalake_core::protocol::ColumnCountStat::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L60).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::ColumnCountStat", "path": "ColumnCountStat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 34], "end": [60, 39], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/protocol/mod.rs:60`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef897c9d9ffe687fdfaf518a"></a>
## serialize

`function` · `deltalake_core::protocol::ColumnCountStat::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L60).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::ColumnCountStat", "path": "ColumnCountStat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 10], "end": [60, 19], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/protocol/mod.rs:60`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
