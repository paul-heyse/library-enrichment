# `buoyant_kernel::expressions::literal_expression_transform`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.expressions.literal_expression_transform.json).

<a id="op-726d11ce1b69df7fc3334dd6"></a>
## literal_expression_transform

`module` · `buoyant_kernel::expressions::literal_expression_transform` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod literal_expression_transform
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/literal_expression_transform.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/literal_expression_transform.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Transforms a [`StructType`](../operations/buoyant_kernel.schema.StructType.md#op-ac9d10e1b52f1ac2eacc4e98) and an ordered list of leaf values (scalars) into an
[`Expression`](../operations/buoyant_kernel.expressions.Expression.md#op-b6e8a7405239f2ae57fc5e51) with a literal value for each leaf.
