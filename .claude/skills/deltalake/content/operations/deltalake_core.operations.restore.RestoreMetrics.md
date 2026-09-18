# `deltalake_core::operations::restore::RestoreMetrics`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.restore.RestoreMetrics.json).

<a id="op-67036c6eed450f33138b8136"></a>
## RestoreMetrics

`struct` · `deltalake_core::operations::restore::RestoreMetrics` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct RestoreMetrics
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/restore.rs#L72).

Source: `crates/core/src/operations/restore.rs:72`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Metrics from Restore

<a id="op-2de8b49302237fe45ab0f802"></a>
## default

`function` · `deltalake_core::operations::restore::RestoreMetrics::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> RestoreMetrics
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/restore.rs#L70).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::restore::RestoreMetrics", "path": "RestoreMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 10], "end": [70, 17], "filename": "crates/core/src/operations/restore.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/operations/restore.rs:70`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c84979729b3869dab77003e6"></a>
## fmt

`function` · `deltalake_core::operations::restore::RestoreMetrics::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/restore.rs#L70).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::restore::RestoreMetrics", "path": "RestoreMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 19], "end": [70, 24], "filename": "crates/core/src/operations/restore.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/operations/restore.rs:70`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b33bb1570e6e8241342bbeb2"></a>
## num_removed_file

`struct_field` · `deltalake_core::operations::restore::RestoreMetrics::num_removed_file` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_removed_file: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/restore.rs#L74).

Source: `crates/core/src/operations/restore.rs:74`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of files removed

<a id="op-58baf8767bc09b811204dbb1"></a>
## num_restored_file

`struct_field` · `deltalake_core::operations::restore::RestoreMetrics::num_restored_file` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_restored_file: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/restore.rs#L76).

Source: `crates/core/src/operations/restore.rs:76`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of files restored

<a id="op-f4bc7d015c55c8c02fbb83ed"></a>
## serialize

`function` · `deltalake_core::operations::restore::RestoreMetrics::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/restore.rs#L70).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::restore::RestoreMetrics", "path": "RestoreMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 26], "end": [70, 35], "filename": "crates/core/src/operations/restore.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/operations/restore.rs:70`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
