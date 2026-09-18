# `buoyant_kernel::kernel_predicates::DirectPredicateEvaluator`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.kernel_predicates.DirectPredicateEvaluator.json).

<a id="op-ae005f39b2a93dbdf09be0fc"></a>
## DirectPredicateEvaluator

`type_alias` · `buoyant_kernel::kernel_predicates::DirectPredicateEvaluator` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type DirectPredicateEvaluator<'a> = dyn KernelPredicateEvaluator<Output = bool> + 'a
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L32).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:32`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A predicate evaluator that directly evaluates predicates, resolving column references to scalar
values.
