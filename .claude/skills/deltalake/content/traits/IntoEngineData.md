# IntoEngineData

`buoyant_kernel::IntoEngineData`

```rust
trait IntoEngineData
```

Also reachable as `delta_kernel::IntoEngineData`

Prose: [`api/buoyant_kernel.md`](../api/buoyant_kernel.md#intoenginedata) · records: [`model/buoyant_kernel.json`](../model/buoyant_kernel.json)

## Required

Every implementation must supply these.

```rust
fn into_engine_data(self, schema: SchemaRef, engine: &dyn Engine) -> DeltaResult<Box<dyn EngineData>>
```

## Implementors (5)

Read one before writing your own.

- `buoyant_kernel::actions::CommitInfo`
- `buoyant_kernel::actions::DomainMetadata`
- `buoyant_kernel::actions::Metadata`
- `buoyant_kernel::actions::Protocol`
- `buoyant_kernel::actions::SetTransaction`

## Documentation

A trait that allows converting a type into (single-row) EngineData

This is typically used with the `#[derive(IntoEngineData)]` macro
which leverages the traits `ToDataType` and `Into<Scalar>` for struct fields
to convert a struct into EngineData.

# Example
```ignore
# use buoyant_kernel as delta_kernel;
# use std::sync::Arc;
# use delta_kernel_derive::{Schema, IntoEngineData};

#[derive(Schema, IntoEngineData)]
struct MyStruct {
   a: i32,
   b: String,
}

let my_struct = MyStruct { a: 42, b: "Hello".to_string() };
// typically used with ToSchema
let schema = Arc::new(MyStruct::to_schema());
// single-row EngineData
let engine = todo!(); // create an engine
let engine_data = my_struct.into_engine_data(schema, engine);
```
