# `deltalake_core::kernel::scalars`

Crate `deltalake-core` · 1 public items · structured records in [`model/deltalake_core.kernel.scalars.json`](../model/deltalake_core.kernel.scalars.json)

## ScalarExt

`trait` · `deltalake_core::kernel::scalars::ScalarExt`
[Full member contracts, output types and access classification](../operations/deltalake_core.kernel.scalars.ScalarExt.md)

Also reachable as `deltalake::kernel::scalars::ScalarExt`

```rust
trait ScalarExt: Sized
```

**Implementors** (1)

- `buoyant_kernel::expressions::scalars::Scalar`

**Methods** (4)

```rust
fn from_array(arr: &dyn Array, index: usize) -> Option<Self>
fn serialize(&self) -> String
fn serialize_encoded(&self) -> String
fn to_json(&self) -> Value
```

Auxiliary methods for dealing with kernel scalars

---
