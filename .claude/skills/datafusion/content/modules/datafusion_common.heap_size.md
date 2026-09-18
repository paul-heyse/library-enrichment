# `datafusion_common::heap_size`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.heap_size.json).

<a id="op-ed1bb92de09dafa6296c57b0"></a>
## heap_size

`module` · `datafusion_common::heap_size` · datafusion-common 55.1.0

```rust
mod heap_size
```

Source: `src/heap_size.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Estimating the heap-allocated memory owned by a value.

The [`DFHeapSize`](../operations/datafusion_common.heap_size.DFHeapSize.md#op-11580ab713a1fff40d57dcfe) trait reports the number of bytes a value owns on the
heap, **excluding** the stack size of the value itself.

Implementations need to use [`DFHeapSizeCtx`](../operations/datafusion_common.heap_size.DFHeapSizeCtx.md#op-b5ecad0c4c5c6d7a3d4d2fa6) that is pushed through every
nested call. The context records which allocations have already been measured
so they are only counted once.

# Example

```
use datafusion_common::heap_size::{DFHeapSize, DFHeapSizeCtx};
use std::sync::Arc;

let shared: Arc<String> = Arc::new("hello".to_string());
let alias = Arc::clone(&shared);

let mut ctx = DFHeapSizeCtx::default();
// The shared allocation is counted once even when reached twice.
let total = shared.heap_size(&mut ctx) + alias.heap_size(&mut ctx);
assert_eq!(total, shared.heap_size(&mut DFHeapSizeCtx::default()));
```
