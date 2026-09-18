# `datafusion_substrait`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.json).

<a id="op-7acd54af47c417dc169179cb"></a>
## datafusion_substrait

`module` · `datafusion_substrait` · datafusion-substrait 55.1.0

```rust
mod datafusion_substrait
```

Source: `src/lib.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Serialize / Deserialize DataFusion Plans to [Substrait.io]

This crate provides support for serializing and deserializing both DataFusion
[`LogicalPlan`] and [`ExecutionPlan`] to and from the generated types in
[substrait::proto] from the [substrait] crate.

[Substrait.io] provides a cross-language serialization format for relational
algebra (e.g. query plans and expressions), based on protocol buffers.

[Substrait.io]: https://substrait.io/

[`LogicalPlan`]: datafusion::logical_expr::LogicalPlan
[`ExecutionPlan`]: datafusion::physical_plan::ExecutionPlan

Potential uses of this crate:
* Use DataFusion to run Substrait plans created by other systems (e.g. Apache Calcite)
* Use DataFusion to create plans to run on other systems
* Pass query plans over FFI boundaries, such as from Python to Rust
* Pass query plans across node boundaries

# See Also

Substrait does not (yet) support the full range of plans and expressions
that DataFusion offers. See the [datafusion-proto]  crate for a DataFusion
specific format that does support of the full range.

[datafusion-proto]: https://docs.rs/datafusion-proto/latest/datafusion_proto

Note that generated types  such as [`substrait::proto::Plan`] and
[`substrait::proto::Rel`] can be serialized / deserialized to bytes, JSON and
other formats using [prost] and the rest of the Rust protobuf ecosystem.

# Example: Serializing [`LogicalPlan`]s
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# #[tokio::main(flavor = "current_thread")]
# async fn main() -> Result<()>{
# use std::sync::Arc;
# use datafusion::arrow::array::{Int32Array, RecordBatch};
# use datafusion_substrait::logical_plan;
// Create a plan that scans table 't'
let ctx = SessionContext::new();
let batch = RecordBatch::try_from_iter(vec![(
    "x",
    Arc::new(Int32Array::from(vec![42])) as _,
)])?;
ctx.register_batch("t", batch)?;
let df = ctx.sql("SELECT x from t").await?;
let plan = df.into_optimized_plan()?;

// Convert the plan into a substrait (protobuf) Plan
let substrait_plan = logical_plan::producer::to_substrait_plan(&plan, &ctx.state())?;

// Receive a substrait protobuf from somewhere, and turn it into a LogicalPlan
let logical_round_trip =
    logical_plan::consumer::from_substrait_plan(&ctx.state(), &substrait_plan)
        .await?;
let logical_round_trip = ctx.state().optimize(&logical_round_trip)?;
assert_eq!(format!("{:?}", plan), format!("{:?}", logical_round_trip));
# Ok(())
# }
```

Unresolved upstream links (retained, not inferred): `substrait`, ``substrait::proto::Plan``, ``substrait::proto::Rel``, `prost`, `substrait::proto`.
