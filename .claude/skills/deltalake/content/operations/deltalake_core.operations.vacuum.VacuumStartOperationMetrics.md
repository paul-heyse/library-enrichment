# `deltalake_core::operations::vacuum::VacuumStartOperationMetrics`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.vacuum.VacuumStartOperationMetrics.json).

<a id="op-9d238400318797ec0b51fe40"></a>
## VacuumStartOperationMetrics

`struct` · `deltalake_core::operations::vacuum::VacuumStartOperationMetrics` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct VacuumStartOperationMetrics
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L237).

Source: `crates/core/src/operations/vacuum.rs:237`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Details for the Vacuum start operation for the transaction log

<a id="op-2a510a1304ee647fc7fe4249"></a>
## num_files_to_delete

`struct_field` · `deltalake_core::operations::vacuum::VacuumStartOperationMetrics::num_files_to_delete` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_files_to_delete: i64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L239).

Source: `crates/core/src/operations/vacuum.rs:239`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The number of files that will be deleted

<a id="op-bc898fcd27116b975a57d48d"></a>
## serialize

`function` · `deltalake_core::operations::vacuum::VacuumStartOperationMetrics::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L235).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::vacuum::VacuumStartOperationMetrics", "path": "VacuumStartOperationMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 10], "end": [235, 19], "filename": "crates/core/src/operations/vacuum.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/operations/vacuum.rs:235`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14c2b735594d3bd1e4e3dd7b"></a>
## size_of_data_to_delete

`struct_field` · `deltalake_core::operations::vacuum::VacuumStartOperationMetrics::size_of_data_to_delete` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
size_of_data_to_delete: i64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L241).

Source: `crates/core/src/operations/vacuum.rs:241`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Size of the data to be deleted in bytes
