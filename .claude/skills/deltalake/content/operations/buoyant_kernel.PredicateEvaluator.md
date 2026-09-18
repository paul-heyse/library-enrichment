# `buoyant_kernel::PredicateEvaluator`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.PredicateEvaluator.json).

<a id="op-a92b57d01e446154b97fd576"></a>
## PredicateEvaluator

`trait` · `buoyant_kernel::PredicateEvaluator` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait PredicateEvaluator: AsAny
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L437).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:437`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Trait for implementing a Predicate evaluator.

It contains one Predicate which can be evaluated on multiple ColumnarBatches.
Connectors can implement this trait to optimize the evaluation using the
connector specific capabilities.

<a id="op-d2e1c0251daab89216986210"></a>
## evaluate

`function` · `buoyant_kernel::PredicateEvaluator::evaluate` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn evaluate(&self, batch: &dyn EngineData) -> DeltaResult<Box<dyn EngineData>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L441).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:441`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Evaluate the predicate on a given EngineData.

Produces one boolean value for each row of the input.
