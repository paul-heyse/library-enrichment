# `deltalake_core::operations::convert_to_delta::PartitionStrategy`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.convert_to_delta.PartitionStrategy.json).

<a id="op-b403c089e6a9462461a9b100"></a>
## PartitionStrategy

`enum` · `deltalake_core::operations::convert_to_delta::PartitionStrategy` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum PartitionStrategy
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L87).

Source: `crates/core/src/operations/convert_to_delta.rs:87`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The partition strategy used by the Parquet table
Currently only hive-partitioning is supported for Parquet paths

<a id="op-a1542ca0cab27c77765ca6d8"></a>
## Err

`assoc_type` · `deltalake_core::operations::convert_to_delta::PartitionStrategy::Err` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Err = DeltaTableError
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L94).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::convert_to_delta::PartitionStrategy", "path": "PartitionStrategy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [104, 2], "filename": "crates/core/src/operations/convert_to_delta.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `crates/core/src/operations/convert_to_delta.rs:94`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7bc7a08675747bdb202a33b8"></a>
## Hive

`variant` · `deltalake_core::operations::convert_to_delta::PartitionStrategy::Hive` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Hive
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L90).

Source: `crates/core/src/operations/convert_to_delta.rs:90`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Hive-partitioning

<a id="op-8963450ca89dcffa5ed04f92"></a>
## default

`function` · `deltalake_core::operations::convert_to_delta::PartitionStrategy::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> PartitionStrategy
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L86).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::convert_to_delta::PartitionStrategy", "path": "PartitionStrategy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 10], "end": [86, 17], "filename": "crates/core/src/operations/convert_to_delta.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/operations/convert_to_delta.rs:86`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-894802e6e0a47a176149c6dd"></a>
## from_str

`function` · `deltalake_core::operations::convert_to_delta::PartitionStrategy::from_str` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_str(s: &str) -> DeltaResult<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L96).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::convert_to_delta::PartitionStrategy", "path": "PartitionStrategy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [104, 2], "filename": "crates/core/src/operations/convert_to_delta.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `crates/core/src/operations/convert_to_delta.rs:96`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
