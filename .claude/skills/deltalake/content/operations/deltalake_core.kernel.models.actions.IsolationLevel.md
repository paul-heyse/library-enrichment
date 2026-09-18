# `deltalake_core::kernel::models::actions::IsolationLevel`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.models.actions.IsolationLevel.json).

<a id="op-8614ec085a86390f67f1540e"></a>
## IsolationLevel

`enum` · `deltalake_core::kernel::models::actions::IsolationLevel` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum IsolationLevel
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1202).

Source: `crates/core/src/kernel/models/actions.rs:1202`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The isolation level applied during transaction

<a id="op-3a53e2753d1c4ee924d8965a"></a>
## Err

`assoc_type` · `deltalake_core::kernel::models::actions::IsolationLevel::Err` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Err = Error
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1238).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::IsolationLevel", "path": "IsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1237, 1], "end": [1248, 2], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `crates/core/src/kernel/models/actions.rs:1238`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68063c193225a63b79704925"></a>
## Serializable

`variant` · `deltalake_core::kernel::models::actions::IsolationLevel::Serializable` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Serializable
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1209).

Source: `crates/core/src/kernel/models/actions.rs:1209`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The strongest isolation level. It ensures that committed write operations
and all reads are Serializable. Operations are allowed as long as there
exists a serial sequence of executing them one-at-a-time that generates
the same outcome as that seen in the table. For the write operations,
the serial sequence is exactly the same as that seen in the table’s history.

<a id="op-3fb3479fbdad32858e0d9609"></a>
## SnapshotIsolation

`variant` · `deltalake_core::kernel::models::actions::IsolationLevel::SnapshotIsolation` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
SnapshotIsolation
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1221).

Source: `crates/core/src/kernel/models/actions.rs:1221`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

SnapshotIsolation is a guarantee that all reads made in a transaction will see a consistent
snapshot of the database (in practice it reads the last committed values that existed at the
time it started), and the transaction itself will successfully commit only if no updates
it has made conflict with any concurrent updates made since that snapshot.

<a id="op-1ec7cf35dd592cad9d566765"></a>
## WriteSerializable

`variant` · `deltalake_core::kernel::models::actions::IsolationLevel::WriteSerializable` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
WriteSerializable
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1215).

Source: `crates/core/src/kernel/models/actions.rs:1215`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A weaker isolation level than Serializable. It ensures only that the write
operations (that is, not reads) are serializable. However, this is still stronger
than Snapshot isolation. WriteSerializable is the default isolation level because
it provides great balance of data consistency and availability for most common operations.

<a id="op-e0b3bb50102ab5360d6b6356"></a>
## as_ref

`function` · `deltalake_core::kernel::models::actions::IsolationLevel::as_ref` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn as_ref(&self) -> &str
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1228).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::IsolationLevel", "path": "IsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1227, 1], "end": [1235, 2], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "str"}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}, "trait_path": "core::convert::AsRef"}`

Source: `crates/core/src/kernel/models/actions.rs:1228`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9bc8cf41a98cc2a393184cf0"></a>
## clone

`function` · `deltalake_core::kernel::models::actions::IsolationLevel::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> IsolationLevel
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1199).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::IsolationLevel", "path": "IsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1199, 47], "end": [1199, 52], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/kernel/models/actions.rs:1199`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e1a853350dea4cf72637fcf"></a>
## default

`function` · `deltalake_core::kernel::models::actions::IsolationLevel::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> IsolationLevel
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1201).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::IsolationLevel", "path": "IsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1201, 10], "end": [1201, 17], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/kernel/models/actions.rs:1201`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22f79aa6e36d914b4127c684"></a>
## deserialize

`function` · `deltalake_core::kernel::models::actions::IsolationLevel::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1199).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::IsolationLevel", "path": "IsolationLevel"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1199, 21], "end": [1199, 32], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/kernel/models/actions.rs:1199`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4c4c452e06752fa59045c2e"></a>
## eq

`function` · `deltalake_core::kernel::models::actions::IsolationLevel::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &IsolationLevel) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1199).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::IsolationLevel", "path": "IsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1199, 54], "end": [1199, 63], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/kernel/models/actions.rs:1199`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d10bc178ea7754f34922acb3"></a>
## fmt

`function` · `deltalake_core::kernel::models::actions::IsolationLevel::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1199).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::IsolationLevel", "path": "IsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1199, 34], "end": [1199, 39], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/kernel/models/actions.rs:1199`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a9e2f248ee6c1c2459f300d"></a>
## from_str

`function` · `deltalake_core::kernel::models::actions::IsolationLevel::from_str` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1240).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::IsolationLevel", "path": "IsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1237, 1], "end": [1248, 2], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `crates/core/src/kernel/models/actions.rs:1240`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7de1c8892cc25afb8c08e92f"></a>
## serialize

`function` · `deltalake_core::kernel::models::actions::IsolationLevel::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1199).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::IsolationLevel", "path": "IsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1199, 10], "end": [1199, 19], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/kernel/models/actions.rs:1199`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
