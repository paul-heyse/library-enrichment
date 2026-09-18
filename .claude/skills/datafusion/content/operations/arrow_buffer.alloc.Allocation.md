# `arrow_buffer::alloc::Allocation`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.alloc.Allocation.json).

<a id="op-4d4e5ee0400b139a1788a1a4"></a>
## Allocation

`trait` · `arrow_buffer::alloc::Allocation` · arrow-buffer 59.3.0

```rust
trait Allocation: RefUnwindSafe + Send + Sync
```

Source: `src/alloc/mod.rs:31`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

The owner of an allocation.
The trait implementation is responsible for dropping the allocations once no more references exist.
