# `datafusion_physical_plan::joins::StreamJoinPartitionMode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.StreamJoinPartitionMode.json).

<a id="op-f906260debd31240ac4c5bf7"></a>
## StreamJoinPartitionMode

`enum` · `datafusion_physical_plan::joins::StreamJoinPartitionMode` · datafusion-physical-plan 55.1.0

```rust
enum StreamJoinPartitionMode
```

Source: `src/joins/mod.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Partitioning mode to use for symmetric hash join

<a id="op-7e897c5243716d5a9a79de5b"></a>
## Partitioned

`variant` · `datafusion_physical_plan::joins::StreamJoinPartitionMode::Partitioned` · datafusion-physical-plan 55.1.0

```rust
Partitioned
```

Source: `src/joins/mod.rs:111`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Left/right children are partitioned using the left and right keys

<a id="op-c6afe7035cdd8b2a08d5a677"></a>
## SinglePartition

`variant` · `datafusion_physical_plan::joins::StreamJoinPartitionMode::SinglePartition` · datafusion-physical-plan 55.1.0

```rust
SinglePartition
```

Source: `src/joins/mod.rs:113`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Both sides will collected into one partition

<a id="op-a162cef78a9bdb366575a377"></a>
## clone

`function` · `datafusion_physical_plan::joins::StreamJoinPartitionMode::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> StreamJoinPartitionMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::StreamJoinPartitionMode", "path": "StreamJoinPartitionMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 16], "end": [108, 21], "filename": "src/joins/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/joins/mod.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-98ba9871f7e83db6dc60a532"></a>
## eq

`function` · `datafusion_physical_plan::joins::StreamJoinPartitionMode::eq` · datafusion-physical-plan 55.1.0

```rust
fn eq(&self, other: &StreamJoinPartitionMode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::StreamJoinPartitionMode", "path": "StreamJoinPartitionMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 36], "end": [108, 45], "filename": "src/joins/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/joins/mod.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9a47915613dd95331596585"></a>
## fmt

`function` · `datafusion_physical_plan::joins::StreamJoinPartitionMode::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::StreamJoinPartitionMode", "path": "StreamJoinPartitionMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 29], "end": [108, 34], "filename": "src/joins/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/joins/mod.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01d2e8b5e5bf46d631954610"></a>
## hash

`function` · `datafusion_physical_plan::joins::StreamJoinPartitionMode::hash` · datafusion-physical-plan 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::StreamJoinPartitionMode", "path": "StreamJoinPartitionMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 10], "end": [108, 14], "filename": "src/joins/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/joins/mod.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
