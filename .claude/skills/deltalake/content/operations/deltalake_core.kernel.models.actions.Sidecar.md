# `deltalake_core::kernel::models::actions::Sidecar`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.models.actions.Sidecar.json).

<a id="op-173fadbdbee85f2fe645693e"></a>
## Sidecar

`struct` · `deltalake_core::kernel::models::actions::Sidecar` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct Sidecar
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1178).

Source: `crates/core/src/kernel/models/actions.rs:1178`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The sidecar action references a sidecar file which provides some of the checkpoint's file actions.
This action is only allowed in checkpoints following V2 spec.

<a id="op-6e88c17ba017977c8418720a"></a>
## clone

`function` · `deltalake_core::kernel::models::actions::Sidecar::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> Sidecar
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1176).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Sidecar", "path": "Sidecar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1176, 41], "end": [1176, 46], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/kernel/models/actions.rs:1176`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c215e89cdf47c777c2ad864"></a>
## default

`function` · `deltalake_core::kernel::models::actions::Sidecar::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> Sidecar
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1176).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Sidecar", "path": "Sidecar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1176, 48], "end": [1176, 55], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/kernel/models/actions.rs:1176`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d755cead8c0c3a6db212b2b"></a>
## deserialize

`function` · `deltalake_core::kernel::models::actions::Sidecar::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1176).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Sidecar", "path": "Sidecar"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1176, 21], "end": [1176, 32], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/kernel/models/actions.rs:1176`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52b54fcdd1593640901cdab0"></a>
## eq

`function` · `deltalake_core::kernel::models::actions::Sidecar::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &Sidecar) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1176).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Sidecar", "path": "Sidecar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1176, 57], "end": [1176, 66], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/kernel/models/actions.rs:1176`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb83203e7aab0c0d0767dc68"></a>
## file_name

`struct_field` · `deltalake_core::kernel::models::actions::Sidecar::file_name` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
file_name: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1181).

Source: `crates/core/src/kernel/models/actions.rs:1181`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The name of the sidecar file (not a path).
The file must reside in the _delta_log/_sidecars directory.

<a id="op-17aa45885e635519b5a30624"></a>
## fmt

`function` · `deltalake_core::kernel::models::actions::Sidecar::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1176).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Sidecar", "path": "Sidecar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1176, 34], "end": [1176, 39], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/kernel/models/actions.rs:1176`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be3393f608e9747bc3efe408"></a>
## modification_time

`struct_field` · `deltalake_core::kernel::models::actions::Sidecar::modification_time` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
modification_time: i64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1187).

Source: `crates/core/src/kernel/models/actions.rs:1187`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The time this sidecar file was created, as milliseconds since the epoch.

<a id="op-f942fd4cf4b2d56ef1639f99"></a>
## serialize

`function` · `deltalake_core::kernel::models::actions::Sidecar::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1176).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Sidecar", "path": "Sidecar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1176, 10], "end": [1176, 19], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/kernel/models/actions.rs:1176`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-faa12c583babe0fe30ea765c"></a>
## sidecar_type

`struct_field` · `deltalake_core::kernel::models::actions::Sidecar::sidecar_type` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
sidecar_type: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1192).

Source: `crates/core/src/kernel/models/actions.rs:1192`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Type of sidecar. Valid values are: "fileaction".
This could be extended in future to allow different kinds of sidecars.

<a id="op-9dee34802a0f89f27db31e42"></a>
## size_in_bytes

`struct_field` · `deltalake_core::kernel::models::actions::Sidecar::size_in_bytes` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
size_in_bytes: i64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1184).

Source: `crates/core/src/kernel/models/actions.rs:1184`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The size of the sidecar file in bytes

<a id="op-e3de19c2d7b6fc2509a417f9"></a>
## tags

`struct_field` · `deltalake_core::kernel::models::actions::Sidecar::tags` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
tags: Option<std::collections::HashMap<String, Option<String>>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1196).

Source: `crates/core/src/kernel/models/actions.rs:1196`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Map containing any additional metadata about the checkpoint sidecar file.
