# `buoyant_kernel::table_properties::IsolationLevel`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.table_properties.IsolationLevel.json).

<a id="op-0e84bb8695a601705ec9659b"></a>
## IsolationLevel

`enum` · `buoyant_kernel::table_properties::IsolationLevel` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum IsolationLevel
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L326).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:326`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The isolation level applied during transaction

<a id="op-739de2d674eef03587310e38"></a>
## Err

`assoc_type` · `buoyant_kernel::table_properties::IsolationLevel::Err` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Err = ParseError
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L324).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_properties::IsolationLevel", "path": "IsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [324, 17], "end": [324, 27], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:324`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3fde153e1a244c19f4b565b6"></a>
## Error

`assoc_type` · `buoyant_kernel::table_properties::IsolationLevel::Error` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Error = ParseError
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L324).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_properties::IsolationLevel", "path": "IsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [324, 17], "end": [324, 27], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:324`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ee3c4b71fdb8135d7c451b7"></a>
## Serializable

`variant` · `buoyant_kernel::table_properties::IsolationLevel::Serializable` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Serializable
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L333).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:333`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The strongest isolation level. It ensures that committed write operations
and all reads are Serializable. Operations are allowed as long as there
exists a serial sequence of executing them one-at-a-time that generates
the same outcome as that seen in the table. For the write operations,
the serial sequence is exactly the same as that seen in the table’s history.

<a id="op-d3b7109d03a89559013c0f72"></a>
## SnapshotIsolation

`variant` · `buoyant_kernel::table_properties::IsolationLevel::SnapshotIsolation` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
SnapshotIsolation
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L345).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:345`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

SnapshotIsolation is a guarantee that all reads made in a transaction will see a consistent
snapshot of the database (in practice it reads the last committed values that existed at
the time it started), and the transaction itself will successfully commit only if no
updates it has made conflict with any concurrent updates made since that snapshot.

<a id="op-25f8f9ed1fb964f5de5e20e6"></a>
## WriteSerializable

`variant` · `buoyant_kernel::table_properties::IsolationLevel::WriteSerializable` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
WriteSerializable
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L339).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:339`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A weaker isolation level than Serializable. It ensures only that the write
operations (that is, not reads) are serializable. However, this is still stronger
than Snapshot isolation. WriteSerializable is the default isolation level because
it provides great balance of data consistency and availability for most common operations.

<a id="op-ecc4666817d5bdbf7fb9ae17"></a>
## clone

`function` · `buoyant_kernel::table_properties::IsolationLevel::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> IsolationLevel
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L324).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_properties::IsolationLevel", "path": "IsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [324, 44], "end": [324, 49], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:324`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a41a9ff51ae88473be5fedf6"></a>
## default

`function` · `buoyant_kernel::table_properties::IsolationLevel::default` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> IsolationLevel
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L324).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_properties::IsolationLevel", "path": "IsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [324, 29], "end": [324, 36], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:324`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-648223b952dcfeebd16ff3b4"></a>
## eq

`function` · `buoyant_kernel::table_properties::IsolationLevel::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &IsolationLevel) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L324).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_properties::IsolationLevel", "path": "IsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [324, 51], "end": [324, 60], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:324`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7341a17545a12c35cdcf2420"></a>
## fmt

`function` · `buoyant_kernel::table_properties::IsolationLevel::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L324).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_properties::IsolationLevel", "path": "IsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [324, 10], "end": [324, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:324`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8c9fe977a0f07fe005b7a92"></a>
## from_str

`function` · `buoyant_kernel::table_properties::IsolationLevel::from_str` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_str(s: &str) -> ::core::result::Result<IsolationLevel, <Self as ::core::str::FromStr>::Err>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L324).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_properties::IsolationLevel", "path": "IsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [324, 17], "end": [324, 27], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:324`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-552042229dce3460fbca0667"></a>
## try_from

`function` · `buoyant_kernel::table_properties::IsolationLevel::try_from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_from(s: &str) -> ::core::result::Result<IsolationLevel, <Self as ::core::convert::TryFrom>::Error>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L324).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_properties::IsolationLevel", "path": "IsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [324, 17], "end": [324, 27], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:324`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
