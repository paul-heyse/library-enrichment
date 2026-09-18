# `deltalake_core::kernel::models::actions::DomainMetadata`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.models.actions.DomainMetadata.json).

<a id="op-4d4af16ae24e0a8e80a11bee"></a>
## DomainMetadata

`struct` · `deltalake_core::kernel::models::actions::DomainMetadata` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DomainMetadata
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1152).

Source: `crates/core/src/kernel/models/actions.rs:1152`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The domain metadata action contains a configuration (string) for a named metadata domain

<a id="op-d2e6381516b81cb2f9eafc6c"></a>
## clone

`function` · `deltalake_core::kernel::models::actions::DomainMetadata::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> DomainMetadata
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1150).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::DomainMetadata", "path": "DomainMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1150, 41], "end": [1150, 46], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/kernel/models/actions.rs:1150`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e018e8eee924896e034f55e"></a>
## configuration

`struct_field` · `deltalake_core::kernel::models::actions::DomainMetadata::configuration` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
configuration: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1157).

Source: `crates/core/src/kernel/models/actions.rs:1157`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

String containing configuration for the metadata domain

<a id="op-3ddab209499e4400887777be"></a>
## default

`function` · `deltalake_core::kernel::models::actions::DomainMetadata::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> DomainMetadata
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1150).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::DomainMetadata", "path": "DomainMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1150, 48], "end": [1150, 55], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/kernel/models/actions.rs:1150`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7564842879086f7527a2596d"></a>
## deserialize

`function` · `deltalake_core::kernel::models::actions::DomainMetadata::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1150).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::DomainMetadata", "path": "DomainMetadata"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1150, 21], "end": [1150, 32], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/kernel/models/actions.rs:1150`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a63a6da156274b92490eafd"></a>
## domain

`struct_field` · `deltalake_core::kernel::models::actions::DomainMetadata::domain` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
domain: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1154).

Source: `crates/core/src/kernel/models/actions.rs:1154`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Identifier for this domain (system or user-provided)

<a id="op-bcd7696239efb6b9c7e6cf58"></a>
## eq

`function` · `deltalake_core::kernel::models::actions::DomainMetadata::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &DomainMetadata) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1150).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::DomainMetadata", "path": "DomainMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1150, 57], "end": [1150, 66], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/kernel/models/actions.rs:1150`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b599e4b98c37361e429065e2"></a>
## fmt

`function` · `deltalake_core::kernel::models::actions::DomainMetadata::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1150).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::DomainMetadata", "path": "DomainMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1150, 34], "end": [1150, 39], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/kernel/models/actions.rs:1150`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a4105326566fd70b18f3168"></a>
## removed

`struct_field` · `deltalake_core::kernel::models::actions::DomainMetadata::removed` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
removed: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1160).

Source: `crates/core/src/kernel/models/actions.rs:1160`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

When `true` the action serves as a tombstone

<a id="op-7843ad5dd2a655423dcf7f46"></a>
## serialize

`function` · `deltalake_core::kernel::models::actions::DomainMetadata::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1150).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::DomainMetadata", "path": "DomainMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1150, 10], "end": [1150, 19], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/kernel/models/actions.rs:1150`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
