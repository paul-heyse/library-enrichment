# `buoyant_kernel::IntoEngineData`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.IntoEngineData.json).

<a id="op-d602e6871a04598f54f1bb34"></a>
## IntoEngineData

`trait` · `buoyant_kernel::IntoEngineData` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait IntoEngineData
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L584).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:584`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

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

<a id="op-59dc660cb9f7262fbbbee961"></a>
## into_engine_data

`function` · `buoyant_kernel::IntoEngineData::into_engine_data` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_engine_data(self, schema: SchemaRef, engine: &dyn Engine) -> DeltaResult<Box<dyn EngineData>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L586).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:586`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Consume this type to produce a single-row EngineData using the provided schema.
