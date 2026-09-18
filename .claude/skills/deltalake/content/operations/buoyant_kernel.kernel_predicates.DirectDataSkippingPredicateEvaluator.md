# `buoyant_kernel::kernel_predicates::DirectDataSkippingPredicateEvaluator`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.kernel_predicates.DirectDataSkippingPredicateEvaluator.json).

<a id="op-55861932ef47a08b45ec2d61"></a>
## DirectDataSkippingPredicateEvaluator

`type_alias` · `buoyant_kernel::kernel_predicates::DirectDataSkippingPredicateEvaluator` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type DirectDataSkippingPredicateEvaluator<'a> = dyn DataSkippingPredicateEvaluator<Output = bool, ColumnStat = expressions::Scalar> + 'a
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L36).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:36`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A data skipping predicate evaluator that directly applies data skipping, resolving column
references to scalar stats values such as those provided by parquet footer stats.
