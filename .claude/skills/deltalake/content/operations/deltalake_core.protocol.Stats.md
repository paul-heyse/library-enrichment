# `deltalake_core::protocol::Stats`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.protocol.Stats.json).

<a id="op-606fde0629a98cf1abebf832"></a>
## Stats

`struct` · `deltalake_core::protocol::Stats` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct Stats
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L90).

Source: `crates/core/src/protocol/mod.rs:90`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Statistics associated with Add actions contained in the Delta log.

<a id="op-7127b34bd70add00be1445ab"></a>
## default

`function` · `deltalake_core::protocol::Stats::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> Stats
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L88).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::Stats", "path": "Stats"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 41], "end": [88, 48], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/protocol/mod.rs:88`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f254d67b10bd74cec5cda5e"></a>
## deserialize

`function` · `deltalake_core::protocol::Stats::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L88).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::Stats", "path": "Stats"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 21], "end": [88, 32], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/protocol/mod.rs:88`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-758017b23d54b2e8e2b3c8a9"></a>
## eq

`function` · `deltalake_core::protocol::Stats::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &Stats) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L88).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::Stats", "path": "Stats"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 50], "end": [88, 59], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/protocol/mod.rs:88`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc90cc31935f1aa1dd30a673"></a>
## fmt

`function` · `deltalake_core::protocol::Stats::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L88).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::Stats", "path": "Stats"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 34], "end": [88, 39], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/protocol/mod.rs:88`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c5a065c83fd807254fd7cf2"></a>
## max_values

`struct_field` · `deltalake_core::protocol::Stats::max_values` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
max_values: std::collections::HashMap<String, ColumnValueStat>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L98).

Source: `crates/core/src/protocol/mod.rs:98`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Contains a value larger than all values present in the file for all columns.

<a id="op-11d42e0ac41ee3a96e7fdc30"></a>
## min_values

`struct_field` · `deltalake_core::protocol::Stats::min_values` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
min_values: std::collections::HashMap<String, ColumnValueStat>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L96).

Source: `crates/core/src/protocol/mod.rs:96`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Contains a value smaller than all values present in the file for all columns.

<a id="op-b60a82f3456d997f903d98b6"></a>
## null_count

`struct_field` · `deltalake_core::protocol::Stats::null_count` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
null_count: std::collections::HashMap<String, ColumnCountStat>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L100).

Source: `crates/core/src/protocol/mod.rs:100`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The number of null values for all columns.

<a id="op-e3490a0e9f49c06e81ad5993"></a>
## num_records

`struct_field` · `deltalake_core::protocol::Stats::num_records` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_records: i64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L92).

Source: `crates/core/src/protocol/mod.rs:92`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of records in the file associated with the log action.

<a id="op-4367477fa7081f6b5dbf2b27"></a>
## serialize

`function` · `deltalake_core::protocol::Stats::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L88).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::Stats", "path": "Stats"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 10], "end": [88, 19], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/protocol/mod.rs:88`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
