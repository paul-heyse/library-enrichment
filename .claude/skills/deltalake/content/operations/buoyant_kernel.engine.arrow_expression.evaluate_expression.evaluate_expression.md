# `buoyant_kernel::engine::arrow_expression::evaluate_expression::evaluate_expression`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.arrow_expression.evaluate_expression.evaluate_expression.json).

<a id="op-e6a0596cf3347dea1ad10b2f"></a>
## evaluate_expression

`function` · `buoyant_kernel::engine::arrow_expression::evaluate_expression::evaluate_expression` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn evaluate_expression(expression: &expressions::Expression, batch: &arrow::array::RecordBatch, result_type: Option<&schema::DataType>) -> error::DeltaResult<arrow::array::ArrayRef>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/evaluate_expression.rs#L262).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/evaluate_expression.rs:262`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Evaluates a kernel expression over a record batch
