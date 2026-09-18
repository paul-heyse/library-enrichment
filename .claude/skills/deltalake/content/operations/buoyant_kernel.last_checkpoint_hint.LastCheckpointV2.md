# `buoyant_kernel::last_checkpoint_hint::LastCheckpointV2`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.last_checkpoint_hint.LastCheckpointV2.json).

<a id="op-4615f576c8f975ebdde3d30e"></a>
## LastCheckpointV2

`struct` · `buoyant_kernel::last_checkpoint_hint::LastCheckpointV2` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct LastCheckpointV2
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L68).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:68`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The `v2Checkpoint` object embedded in a `_last_checkpoint` hint for a V2 checkpoint.

Carries the V2 checkpoint file's identity and metadata plus the actions a reader would otherwise
read from the checkpoint itself -- its sidecar references and its non-file actions. Absent for
V1 / classic checkpoints.

<a id="op-fcc586de591cfb91795e7ad7"></a>
## clone

`function` · `buoyant_kernel::last_checkpoint_hint::LastCheckpointV2::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> LastCheckpointV2
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L64).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::last_checkpoint_hint::LastCheckpointV2", "path": "LastCheckpointV2"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 17], "end": [64, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:64`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c5bf5b2e12f7d29ce9e5b66"></a>
## deserialize

`function` · `buoyant_kernel::last_checkpoint_hint::LastCheckpointV2::deserialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L64).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::last_checkpoint_hint::LastCheckpointV2", "path": "LastCheckpointV2"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 24], "end": [64, 35], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:64`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b4f8cd9608584a00d09d1135"></a>
## eq

`function` · `buoyant_kernel::last_checkpoint_hint::LastCheckpointV2::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &LastCheckpointV2) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L64).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::last_checkpoint_hint::LastCheckpointV2", "path": "LastCheckpointV2"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 48], "end": [64, 57], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:64`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c9c6bcedc553fe6c53c57ecd"></a>
## fmt

`function` · `buoyant_kernel::last_checkpoint_hint::LastCheckpointV2::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L64).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::last_checkpoint_hint::LastCheckpointV2", "path": "LastCheckpointV2"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 10], "end": [64, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:64`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d49190526a0512e648804f45"></a>
## serialize

`function` · `buoyant_kernel::last_checkpoint_hint::LastCheckpointV2::serialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L64).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::last_checkpoint_hint::LastCheckpointV2", "path": "LastCheckpointV2"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 37], "end": [64, 46], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:64`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0514a66d62f851681e5770cb"></a>
## modification_time

`struct_field` · `buoyant_kernel::last_checkpoint_hint::LastCheckpointV2::modification_time` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
modification_time: Option<i64>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L79).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:79`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Modification time of the V2 checkpoint file named by `path`, in milliseconds since the Unix
epoch.

<a id="op-07ae5d13854d020fb54b2524"></a>
## non_file_actions

`struct_field` · `buoyant_kernel::last_checkpoint_hint::LastCheckpointV2::non_file_actions` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
non_file_actions: Option<Vec<HintAction>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L90).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:90`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The checkpoint's non-file actions (see [`HintAction`](../operations/buoyant_kernel.last_checkpoint_hint.HintAction.md#op-7f1761c7488b3ce277a0d428)), letting a reader obtain them
without reading the checkpoint file. Dropped to `None` by
[`LastCheckpointHint::drop_oversized_fields`] when the count exceeds the threshold.

Unresolved upstream links (retained, not inferred): ``LastCheckpointHint::drop_oversized_fields``.

<a id="op-040b20404511c30859557020"></a>
## path

`struct_field` · `buoyant_kernel::last_checkpoint_hint::LastCheckpointV2::path` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
path: String
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L72).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:72`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Bare file name of the V2 checkpoint this hint describes, matched against the selected
checkpoint part's file name. Several V2 checkpoints can share a version, so this identifies
which one the hint's fields describe.

<a id="op-6aae0c5ae73c9d706cfa5399"></a>
## sidecar_files

`struct_field` · `buoyant_kernel::last_checkpoint_hint::LastCheckpointV2::sidecar_files` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
sidecar_files: Option<Vec<actions::Sidecar>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L85).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:85`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The sidecar files this checkpoint references, for a manifest (non-leaf) V2 checkpoint.
Empty/absent for a leaf checkpoint that inlines its file actions. Also dropped to `None` by
[`LastCheckpointHint::drop_oversized_fields`] when the count exceeds the threshold, so
absence is a missing optimization, never a signal that the checkpoint is a leaf.

Unresolved upstream links (retained, not inferred): ``LastCheckpointHint::drop_oversized_fields``.

<a id="op-202c45b7d3fb023fb349af04"></a>
## size_in_bytes

`struct_field` · `buoyant_kernel::last_checkpoint_hint::LastCheckpointV2::size_in_bytes` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
size_in_bytes: Option<i64>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L75).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:75`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Size in bytes of the V2 checkpoint file named by `path`.
