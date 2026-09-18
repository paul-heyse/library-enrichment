# `datafusion_common::utils::memory::get_record_batch_memory_size`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.memory.get_record_batch_memory_size.json).

<a id="op-96c226571bfad24b0ab66e2c"></a>
## get_record_batch_memory_size

`function` · `datafusion_common::utils::memory::get_record_batch_memory_size` · datafusion-common 55.1.0

```rust
fn get_record_batch_memory_size(batch: &arrow::record_batch::RecordBatch) -> usize
```

Source: `src/utils/memory.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Calculate total used memory of this batch.

This function is used to estimate the physical memory usage of the `RecordBatch`.
It only counts the memory of large data `Buffer`s, and ignores metadata like
types and pointers.
The implementation will add up all unique `Buffer`'s memory
size, due to:
- The data pointer inside `Buffer` are memory regions returned by global memory
  allocator, those regions can't have overlap.
- The actual used range of `ArrayRef`s inside `RecordBatch` can have overlap
  or reuse the same `Buffer`. For example: taking a slice from `Array`.

Example:
For a `RecordBatch` with two columns: `col1` and `col2`, two columns are pointing
to a sub-region of the same buffer.

{xxxxxxxxxxxxxxxxxxx} <--- buffer
      ^    ^  ^    ^
      |    |  |    |
col1->{    }  |    |
col2--------->{    }

In the above case, `get_record_batch_memory_size` will return the size of
the buffer, instead of the sum of `col1` and `col2`'s actual memory size.

Note: Current `RecordBatch`.get_array_memory_size()` will double count the
buffer memory size if multiple arrays within the batch are sharing the same
`Buffer`. This method provides temporary fix until the issue is resolved:
<https://github.com/apache/arrow-rs/issues/6439>
