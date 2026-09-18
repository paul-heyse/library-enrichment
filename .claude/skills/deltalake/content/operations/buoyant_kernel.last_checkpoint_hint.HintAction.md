# `buoyant_kernel::last_checkpoint_hint::HintAction`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.last_checkpoint_hint.HintAction.json).

<a id="op-7f1761c7488b3ce277a0d428"></a>
## HintAction

`enum` · `buoyant_kernel::last_checkpoint_hint::HintAction` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum HintAction
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L100).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:100`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

One element of [`LastCheckpointV2`](../operations/buoyant_kernel.last_checkpoint_hint.LastCheckpointV2.md#op-4615f576c8f975ebdde3d30e)'s `non_file_actions`. A log action is exactly one action
type, so this is an externally-tagged enum keyed by the action name, reusing kernel's action
structs to yield the same types as log replay. An unrecognized action key fails the whole-hint
parse; `try_read` swallows that, so the reader falls back to reading the checkpoint.

<a id="op-858adc756f78a493e77f2fc0"></a>
## CheckpointMetadata

`variant` · `buoyant_kernel::last_checkpoint_hint::HintAction::CheckpointMetadata` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
CheckpointMetadata
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L106).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:106`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb3b3b44bbe79416d336ba2a"></a>
## DomainMetadata

`variant` · `buoyant_kernel::last_checkpoint_hint::HintAction::DomainMetadata` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
DomainMetadata
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L105).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:105`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-60bd6b92e2831ca055723bc4"></a>
## Metadata

`variant` · `buoyant_kernel::last_checkpoint_hint::HintAction::Metadata` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Metadata
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L102).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:102`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02a9829cb87381d63d605550"></a>
## Protocol

`variant` · `buoyant_kernel::last_checkpoint_hint::HintAction::Protocol` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Protocol
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L103).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:103`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc583a2ad29cea2d7e428060"></a>
## Txn

`variant` · `buoyant_kernel::last_checkpoint_hint::HintAction::Txn` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Txn
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L104).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:104`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8668b0d3cd77667e9260758b"></a>
## clone

`function` · `buoyant_kernel::last_checkpoint_hint::HintAction::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> HintAction
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L97).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::last_checkpoint_hint::HintAction", "path": "HintAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 17], "end": [97, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:97`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e1959ed989158691661bd42"></a>
## deserialize

`function` · `buoyant_kernel::last_checkpoint_hint::HintAction::deserialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L97).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::last_checkpoint_hint::HintAction", "path": "HintAction"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 24], "end": [97, 35], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:97`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5090bb651aba3a2b801ae9fe"></a>
## eq

`function` · `buoyant_kernel::last_checkpoint_hint::HintAction::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &HintAction) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L97).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::last_checkpoint_hint::HintAction", "path": "HintAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 48], "end": [97, 57], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:97`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-266328a643ea5df7c2bd86b2"></a>
## fmt

`function` · `buoyant_kernel::last_checkpoint_hint::HintAction::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L97).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::last_checkpoint_hint::HintAction", "path": "HintAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 10], "end": [97, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:97`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e90a85fe6225d4a2c286b15a"></a>
## serialize

`function` · `buoyant_kernel::last_checkpoint_hint::HintAction::serialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L97).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::last_checkpoint_hint::HintAction", "path": "HintAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 37], "end": [97, 46], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:97`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
