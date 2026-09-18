# `buoyant_kernel::schema::ToSchema`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.schema.ToSchema.json).

<a id="op-7415228dd6d01071e4f2a1c8"></a>
## ToSchema

`trait` · `buoyant_kernel::schema::ToSchema` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait ToSchema
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L181).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:181`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Converts a type to a [`Schema`](../operations/buoyant_kernel.schema.Schema.md#op-636d4431ddb187018059ed1d) that represents that type. Derivable for struct types using the
[`delta_kernel_derive::ToSchema`] derive macro.

Unresolved upstream links (retained, not inferred): ``delta_kernel_derive::ToSchema``.

<a id="op-8ca4fcd69c91475bb2b24654"></a>
## to_schema

`function` · `buoyant_kernel::schema::ToSchema::to_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn to_schema() -> StructType
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L182).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:182`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
