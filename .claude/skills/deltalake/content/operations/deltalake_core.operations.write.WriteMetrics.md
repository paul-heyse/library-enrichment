# `deltalake_core::operations::write::WriteMetrics`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.write.WriteMetrics.json).

<a id="op-6b14068d1f9063cb9ad0030d"></a>
## WriteMetrics

`struct` · `deltalake_core::operations::write::WriteMetrics` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct WriteMetrics
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L171).

Source: `crates/core/src/operations/write/mod.rs:171`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Metrics for the Write Operation

<a id="op-8c4db34a73b986b6320103e9"></a>
## default

`function` · `deltalake_core::operations::write::WriteMetrics::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> WriteMetrics
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L169).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::WriteMetrics", "path": "WriteMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [169, 10], "end": [169, 17], "filename": "crates/core/src/operations/write/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/operations/write/mod.rs:169`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7d392047ae1ff6280467815"></a>
## deserialize

`function` · `deltalake_core::operations::write::WriteMetrics::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L169).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::WriteMetrics", "path": "WriteMetrics"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [169, 37], "end": [169, 48], "filename": "crates/core/src/operations/write/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/operations/write/mod.rs:169`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33d502fe695c0c15a87bdf3d"></a>
## execution_time_ms

`struct_field` · `deltalake_core::operations::write::WriteMetrics::execution_time_ms` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
execution_time_ms: u64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L181).

Source: `crates/core/src/operations/write/mod.rs:181`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Time taken to execute the entire operation

<a id="op-dba4d24798c69696aaf36b0c"></a>
## fmt

`function` · `deltalake_core::operations::write::WriteMetrics::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L169).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::WriteMetrics", "path": "WriteMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [169, 19], "end": [169, 24], "filename": "crates/core/src/operations/write/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/operations/write/mod.rs:169`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7bdba202e007ea4c2cb62722"></a>
## num_added_files

`struct_field` · `deltalake_core::operations::write::WriteMetrics::num_added_files` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_added_files: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L173).

Source: `crates/core/src/operations/write/mod.rs:173`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of files added

<a id="op-e91181d069437b4f94ea8393"></a>
## num_added_rows

`struct_field` · `deltalake_core::operations::write::WriteMetrics::num_added_rows` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_added_rows: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L179).

Source: `crates/core/src/operations/write/mod.rs:179`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of rows added

<a id="op-68c6e1a71d81fef9a2e457d9"></a>
## num_partitions

`struct_field` · `deltalake_core::operations::write::WriteMetrics::num_partitions` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_partitions: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L177).

Source: `crates/core/src/operations/write/mod.rs:177`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of partitions

<a id="op-7501a8608923d4bd06c06d2a"></a>
## num_removed_files

`struct_field` · `deltalake_core::operations::write::WriteMetrics::num_removed_files` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_removed_files: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L175).

Source: `crates/core/src/operations/write/mod.rs:175`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of files removed

<a id="op-570bd14f521550f97b1f9bae"></a>
## num_retries

`struct_field` · `deltalake_core::operations::write::WriteMetrics::num_retries` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_retries: u64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L184).

Source: `crates/core/src/operations/write/mod.rs:184`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of retries before successful commit

<a id="op-b9a8123502805377556d6de7"></a>
## serialize

`function` · `deltalake_core::operations::write::WriteMetrics::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L169).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::WriteMetrics", "path": "WriteMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [169, 26], "end": [169, 35], "filename": "crates/core/src/operations/write/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/operations/write/mod.rs:169`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
