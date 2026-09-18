# `buoyant_kernel::engine::arrow_expression::evaluate_expression::evaluate_predicate`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.arrow_expression.evaluate_expression.evaluate_predicate.json).

<a id="op-0acb98b3bf661d23a4e8e770"></a>
## evaluate_predicate

`function` · `buoyant_kernel::engine::arrow_expression::evaluate_expression::evaluate_predicate` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn evaluate_predicate(predicate: &expressions::Predicate, batch: &arrow::array::RecordBatch, inverted: bool) -> error::DeltaResult<arrow::array::BooleanArray>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/evaluate_expression.rs#L593).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/evaluate_expression.rs:593`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Evaluates a (possibly inverted) kernel predicate over a record batch
