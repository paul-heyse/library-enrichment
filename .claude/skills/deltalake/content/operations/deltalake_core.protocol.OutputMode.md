# `deltalake_core::protocol::OutputMode`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.protocol.OutputMode.json).

<a id="op-dc79e86a0f7432c76ff33d6e"></a>
## OutputMode

`enum` · `deltalake_core::protocol::OutputMode` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum OutputMode
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L534).

Source: `crates/core/src/protocol/mod.rs:534`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The OutputMode used in streaming operations.

<a id="op-b89004187870e8e7176b26c9"></a>
## Append

`variant` · `deltalake_core::protocol::OutputMode::Append` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Append
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L536).

Source: `crates/core/src/protocol/mod.rs:536`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Only new rows will be written when new data is available.

<a id="op-75d383db841f485eb5575403"></a>
## Complete

`variant` · `deltalake_core::protocol::OutputMode::Complete` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Complete
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L538).

Source: `crates/core/src/protocol/mod.rs:538`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The full output (all rows) will be written whenever new data is available.

<a id="op-f0ab0d88e005b827d33fc5ac"></a>
## Update

`variant` · `deltalake_core::protocol::OutputMode::Update` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Update
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L540).

Source: `crates/core/src/protocol/mod.rs:540`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Only rows with updates will be written when new or changed data is available.

<a id="op-4da38570791a4052c25019a8"></a>
## clone

`function` · `deltalake_core::protocol::OutputMode::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> OutputMode
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L533).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::OutputMode", "path": "OutputMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [533, 47], "end": [533, 52], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/protocol/mod.rs:533`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f378f090e24db1733828cb4"></a>
## deserialize

`function` · `deltalake_core::protocol::OutputMode::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L533).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::OutputMode", "path": "OutputMode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [533, 21], "end": [533, 32], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/protocol/mod.rs:533`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e668c37eee0fc5e582e828b"></a>
## fmt

`function` · `deltalake_core::protocol::OutputMode::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L533).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::OutputMode", "path": "OutputMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [533, 34], "end": [533, 39], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/protocol/mod.rs:533`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dec88f9b9ffaad738a6dd9a0"></a>
## serialize

`function` · `deltalake_core::protocol::OutputMode::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L533).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::OutputMode", "path": "OutputMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [533, 10], "end": [533, 19], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/protocol/mod.rs:533`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
