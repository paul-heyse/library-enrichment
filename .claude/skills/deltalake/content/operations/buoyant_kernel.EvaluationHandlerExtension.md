# `buoyant_kernel::EvaluationHandlerExtension`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.EvaluationHandlerExtension.json).

<a id="op-4b3685023c55c6d600fc99bf"></a>
## EvaluationHandlerExtension

`trait` · `buoyant_kernel::EvaluationHandlerExtension` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait EvaluationHandlerExtension: EvaluationHandler
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L532).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:532`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Internal trait to allow us to have a private `create_one` API that's implemented for all
EvaluationHandlers.

<a id="op-d70d4585bec9a64f7b19ee66"></a>
## create_one

`function` · `buoyant_kernel::EvaluationHandlerExtension::create_one` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn create_one(&self, schema: SchemaRef, values: &[Scalar]) -> DeltaResult<Box<dyn EngineData>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L538).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:538`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a single-row [`EngineData`](../operations/buoyant_kernel.engine_data.EngineData.md#op-f9036ab52623c01f96e1f809) by applying the given schema to the leaf-values given in
`values`.
