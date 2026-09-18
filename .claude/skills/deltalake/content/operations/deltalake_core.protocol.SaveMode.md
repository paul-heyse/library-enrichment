# `deltalake_core::protocol::SaveMode`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.protocol.SaveMode.json).

<a id="op-4925b7d8982cc5cdd1b9def4"></a>
## SaveMode

`enum` · `deltalake_core::protocol::SaveMode` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum SaveMode
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L505).

Source: `crates/core/src/protocol/mod.rs:505`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The SaveMode used when performing a DeltaOperation

<a id="op-e534509e195b77cc629cee34"></a>
## Append

`variant` · `deltalake_core::protocol::SaveMode::Append` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Append
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L507).

Source: `crates/core/src/protocol/mod.rs:507`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Files will be appended to the target location.

<a id="op-e6ed79b554f45f8c5a4b0849"></a>
## Err

`assoc_type` · `deltalake_core::protocol::SaveMode::Err` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Err = DeltaTableError
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L517).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::SaveMode", "path": "SaveMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 1], "end": [530, 2], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `crates/core/src/protocol/mod.rs:517`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fedeae97bb89561f17c5a04d"></a>
## ErrorIfExists

`variant` · `deltalake_core::protocol::SaveMode::ErrorIfExists` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ErrorIfExists
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L511).

Source: `crates/core/src/protocol/mod.rs:511`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

If files exist for the target, the operation must fail.

<a id="op-05a2906b72a839504d48d9e4"></a>
## Ignore

`variant` · `deltalake_core::protocol::SaveMode::Ignore` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Ignore
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L513).

Source: `crates/core/src/protocol/mod.rs:513`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

If files exist for the target, the operation must not proceed or change any data.

<a id="op-9f92b999fc5a64ad7dbfb692"></a>
## Overwrite

`variant` · `deltalake_core::protocol::SaveMode::Overwrite` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Overwrite
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L509).

Source: `crates/core/src/protocol/mod.rs:509`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The target location will be overwritten.

<a id="op-42067c302d70518bc6084864"></a>
## clone

`function` · `deltalake_core::protocol::SaveMode::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> SaveMode
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L504).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::SaveMode", "path": "SaveMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [504, 47], "end": [504, 52], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/protocol/mod.rs:504`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4cd34991c74cc5b212b821df"></a>
## deserialize

`function` · `deltalake_core::protocol::SaveMode::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L504).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::SaveMode", "path": "SaveMode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [504, 21], "end": [504, 32], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/protocol/mod.rs:504`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1c6697cb2d5b052a8a78cd3"></a>
## eq

`function` · `deltalake_core::protocol::SaveMode::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &SaveMode) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L504).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::SaveMode", "path": "SaveMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [504, 54], "end": [504, 63], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/protocol/mod.rs:504`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb9e5af1cdbdc4bd40d8f732"></a>
## fmt

`function` · `deltalake_core::protocol::SaveMode::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L504).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::SaveMode", "path": "SaveMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [504, 34], "end": [504, 39], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/protocol/mod.rs:504`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8277d27ef6e025682e3b2f7c"></a>
## from_str

`function` · `deltalake_core::protocol::SaveMode::from_str` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_str(s: &str) -> DeltaResult<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L519).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::SaveMode", "path": "SaveMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 1], "end": [530, 2], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `crates/core/src/protocol/mod.rs:519`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e70a4568ead428be797f9a9d"></a>
## serialize

`function` · `deltalake_core::protocol::SaveMode::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L504).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::SaveMode", "path": "SaveMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [504, 10], "end": [504, 19], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/protocol/mod.rs:504`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
