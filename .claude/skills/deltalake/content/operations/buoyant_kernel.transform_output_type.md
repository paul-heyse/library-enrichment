# `buoyant_kernel::transform_output_type`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transform_output_type.json).

<a id="op-ad05bd806f416e4d185977fc"></a>
## transform_output_type

`macro` · `buoyant_kernel::transform_output_type` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
macro_rules! transform_output_type
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/mod.rs#L53).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/mod.rs:53`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Defines a transform's `Output` and `Residual` associated types.

Example: fallible schema visitor
```rust,no_run
# use buoyant_kernel as delta_kernel;
# use delta_kernel::transform_output_type;
# use delta_kernel::schema::StructField;
# use delta_kernel::transforms::SchemaTransform;
# use delta_kernel::DeltaResult;
struct Validate;

impl<'a> SchemaTransform<'a> for Validate {
    transform_output_type!(|'a, T| DeltaResult<()>);

    fn transform_struct_field(&mut self, _field: &'a StructField) -> DeltaResult<()> {
        todo!()
    }
}
```
# use buoyant_kernel as delta_kernel;

Example: infallible filtering expression transform
```rust,no_run
# use buoyant_kernel as delta_kernel;
# use std::borrow::Cow;
# use delta_kernel::transform_output_type;
# use delta_kernel::expressions::ColumnName;
# use delta_kernel::transforms::ExpressionTransform;
struct KeepSomeColumns;

impl<'a> ExpressionTransform<'a> for KeepSomeColumns {
    transform_output_type!(|'a, T| Option<Cow<'a, T>>);

    fn transform_expr_column(&mut self, _name: &'a ColumnName) -> Option<Cow<'a, ColumnName>> {
        todo!()
    }
}
```
