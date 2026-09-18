# `buoyant_kernel::last_checkpoint_hint::LastCheckpointHint`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.last_checkpoint_hint.LastCheckpointHint.json).

<a id="op-cc49bf8c5ed0b8b91d681ef0"></a>
## LastCheckpointHint

`struct` · `buoyant_kernel::last_checkpoint_hint::LastCheckpointHint` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct LastCheckpointHint
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L35).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:35`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd099063ca6b5e295d690068"></a>
## clone

`function` · `buoyant_kernel::last_checkpoint_hint::LastCheckpointHint::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> LastCheckpointHint
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::last_checkpoint_hint::LastCheckpointHint", "path": "LastCheckpointHint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 17], "end": [31, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:31`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-43bb5d94741c28a2b9b531ca"></a>
## deserialize

`function` · `buoyant_kernel::last_checkpoint_hint::LastCheckpointHint::deserialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::last_checkpoint_hint::LastCheckpointHint", "path": "LastCheckpointHint"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 24], "end": [31, 35], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:31`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d77fbf0078d6e197386bda5"></a>
## eq

`function` · `buoyant_kernel::last_checkpoint_hint::LastCheckpointHint::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &LastCheckpointHint) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::last_checkpoint_hint::LastCheckpointHint", "path": "LastCheckpointHint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 48], "end": [31, 57], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:31`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02865fcc700777b7223b361b"></a>
## fmt

`function` · `buoyant_kernel::last_checkpoint_hint::LastCheckpointHint::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::last_checkpoint_hint::LastCheckpointHint", "path": "LastCheckpointHint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 10], "end": [31, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:31`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4bbd7b8e3905cbd0f66a0057"></a>
## path

`function` · `buoyant_kernel::last_checkpoint_hint::LastCheckpointHint::path` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn path(log_root: &Url) -> DeltaResult<Url>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L177).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::last_checkpoint_hint::LastCheckpointHint", "path": "LastCheckpointHint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [109, 1], "end": [229, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:177`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the path of the `_last_checkpoint` file given the log root of a table.

<a id="op-dfeb2f5f9043125d5cbfec15"></a>
## serialize

`function` · `buoyant_kernel::last_checkpoint_hint::LastCheckpointHint::serialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::last_checkpoint_hint::LastCheckpointHint", "path": "LastCheckpointHint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 37], "end": [31, 46], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:31`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22ce3e434bdae13c466b3cb1"></a>
## version

`struct_field` · `buoyant_kernel::last_checkpoint_hint::LastCheckpointHint::version` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
version: Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L38).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:38`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The version of the table when the last checkpoint was made.

<a id="op-b6f57c934afa2018f1f63252"></a>
## checkpoint_schema

`struct_field` · `buoyant_kernel::last_checkpoint_hint::LastCheckpointHint::checkpoint_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
checkpoint_schema: Option<schema::SchemaRef>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L49).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:49`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The schema of the checkpoint file.

<a id="op-9dd274707ab693b1aebbdeb4"></a>
## checksum

`struct_field` · `buoyant_kernel::last_checkpoint_hint::LastCheckpointHint::checksum` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
checksum: Option<String>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L51).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:51`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The checksum of the last checkpoint JSON.

<a id="op-60ac7048f1a3a0e8c4667607"></a>
## num_of_add_files

`struct_field` · `buoyant_kernel::last_checkpoint_hint::LastCheckpointHint::num_of_add_files` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
num_of_add_files: Option<i64>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L47).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:47`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The number of AddFile actions in the checkpoint.

<a id="op-b6ff08012b72a2f79c27cfaf"></a>
## parts

`struct_field` · `buoyant_kernel::last_checkpoint_hint::LastCheckpointHint::parts` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
parts: Option<usize>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L43).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:43`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The number of fragments if the last checkpoint was written in multiple parts. `None` means
a single-part or classic checkpoint (i.e. `numParts == 1`).

<a id="op-2d493c6062a582e8385ab11a"></a>
## size

`struct_field` · `buoyant_kernel::last_checkpoint_hint::LastCheckpointHint::size` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
size: i64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L40).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:40`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The number of actions that are stored in the checkpoint.

<a id="op-e289ab71624d06d710c4f050"></a>
## size_in_bytes

`struct_field` · `buoyant_kernel::last_checkpoint_hint::LastCheckpointHint::size_in_bytes` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
size_in_bytes: Option<i64>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L45).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:45`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The number of bytes of the checkpoint.

<a id="op-b491b5fa9d69573ee6727efd"></a>
## tags

`struct_field` · `buoyant_kernel::last_checkpoint_hint::LastCheckpointHint::tags` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
tags: Option<std::collections::HashMap<String, String>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L53).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:53`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Additional metadata about the last checkpoint.

<a id="op-8acb0ca75bbe6a5e148dab18"></a>
## v2_checkpoint

`struct_field` · `buoyant_kernel::last_checkpoint_hint::LastCheckpointHint::v2_checkpoint` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
v2_checkpoint: Option<LastCheckpointV2>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L56).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:56`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

For a V2 checkpoint, the embedded V2 checkpoint info. Identifies the specific checkpoint
file the hint describes. Absent for V1 / classic checkpoints.
