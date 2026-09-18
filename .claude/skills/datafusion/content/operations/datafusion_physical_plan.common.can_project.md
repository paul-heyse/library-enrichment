# `datafusion_physical_plan::common::can_project`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.common.can_project.json).

<a id="op-b0421bc75c5c16d30548be6c"></a>
## can_project

`function` · `datafusion_physical_plan::common::can_project` · datafusion-physical-plan 55.1.0

```rust
fn can_project(schema: &arrow::datatypes::SchemaRef, projection: Option<&[usize]>) -> datafusion_common::Result<()>
```

Source: `src/common.rs:285`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Checks if the given projection is valid for the given schema.
