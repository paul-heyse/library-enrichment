# `buoyant_kernel::ExpressionEvaluator`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.ExpressionEvaluator.json).

<a id="op-02eea996b6bdb19503c4315a"></a>
## ExpressionEvaluator

`trait` · `buoyant_kernel::ExpressionEvaluator` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait ExpressionEvaluator: AsAny
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L423).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:423`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Trait for implementing an Expression evaluator.

It contains one Expression which can be evaluated on multiple ColumnarBatches.
Connectors can implement this trait to optimize the evaluation using the
connector specific capabilities.

<a id="op-236f32309603ecfc975f8b0b"></a>
## evaluate

`function` · `buoyant_kernel::ExpressionEvaluator::evaluate` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn evaluate(&self, batch: &dyn EngineData) -> DeltaResult<Box<dyn EngineData>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L429).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:429`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Evaluate the expression on a given EngineData.

Produces one value for each row of the input.
The data type of the output is same as the type output of the expression this evaluator is
using.
