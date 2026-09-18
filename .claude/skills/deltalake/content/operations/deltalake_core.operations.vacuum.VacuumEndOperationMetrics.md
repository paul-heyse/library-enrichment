# `deltalake_core::operations::vacuum::VacuumEndOperationMetrics`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.vacuum.VacuumEndOperationMetrics.json).

<a id="op-9a6a3f72f30db97adf57f4a3"></a>
## VacuumEndOperationMetrics

`struct` · `deltalake_core::operations::vacuum::VacuumEndOperationMetrics` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct VacuumEndOperationMetrics
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L247).

Source: `crates/core/src/operations/vacuum.rs:247`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Details for the Vacuum End operation for the transaction log

<a id="op-adf939bb4c67c515a73f00f3"></a>
## num_deleted_files

`struct_field` · `deltalake_core::operations::vacuum::VacuumEndOperationMetrics::num_deleted_files` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_deleted_files: i64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L249).

Source: `crates/core/src/operations/vacuum.rs:249`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The number of actually deleted files

<a id="op-c1996a00144d54bda28eac47"></a>
## num_vacuumed_directories

`struct_field` · `deltalake_core::operations::vacuum::VacuumEndOperationMetrics::num_vacuumed_directories` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_vacuumed_directories: i64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L251).

Source: `crates/core/src/operations/vacuum.rs:251`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The number of actually vacuumed directories

<a id="op-7ed70a51593a5d7a01316217"></a>
## serialize

`function` · `deltalake_core::operations::vacuum::VacuumEndOperationMetrics::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L245).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::vacuum::VacuumEndOperationMetrics", "path": "VacuumEndOperationMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [245, 10], "end": [245, 19], "filename": "crates/core/src/operations/vacuum.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/operations/vacuum.rs:245`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
