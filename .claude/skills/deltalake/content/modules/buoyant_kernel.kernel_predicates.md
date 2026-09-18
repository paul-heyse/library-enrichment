# `buoyant_kernel::kernel_predicates`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.kernel_predicates.json).

<a id="op-1b9cda7bc2bdaba925bf9245"></a>
## kernel_predicates

`module` · `buoyant_kernel::kernel_predicates` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod kernel_predicates
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Support for kernel-driven predicate evaluation via the [`KernelPredicateEvaluator`](../operations/buoyant_kernel.kernel_predicates.KernelPredicateEvaluator.md#op-1dc6ecdedb87f64e3b8474da)
trait. Various trait implementations are used for partition pruning, stats-based data skipping,
and parquet row group filtering. The evaluation is normally performed over [`Scalar`](../operations/buoyant_kernel.expressions.scalars.Scalar.md#op-8dd45baeed3da91aa357441a) values,
but data skipping "evaluation" actually produces a transformed predicate that replaces column
references with stats column references, which log replay will instruct the engine to evaluate.
