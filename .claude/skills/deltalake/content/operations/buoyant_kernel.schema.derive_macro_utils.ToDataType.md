# `buoyant_kernel::schema::derive_macro_utils::ToDataType`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.schema.derive_macro_utils.ToDataType.json).

<a id="op-07cabab8877812d7bcc53456"></a>
## ToDataType

`trait` · `buoyant_kernel::schema::derive_macro_utils::ToDataType` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait ToDataType
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/derive_macro_utils.rs#L13).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/derive_macro_utils.rs:13`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Converts a type to a [`DataType`](../operations/buoyant_kernel.schema.DataType.md#op-ae66d4bfcb2bbb0ad1521faa). Implemented for the primitive types and automatically derived
for all types that implement [`ToSchema`](../operations/buoyant_kernel.schema.ToSchema.md#op-7415228dd6d01071e4f2a1c8).

<a id="op-044370b63bffdaa4f624c978"></a>
## to_data_type

`function` · `buoyant_kernel::schema::derive_macro_utils::ToDataType::to_data_type` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn to_data_type() -> DataType
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/derive_macro_utils.rs#L14).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/derive_macro_utils.rs:14`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
