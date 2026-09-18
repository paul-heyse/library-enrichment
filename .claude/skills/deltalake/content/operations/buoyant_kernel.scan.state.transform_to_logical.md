# `buoyant_kernel::scan::state::transform_to_logical`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.scan.state.transform_to_logical.json).

<a id="op-2cc75df3421f6704fd535b62"></a>
## transform_to_logical

`function` · `buoyant_kernel::scan::state::transform_to_logical` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_to_logical(engine: &dyn Engine, physical_data: Box<dyn EngineData>, physical_schema: &schema::SchemaRef, logical_schema: &schema::Schema, transform: Option<ExpressionRef>) -> DeltaResult<Box<dyn EngineData>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/state.rs#L92).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/state.rs:92`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

utility function for applying a transform expression to convert data from physical to logical
format
