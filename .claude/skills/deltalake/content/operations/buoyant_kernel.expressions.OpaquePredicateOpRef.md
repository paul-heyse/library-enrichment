# `buoyant_kernel::expressions::OpaquePredicateOpRef`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.expressions.OpaquePredicateOpRef.json).

<a id="op-83d210406a1f9789e39648da"></a>
## OpaquePredicateOpRef

`type_alias` · `buoyant_kernel::expressions::OpaquePredicateOpRef` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type OpaquePredicateOpRef = std::sync::Arc<dyn OpaquePredicateOp>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L223).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:223`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A shared reference to an [`OpaquePredicateOp`](../operations/buoyant_kernel.expressions.OpaquePredicateOp.md#op-0af44540a36e1b39fab1b889) instance.
