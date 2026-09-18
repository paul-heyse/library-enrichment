# `datafusion_physical_plan::joins::PartitionMode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.PartitionMode.json).

<a id="op-eac886db6323e4a5f561fa65"></a>
## PartitionMode

`enum` · `datafusion_physical_plan::joins::PartitionMode` · datafusion-physical-plan 55.1.0

```rust
enum PartitionMode
```

Source: `src/joins/mod.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Hash join Partitioning mode

<a id="op-7293c129fc36b5a0269f1a78"></a>
## Auto

`variant` · `datafusion_physical_plan::joins::PartitionMode::Auto` · datafusion-physical-plan 55.1.0

```rust
Auto
```

Source: `src/joins/mod.rs:104`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

DataFusion optimizer decides which PartitionMode
mode(Partitioned/CollectLeft) is optimal based on statistics. It will
also consider swapping the left and right inputs for the Join

<a id="op-f4e12708c93784574ae56b37"></a>
## CollectLeft

`variant` · `datafusion_physical_plan::joins::PartitionMode::CollectLeft` · datafusion-physical-plan 55.1.0

```rust
CollectLeft
```

Source: `src/joins/mod.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Left side will collected into one partition

<a id="op-b71396d1d1725b5437c17c9b"></a>
## Partitioned

`variant` · `datafusion_physical_plan::joins::PartitionMode::Partitioned` · datafusion-physical-plan 55.1.0

```rust
Partitioned
```

Source: `src/joins/mod.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Left/right children are partitioned using the left and right keys

<a id="op-507856cb1550d68bb4408ebd"></a>
## clone

`function` · `datafusion_physical_plan::joins::PartitionMode::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> PartitionMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::PartitionMode", "path": "PartitionMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 10], "end": [94, 15], "filename": "src/joins/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/joins/mod.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb77d1dd3ac17b66048767f5"></a>
## eq

`function` · `datafusion_physical_plan::joins::PartitionMode::eq` · datafusion-physical-plan 55.1.0

```rust
fn eq(&self, other: &PartitionMode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::PartitionMode", "path": "PartitionMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 30], "end": [94, 39], "filename": "src/joins/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/joins/mod.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59192227deb02a8c48b1af92"></a>
## fmt

`function` · `datafusion_physical_plan::joins::PartitionMode::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::PartitionMode", "path": "PartitionMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 23], "end": [94, 28], "filename": "src/joins/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/joins/mod.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
