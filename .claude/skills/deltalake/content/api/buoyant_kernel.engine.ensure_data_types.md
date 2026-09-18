# `buoyant_kernel::engine::ensure_data_types`

Crate `buoyant_kernel` · 3 public items · structured records in [`model/buoyant_kernel.engine.ensure_data_types.json`](../model/buoyant_kernel.engine.ensure_data_types.json)

## DataTypeCompat

`enum` · `buoyant_kernel::engine::ensure_data_types::DataTypeCompat`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine.ensure_data_types.DataTypeCompat.md)

Also reachable as `delta_kernel::engine::ensure_data_types::DataTypeCompat`

```rust
enum DataTypeCompat
```

**Variants**: `Identical`, `NeedsCast`, `Nested`

Capture the compatibility between two data-types, as passed to [`ensure_data_types`]

---

## ValidationMode

`enum` · `buoyant_kernel::engine::ensure_data_types::ValidationMode`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine.ensure_data_types.ValidationMode.md)

Also reachable as `delta_kernel::engine::ensure_data_types::ValidationMode`

```rust
enum ValidationMode
```

**Variants**: `TypesOnly`, `TypesAndNames`, `Full`

**Derives**: Clone, Copy

Controls how `ensure_data_types` validates struct fields and metadata.

---

## ensure_data_types

`function` · `buoyant_kernel::engine::ensure_data_types::ensure_data_types`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine.ensure_data_types.ensure_data_types.md)

Also reachable as `delta_kernel::engine::ensure_data_types::ensure_data_types`

```rust
fn ensure_data_types(kernel_type: &schema::DataType, arrow_type: &arrow::datatypes::DataType, mode: ValidationMode) -> DeltaResult<DataTypeCompat>
```

Ensure a kernel data type matches an arrow data type. This only ensures that the actual "type"
is the same, but does so recursively into structs, and ensures lists and maps have the correct
associated types as well.

The `mode` parameter controls how struct fields are matched and whether nullability/metadata
are checked. See [`ValidationMode`] for details.

This returns an `Ok(DataTypeCompat)` if the types are compatible, and
will indicate what kind of compatibility they have, or an error if the types do not match. If
there is a `struct` type included and the mode uses name-based matching, we only ensure that
the named fields that the kernel is asking for exist, and that for those fields the types
match. Un-selected fields are ignored.

---
