# `deltalake_core::operations::delete::DeleteMetrics`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.delete.DeleteMetrics.json).

<a id="op-ecc0df20cfba2868788e0be9"></a>
## DeleteMetrics

`struct` · `deltalake_core::operations::delete::DeleteMetrics` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DeleteMetrics
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L127).

Source: `crates/core/src/operations/delete.rs:127`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Metrics for the Delete Operation

<a id="op-705d7dc42c0d30d2b5b68684"></a>
## default

`function` · `deltalake_core::operations::delete::DeleteMetrics::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> DeleteMetrics
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L125).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::delete::DeleteMetrics", "path": "DeleteMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [125, 10], "end": [125, 17], "filename": "crates/core/src/operations/delete.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/operations/delete.rs:125`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d722caa64cb9ba630a10a0ef"></a>
## execution_time_ms

`struct_field` · `deltalake_core::operations::delete::DeleteMetrics::execution_time_ms` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
execution_time_ms: u64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L144).

Source: `crates/core/src/operations/delete.rs:144`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Time taken to execute the entire operation

<a id="op-aaf4dc43d80ea1dea7309966"></a>
## fmt

`function` · `deltalake_core::operations::delete::DeleteMetrics::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L125).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::delete::DeleteMetrics", "path": "DeleteMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [125, 19], "end": [125, 24], "filename": "crates/core/src/operations/delete.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/operations/delete.rs:125`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78f684a8fb90c346d5e6b35c"></a>
## num_added_files

`struct_field` · `deltalake_core::operations::delete::DeleteMetrics::num_added_files` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_added_files: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L129).

Source: `crates/core/src/operations/delete.rs:129`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of files added

<a id="op-48703e47647eb5b6dc57e062"></a>
## num_copied_rows

`struct_field` · `deltalake_core::operations::delete::DeleteMetrics::num_copied_rows` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_copied_rows: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L142).

Source: `crates/core/src/operations/delete.rs:142`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of rows copied in the process of deleting files

<a id="op-0011bb064fc19823c3b3abec"></a>
## num_deleted_rows

`struct_field` · `deltalake_core::operations::delete::DeleteMetrics::num_deleted_rows` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_deleted_rows: Option<usize>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L140).

Source: `crates/core/src/operations/delete.rs:140`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Deleted row count when available from rewrite metrics or file metadata.

This is `None` for metadata only full file deletes when this library cannot
derive an exact count from file metadata alone.

Breaking change: this field is `Option<usize>` rather than `usize` so
callers must handle the unknown count case explicitly.

<a id="op-43138304d5cbeb127b56f39a"></a>
## num_removed_files

`struct_field` · `deltalake_core::operations::delete::DeleteMetrics::num_removed_files` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_removed_files: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L131).

Source: `crates/core/src/operations/delete.rs:131`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of files removed

<a id="op-7cf063483cb09dc2a7d12411"></a>
## rewrite_time_ms

`struct_field` · `deltalake_core::operations::delete::DeleteMetrics::rewrite_time_ms` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
rewrite_time_ms: u64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L148).

Source: `crates/core/src/operations/delete.rs:148`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Time taken to rewrite the matched files

<a id="op-89447004f431793c627de39f"></a>
## scan_time_ms

`struct_field` · `deltalake_core::operations::delete::DeleteMetrics::scan_time_ms` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
scan_time_ms: u64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L146).

Source: `crates/core/src/operations/delete.rs:146`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Time taken to scan the file for matches

<a id="op-bebd63d194fc62b0d8722b14"></a>
## serialize

`function` · `deltalake_core::operations::delete::DeleteMetrics::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/delete.rs#L125).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::delete::DeleteMetrics", "path": "DeleteMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [125, 26], "end": [125, 35], "filename": "crates/core/src/operations/delete.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/operations/delete.rs:125`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
