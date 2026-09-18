# `buoyant_kernel::kernel_predicates::IndirectDataSkippingPredicateEvaluator`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.kernel_predicates.IndirectDataSkippingPredicateEvaluator.json).

<a id="op-c35596042fd4d3b57c5f5523"></a>
## IndirectDataSkippingPredicateEvaluator

`type_alias` · `buoyant_kernel::kernel_predicates::IndirectDataSkippingPredicateEvaluator` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type IndirectDataSkippingPredicateEvaluator<'a> = dyn DataSkippingPredicateEvaluator<Output = expressions::Predicate, ColumnStat = expressions::Expression> + 'a
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L42).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:42`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A data skipping predicate evaluator that rewrites the input to a predicate that performs data
skipping over column stats for all referenced columns. The resulting predicate can be evaluated
against batches of column stats at some future point.
