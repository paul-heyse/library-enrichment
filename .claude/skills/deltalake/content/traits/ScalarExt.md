# ScalarExt

`deltalake_core::kernel::scalars::ScalarExt`

```rust
trait ScalarExt: Sized
```

Also reachable as `deltalake::kernel::scalars::ScalarExt`

Prose: [`api/deltalake_core.kernel.scalars.md`](../api/deltalake_core.kernel.scalars.md#scalarext) · records: [`model/deltalake_core.kernel.scalars.json`](../model/deltalake_core.kernel.scalars.json)

## Required

Every implementation must supply these.

```rust
fn from_array(arr: &dyn Array, index: usize) -> Option<Self>
fn serialize(&self) -> String
fn serialize_encoded(&self) -> String
fn to_json(&self) -> Value
```

## Implementors (1)

Read one before writing your own.

- `buoyant_kernel::expressions::scalars::Scalar`

## Documentation

Auxiliary methods for dealing with kernel scalars
