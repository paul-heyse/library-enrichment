# `buoyant_kernel::expressions::ExpressionStructPatchBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.expressions.ExpressionStructPatchBuilder.json).

<a id="op-c292c4f3a2cbf09638cc5e18"></a>
## ExpressionStructPatchBuilder

`type_alias` · `buoyant_kernel::expressions::ExpressionStructPatchBuilder` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type ExpressionStructPatchBuilder = struct_patch::StructPatchBuilder<ExpressionRef>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L54).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:54`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A [`StructPatchBuilder`](crate::struct_patch::StructPatchBuilder) whose emitted items are
expressions, lowered into an [`ExpressionStructPatch`](../operations/buoyant_kernel.struct_patch.ExpressionStructPatch.md#op-1f79369df99deab3abc687ed) that can be embedded in an
[`Expression`](../operations/buoyant_kernel.expressions.Expression.md#op-b6e8a7405239f2ae57fc5e51).
