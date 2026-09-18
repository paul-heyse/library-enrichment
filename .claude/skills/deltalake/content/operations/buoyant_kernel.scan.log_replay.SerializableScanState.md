# `buoyant_kernel::scan::log_replay::SerializableScanState`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.scan.log_replay.SerializableScanState.json).

<a id="op-a3955a174cb07140f1222cd8"></a>
## SerializableScanState

`struct` · `buoyant_kernel::scan::log_replay::SerializableScanState` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct SerializableScanState
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/log_replay.rs#L116).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/log_replay.rs:116`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Serializable processor state for distributed processing. This can be serialized using the
default serde serialization, or through custom serialization in the engine.

This struct contains all the information needed to reconstruct a `ScanLogReplayProcessor`
on remote compute nodes, enabling distributed log replay processing.

# Serialization Limitations

- **Opaque expressions**: Predicates containing [`Predicate::Opaque`] or expressions containing
  [`Expression::Opaque`] cannot be serialized using serde. Attempting to serialize state with
  opaque expressions will result in an error. Connectors that require opaque expression support
  can work around this by serializing the predicate separately using their own serialization
  mechanism, then reconstructing the processor state on the remote node.

- **Large state**: The `seen_file_keys` field can be large for tables with many commits.
  Connectors are free to serialize this field using their own format (e.g., more compact binary
  representations) rather than using the serde-based serialization.

[`Predicate::Opaque`]: crate::expressions::Predicate::Opaque
[`Expression::Opaque`]: crate::expressions::Expression::Opaque

<a id="op-15dfddd8e8b52a8569dca320"></a>
## deserialize

`function` · `buoyant_kernel::scan::log_replay::SerializableScanState::deserialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/log_replay.rs#L114).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::log_replay::SerializableScanState", "path": "SerializableScanState"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 10], "end": [114, 21], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/log_replay.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/log_replay.rs:114`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4283aee0d090d57324a1250"></a>
## internal_state_blob

`struct_field` · `buoyant_kernel::scan::log_replay::SerializableScanState::internal_state_blob` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
internal_state_blob: Vec<u8>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/log_replay.rs#L120).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/log_replay.rs:120`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Opaque internal state blob

<a id="op-a765f7ab2be86775882b7cb2"></a>
## predicate

`struct_field` · `buoyant_kernel::scan::log_replay::SerializableScanState::predicate` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
predicate: Option<expressions::PredicateRef>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/log_replay.rs#L118).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/log_replay.rs:118`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Optional predicate for data skipping (if provided)

<a id="op-0f82f9a36ff068c8427ab33f"></a>
## seen_file_keys

`struct_field` · `buoyant_kernel::scan::log_replay::SerializableScanState::seen_file_keys` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
seen_file_keys: std::collections::HashSet<log_replay::FileActionKey>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/log_replay.rs#L122).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/log_replay.rs:122`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Set of file action keys that have already been processed.

<a id="op-210dbbce742b2eb6aa6f0906"></a>
## serialize

`function` · `buoyant_kernel::scan::log_replay::SerializableScanState::serialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/log_replay.rs#L114).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::log_replay::SerializableScanState", "path": "SerializableScanState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 23], "end": [114, 32], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/log_replay.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/log_replay.rs:114`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a5478d633b4b380183dc11d"></a>
## checkpoint_info

`struct_field` · `buoyant_kernel::scan::log_replay::SerializableScanState::checkpoint_info` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
checkpoint_info: log_segment::CheckpointReadInfo
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/log_replay.rs#L124).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/log_replay.rs:124`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Information about checkpoint reading for stats optimization
