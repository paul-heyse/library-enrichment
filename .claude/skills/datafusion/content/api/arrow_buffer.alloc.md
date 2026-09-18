# `arrow_buffer::alloc`

Crate `arrow-buffer` · 1 public items · structured records in [`model/arrow_buffer.alloc.json`](../model/arrow_buffer.alloc.json)

## Allocation

`trait` · `arrow_buffer::alloc::Allocation`

Also reachable as `arrow::alloc::Allocation`

```rust
trait Allocation: RefUnwindSafe + Send + Sync
```

[Full member, field, variant and typed contracts](../operations/arrow_buffer.alloc.Allocation.md).


The owner of an allocation.
The trait implementation is responsible for dropping the allocations once no more references exist.

---
