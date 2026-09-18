# `buoyant_kernel::schema::SchemaStructPatchBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.schema.SchemaStructPatchBuilder.json).

<a id="op-b0fd11705149ab0b25dec628"></a>
## SchemaStructPatchBuilder

`type_alias` · `buoyant_kernel::schema::SchemaStructPatchBuilder` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type SchemaStructPatchBuilder = struct_patch::StructPatchBuilder<StructField>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L176).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:176`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A [`StructPatchBuilder`](crate::struct_patch::StructPatchBuilder) whose emitted items are schema
fields, lowered into an output [`StructType`](../operations/buoyant_kernel.schema.StructType.md#op-ac9d10e1b52f1ac2eacc4e98) directly from an input schema via
[`build`](crate::struct_patch::StructPatchBuilder::<StructField>::build).
