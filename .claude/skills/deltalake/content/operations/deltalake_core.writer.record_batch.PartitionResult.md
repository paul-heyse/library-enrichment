# `deltalake_core::writer::record_batch::PartitionResult`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.writer.record_batch.PartitionResult.json).

<a id="op-e0b16890117347dce8e15d56"></a>
## PartitionResult

`struct` · `deltalake_core::writer::record_batch::PartitionResult` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct PartitionResult
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/record_batch.rs#L446).

Source: `crates/core/src/writer/record_batch.rs:446`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Helper container for partitioned record batches

<a id="op-4dc3041b272f7896aa2095c8"></a>
## clone

`function` · `deltalake_core::writer::record_batch::PartitionResult::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> PartitionResult
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/record_batch.rs#L445).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::record_batch::PartitionResult", "path": "PartitionResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [445, 10], "end": [445, 15], "filename": "crates/core/src/writer/record_batch.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/writer/record_batch.rs:445`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-251642ebd9cc1c7707b29604"></a>
## fmt

`function` · `deltalake_core::writer::record_batch::PartitionResult::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/record_batch.rs#L445).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::record_batch::PartitionResult", "path": "PartitionResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [445, 17], "end": [445, 22], "filename": "crates/core/src/writer/record_batch.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/writer/record_batch.rs:445`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6cd1e9335802adfe33d1e868"></a>
## partition_values

`struct_field` · `deltalake_core::writer::record_batch::PartitionResult::partition_values` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
partition_values: indexmap::IndexMap<String, delta_kernel::expressions::Scalar>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/record_batch.rs#L448).

Source: `crates/core/src/writer/record_batch.rs:448`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

values found in partition columns

<a id="op-f4c478dc510f244d0e61a892"></a>
## record_batch

`struct_field` · `deltalake_core::writer::record_batch::PartitionResult::record_batch` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
record_batch: arrow_array::RecordBatch
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/record_batch.rs#L450).

Source: `crates/core/src/writer/record_batch.rs:450`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

remaining dataset with partition column values removed
