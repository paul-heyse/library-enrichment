# `datafusion_execution::cache::cache_manager::meta_heap_bytes`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.cache.cache_manager.meta_heap_bytes.json).

<a id="op-caaa93ed3c9013fc4fe84ea3"></a>
## meta_heap_bytes

`function` · `datafusion_execution::cache::cache_manager::meta_heap_bytes` · datafusion-execution 55.1.0

```rust
fn meta_heap_bytes(object_meta: &object_store::ObjectMeta) -> usize
```

Source: `src/cache/cache_manager.rs:215`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Calculates the number of bytes an [`ObjectMeta`](../operations/object_store.ObjectMeta.md#op-84641755fb7ee613fe92d518) occupies in the heap.
