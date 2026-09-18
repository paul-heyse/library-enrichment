# `buoyant_kernel::schema::derive_macro_utils::GetStructField`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.schema.derive_macro_utils.GetStructField.json).

<a id="op-b922dc7054b791b2af95724d"></a>
## GetStructField

`trait` · `buoyant_kernel::schema::derive_macro_utils::GetStructField` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait GetStructField
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/derive_macro_utils.rs#L80).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/derive_macro_utils.rs:80`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The [`delta_kernel_derive::ToSchema`] macro uses this to convert a struct field's name + type
into a `StructField` definition. A blanket impl for `Option<T: ToDataType>` supports nullable
struct fields, which otherwise default to non-nullable.

Unresolved upstream links (retained, not inferred): ``delta_kernel_derive::ToSchema``.

<a id="op-c31b1ed1a976b7266d51cc52"></a>
## get_struct_field

`function` · `buoyant_kernel::schema::derive_macro_utils::GetStructField::get_struct_field` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_struct_field(name: impl Into<String>) -> StructField
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/derive_macro_utils.rs#L81).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/derive_macro_utils.rs:81`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
