# `deltalake_core::kernel::arrow::engine_ext`

Crate `deltalake-core` · 1 public items · structured records in [`model/deltalake_core.kernel.arrow.engine_ext.json`](../model/deltalake_core.kernel.arrow.engine_ext.json)

## StructDataExt

`trait` · `deltalake_core::kernel::arrow::engine_ext::StructDataExt`

Also reachable as `deltalake::kernel::StructDataExt`, `deltalake_core::kernel::StructDataExt`

```rust
trait StructDataExt
```

**Implementors** (1)

- `buoyant_kernel::expressions::scalars::StructData`

**Methods** (3)

```rust
fn field(&self, name: &str) -> Option<&StructField>
fn index_of(&self, name: &str) -> Option<usize>
fn value(&self, index: usize) -> Option<&Scalar>
```

Extension trait for Kernel's [`StructData`].

StructData is the data structure contained in a Struct scalar.
The exposed API on kernels struct data is very minimal and does not allow
for conveniently probing the fields / values contained within [`StructData`].

This trait therefore adds convenience methods for accessing fields and values.

---
