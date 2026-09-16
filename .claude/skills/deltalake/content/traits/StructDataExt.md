# StructDataExt

`deltalake_core::kernel::arrow::engine_ext::StructDataExt`

```rust
trait StructDataExt
```

Also reachable as `deltalake::kernel::StructDataExt`, `deltalake_core::kernel::StructDataExt`

Prose: [`api/deltalake_core.kernel.arrow.engine_ext.md`](../api/deltalake_core.kernel.arrow.engine_ext.md#structdataext) · records: [`model/deltalake_core.kernel.arrow.engine_ext.json`](../model/deltalake_core.kernel.arrow.engine_ext.json)

## Required

Every implementation must supply these.

```rust
fn field(&self, name: &str) -> Option<&StructField>
fn index_of(&self, name: &str) -> Option<usize>
fn value(&self, index: usize) -> Option<&Scalar>
```

## Implementors (1)

Read one before writing your own.

- `buoyant_kernel::expressions::scalars::StructData`

## Documentation

Extension trait for Kernel's [`StructData`].

StructData is the data structure contained in a Struct scalar.
The exposed API on kernels struct data is very minimal and does not allow
for conveniently probing the fields / values contained within [`StructData`].

This trait therefore adds convenience methods for accessing fields and values.
