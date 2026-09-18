# `buoyant_kernel::expressions::OpaqueExpressionOpRef`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.expressions.OpaqueExpressionOpRef.json).

<a id="op-7192d507bfc517a436375c74"></a>
## OpaqueExpressionOpRef

`type_alias` · `buoyant_kernel::expressions::OpaqueExpressionOpRef` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type OpaqueExpressionOpRef = std::sync::Arc<dyn OpaqueExpressionOp>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L220).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:220`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A shared reference to an [`OpaqueExpressionOp`](../operations/buoyant_kernel.expressions.OpaqueExpressionOp.md#op-353b5fa46cb006e99cf79ad2) instance.
