# `buoyant_kernel::struct_patch`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.struct_patch.json).

<a id="op-8311bf2e09e602d4a52be7cf"></a>
## struct_patch

`module` · `buoyant_kernel::struct_patch` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod struct_patch
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Struct patches: sparse, `O(changes)` edits to the fields of an input struct.

A struct patch keeps, drops, replaces, or inserts fields relative to an input struct without
enumerating untouched fields. Patches are built with a [`StructPatchBuilder`](../operations/buoyant_kernel.struct_patch.StructPatchBuilder.md#op-efa0d2ea419cd24c4cbedbb3), which validates
conflicting operations and lowers nested field paths into recursive patches. Three flavors share
the same builder surface, differing only in their item type and terminal `build` step:

* [`ExpressionStructPatchBuilder`](crate::expressions::ExpressionStructPatchBuilder) emits
  expressions and produces a sparse [`ExpressionStructPatch`](../operations/buoyant_kernel.struct_patch.ExpressionStructPatch.md#op-1f79369df99deab3abc687ed) that is embedded in an
  [`Expression`](../operations/buoyant_kernel.expressions.Expression.md#op-b6e8a7405239f2ae57fc5e51) and applied to data at evaluation time.
* [`SchemaStructPatchBuilder`](crate::schema::SchemaStructPatchBuilder) emits schema fields and
  produces an output [`StructType`](../operations/buoyant_kernel.schema.StructType.md#op-ac9d10e1b52f1ac2eacc4e98) directly from an input schema.
* [`ProjectionStructPatchBuilder`](../operations/buoyant_kernel.struct_patch.ProjectionStructPatchBuilder.md#op-c538ec8abbe84f3fd9494ac1) pairs each output field with the expression that produces it
  and lowers both at once to a matched ([`SchemaRef`](../operations/buoyant_kernel.schema.SchemaRef.md#op-bcbc708676b5e88d4183ee59), `[ExpressionRef`]) pair.
