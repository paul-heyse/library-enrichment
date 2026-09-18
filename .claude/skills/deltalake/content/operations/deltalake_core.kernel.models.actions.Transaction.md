# `deltalake_core::kernel::models::actions::Transaction`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.models.actions.Transaction.json).

<a id="op-a3f53757c12624ebe6835f51"></a>
## Transaction

`struct` · `deltalake_core::kernel::models::actions::Transaction` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct Transaction
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1062).

Source: `crates/core/src/kernel/models/actions.rs:1062`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Action used by streaming systems to track progress using application-specific versions to
enable idempotency.

<a id="op-f0fe73a825ed6f351524c9c8"></a>
## app_id

`struct_field` · `deltalake_core::kernel::models::actions::Transaction::app_id` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
app_id: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1064).

Source: `crates/core/src/kernel/models/actions.rs:1064`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A unique identifier for the application performing the transaction.

<a id="op-ac499f58a55dc45e7a079d8e"></a>
## clone

`function` · `deltalake_core::kernel::models::actions::Transaction::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> Transaction
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1060).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Transaction", "path": "Transaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1060, 50], "end": [1060, 55], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/kernel/models/actions.rs:1060`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3737085c93c4a8eba48a962f"></a>
## default

`function` · `deltalake_core::kernel::models::actions::Transaction::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> Transaction
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1060).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Transaction", "path": "Transaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1060, 41], "end": [1060, 48], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/kernel/models/actions.rs:1060`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce043cb0c9f16a781bd35be8"></a>
## deserialize

`function` · `deltalake_core::kernel::models::actions::Transaction::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1060).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Transaction", "path": "Transaction"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1060, 21], "end": [1060, 32], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/kernel/models/actions.rs:1060`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a0af364753b16cc15db584c"></a>
## eq

`function` · `deltalake_core::kernel::models::actions::Transaction::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &Transaction) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1060).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Transaction", "path": "Transaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1060, 57], "end": [1060, 66], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/kernel/models/actions.rs:1060`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb43b93558c9d98c323cbd89"></a>
## fmt

`function` · `deltalake_core::kernel::models::actions::Transaction::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1060).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Transaction", "path": "Transaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1060, 34], "end": [1060, 39], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/kernel/models/actions.rs:1060`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e805775acb0805c5d2039b15"></a>
## last_updated

`struct_field` · `deltalake_core::kernel::models::actions::Transaction::last_updated` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
last_updated: Option<i64>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1071).

Source: `crates/core/src/kernel/models/actions.rs:1071`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The time when this transaction action was created in milliseconds since the Unix epoch.

<a id="op-2e71c7743f89ced1de81aab3"></a>
## new

`function` · `deltalake_core::kernel::models::actions::Transaction::new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(app_id: impl ToString, version: i64) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1076).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Transaction", "path": "Transaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1074, 1], "end": [1092, 2], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/models/actions.rs:1076`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new application transactions. See [`Txn`] for details.

<a id="op-6d0b649393036f8afb41e574"></a>
## new_with_last_update

`function` · `deltalake_core::kernel::models::actions::Transaction::new_with_last_update` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new_with_last_update(app_id: impl ToString, version: i64, last_updated: Option<i64>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1081).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Transaction", "path": "Transaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1074, 1], "end": [1092, 2], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/models/actions.rs:1081`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new application transactions. See [`Txn`] for details.

<a id="op-77e94b63e74da6545ef60167"></a>
## serialize

`function` · `deltalake_core::kernel::models::actions::Transaction::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1060).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Transaction", "path": "Transaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1060, 10], "end": [1060, 19], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/kernel/models/actions.rs:1060`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d59f814d8d8e827e73d628ad"></a>
## version

`struct_field` · `deltalake_core::kernel::models::actions::Transaction::version` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
version: i64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1067).

Source: `crates/core/src/kernel/models/actions.rs:1067`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

An application-specific numeric identifier for this transaction.
