# `buoyant_kernel::log_segment::CheckpointReadInfo`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.log_segment.CheckpointReadInfo.json).

<a id="op-e07ae8d4381f4a978ecac0ff"></a>
## CheckpointReadInfo

`struct` · `buoyant_kernel::log_segment::CheckpointReadInfo` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct CheckpointReadInfo
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment/mod.rs#L51).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs:51`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Information about checkpoint reading for data skipping optimization.

Returned alongside the actions iterator from checkpoint reading functions.

<a id="op-23106ce31171f1532b009e64"></a>
## checkpoint_read_schema

`struct_field` · `buoyant_kernel::log_segment::CheckpointReadInfo::checkpoint_read_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
checkpoint_read_schema: schema::SchemaRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment/mod.rs#L64).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs:64`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The schema used to read checkpoint files, potentially including stats_parsed.

<a id="op-ee22e252554a604f2b812294"></a>
## clone

`function` · `buoyant_kernel::log_segment::CheckpointReadInfo::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> CheckpointReadInfo
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment/mod.rs#L49).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::log_segment::CheckpointReadInfo", "path": "CheckpointReadInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 17], "end": [49, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs:49`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b85d302a53083bddfd687d5b"></a>
## deserialize

`function` · `buoyant_kernel::log_segment::CheckpointReadInfo::deserialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment/mod.rs#L49).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::log_segment::CheckpointReadInfo", "path": "CheckpointReadInfo"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 42], "end": [49, 60], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs:49`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b92effadc59e2b9f4523d5bb"></a>
## fmt

`function` · `buoyant_kernel::log_segment::CheckpointReadInfo::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment/mod.rs#L49).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::log_segment::CheckpointReadInfo", "path": "CheckpointReadInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 10], "end": [49, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs:49`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8813c71f2173fd6a45964e1"></a>
## has_partition_values_parsed

`struct_field` · `buoyant_kernel::log_segment::CheckpointReadInfo::has_partition_values_parsed` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
has_partition_values_parsed: bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment/mod.rs#L61).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs:61`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Whether the checkpoint has compatible pre-parsed partition values.
When `true`, checkpoint batches can read typed partition values directly from
`partitionValues_parsed` instead of parsing strings from `partitionValues`.

<a id="op-f51a5e5e320b6e82c9594cf8"></a>
## has_stats_parsed

`struct_field` · `buoyant_kernel::log_segment::CheckpointReadInfo::has_stats_parsed` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
has_stats_parsed: bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment/mod.rs#L55).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs:55`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Whether the checkpoint has compatible pre-parsed stats for data skipping.
When `true`, checkpoint batches can use stats_parsed directly instead of parsing JSON.

<a id="op-7e29740b9ffa3a396de606fd"></a>
## serialize

`function` · `buoyant_kernel::log_segment::CheckpointReadInfo::serialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment/mod.rs#L49).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::log_segment::CheckpointReadInfo", "path": "CheckpointReadInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 24], "end": [49, 40], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs:49`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
