# `buoyant_kernel::engine::ensure_data_types::ensure_data_types`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.ensure_data_types.ensure_data_types.json).

<a id="op-6a09270f4b3ca4f205abd9f0"></a>
## ensure_data_types

`function` · `buoyant_kernel::engine::ensure_data_types::ensure_data_types` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn ensure_data_types(kernel_type: &schema::DataType, arrow_type: &arrow::datatypes::DataType, mode: ValidationMode) -> DeltaResult<DataTypeCompat>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/ensure_data_types.rs#L44).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/ensure_data_types.rs:44`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Ensure a kernel data type matches an arrow data type. This only ensures that the actual "type"
is the same, but does so recursively into structs, and ensures lists and maps have the correct
associated types as well.

The `mode` parameter controls how struct fields are matched and whether nullability/metadata
are checked. See [`ValidationMode`](../operations/buoyant_kernel.engine.ensure_data_types.ValidationMode.md#op-6fb2bbb0327bb0fa70d05659) for details.

This returns an `Ok(DataTypeCompat)` if the types are compatible, and
will indicate what kind of compatibility they have, or an error if the types do not match. If
there is a `struct` type included and the mode uses name-based matching, we only ensure that
the named fields that the kernel is asking for exist, and that for those fields the types
match. Un-selected fields are ignored.
