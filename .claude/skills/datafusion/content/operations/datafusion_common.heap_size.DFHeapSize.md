# `datafusion_common::heap_size::DFHeapSize`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.heap_size.DFHeapSize.json).

<a id="op-11580ab713a1fff40d57dcfe"></a>
## DFHeapSize

`trait` · `datafusion_common::heap_size::DFHeapSize` · datafusion-common 55.1.0

```rust
trait DFHeapSize
```

Source: `src/heap_size.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Trait for computing how many bytes a value has allocated on the heap.

Implementations need to use [`DFHeapSizeCtx`](../operations/datafusion_common.heap_size.DFHeapSizeCtx.md#op-b5ecad0c4c5c6d7a3d4d2fa6) that is pushed through every
nested call. The context records which allocations have already been measured
so they are only counted once.


<a id="op-b1b967e8cc610f96b4a4ba69"></a>
## heap_size

`function` · `datafusion_common::heap_size::DFHeapSize::heap_size` · datafusion-common 55.1.0

```rust
fn heap_size(&self, ctx: &mut DFHeapSizeCtx) -> usize
```

Source: `src/heap_size.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Return the number of bytes this value has allocated on the heap,
including heap memory owned transitively by nested values.

Note that the size of the type itself is not included in the result --
instead, that size is added by the caller (e.g. container).
