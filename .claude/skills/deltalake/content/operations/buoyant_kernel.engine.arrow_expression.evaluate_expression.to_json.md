# `buoyant_kernel::engine::arrow_expression::evaluate_expression::to_json`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.arrow_expression.evaluate_expression.to_json.json).

<a id="op-985838ad760bca982e6375d7"></a>
## to_json

`function` · `buoyant_kernel::engine::arrow_expression::evaluate_expression::to_json` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn to_json(input: &dyn Datum) -> Result<arrow::array::ArrayRef, arrow::error::ArrowError>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/evaluate_expression.rs#L754).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/evaluate_expression.rs:754`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Converts a StructArray to JSON-encoded strings
