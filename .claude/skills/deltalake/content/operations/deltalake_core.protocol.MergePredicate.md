# `deltalake_core::protocol::MergePredicate`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.protocol.MergePredicate.json).

<a id="op-f871b558d20fd1af1ff56fc3"></a>
## MergePredicate

`struct` · `deltalake_core::protocol::MergePredicate` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct MergePredicate
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L219).

Source: `crates/core/src/protocol/mod.rs:219`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Used to record the operations performed to the Delta Log

<a id="op-71afcd398e6cd11f4584b232"></a>
## action_type

`struct_field` · `deltalake_core::protocol::MergePredicate::action_type` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
action_type: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L221).

Source: `crates/core/src/protocol/mod.rs:221`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The type of merge operation performed

<a id="op-b053117107de39a26f3162d9"></a>
## clone

`function` · `deltalake_core::protocol::MergePredicate::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> MergePredicate
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L216).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::MergePredicate", "path": "MergePredicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [216, 41], "end": [216, 46], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/protocol/mod.rs:216`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6bf56d61b4edd1c45223df5"></a>
## deserialize

`function` · `deltalake_core::protocol::MergePredicate::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L216).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::MergePredicate", "path": "MergePredicate"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [216, 21], "end": [216, 32], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/protocol/mod.rs:216`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d577e033829052e8fe08297f"></a>
## fmt

`function` · `deltalake_core::protocol::MergePredicate::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L216).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::MergePredicate", "path": "MergePredicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [216, 34], "end": [216, 39], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/protocol/mod.rs:216`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-925a9b53be4c78aa100cbdbf"></a>
## predicate

`struct_field` · `deltalake_core::protocol::MergePredicate::predicate` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
predicate: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L224).

Source: `crates/core/src/protocol/mod.rs:224`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The predicate used for the merge operation

<a id="op-6a3344d34426ff012cbe0e42"></a>
## serialize

`function` · `deltalake_core::protocol::MergePredicate::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L216).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::MergePredicate", "path": "MergePredicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [216, 10], "end": [216, 19], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/protocol/mod.rs:216`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
