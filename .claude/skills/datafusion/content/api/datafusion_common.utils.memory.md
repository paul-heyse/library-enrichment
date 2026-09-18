# `datafusion_common::utils::memory`

Crate `datafusion-common` · 3 public items · structured records in [`model/datafusion_common.utils.memory.json`](../model/datafusion_common.utils.memory.json)

## estimate_memory_size

`function` · `datafusion_common::utils::memory::estimate_memory_size`

```rust
fn estimate_memory_size<T>(num_elements: usize, fixed_size: usize) -> Result<usize>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.utils.memory.estimate_memory_size.md).


Estimates the memory size required for a hash table prior to allocation.

# Parameters
- `num_elements`: The number of elements expected in the hash table.
- `fixed_size`: A fixed overhead size associated with the collection
  (e.g., HashSet or HashTable).
- `T`: The type of elements stored in the hash table.

# Details
This function calculates the estimated memory size by considering:
- An overestimation of buckets to keep approximately 1/8 of them empty.
- The total memory size is computed as:
  - The size of each entry (`T`) multiplied by the estimated number of
    buckets.
  - One byte overhead for each bucket.
  - The fixed size overhead of the collection.
- If the estimation overflows, we return a [`crate::error::DataFusionError`]

# Examples
---

## From within a struct

```rust
# use datafusion_common::utils::memory::estimate_memory_size;
# use datafusion_common::Result;

struct MyStruct<T> {
    values: Vec<T>,
    other_data: usize,
}

impl<T> MyStruct<T> {
    fn size(&self) -> Result<usize> {
        let num_elements = self.values.len();
        let fixed_size =
            std::mem::size_of_val(self) + std::mem::size_of_val(&self.values);

        estimate_memory_size::<T>(num_elements, fixed_size)
    }
}
```
---
## With a simple collection

```rust
# use datafusion_common::utils::memory::estimate_memory_size;
# use std::collections::HashMap;

let num_rows = 100;
let fixed_size = std::mem::size_of::<HashMap<u64, u64>>();
let estimated_hashtable_size =
    estimate_memory_size::<(u64, u64)>(num_rows, fixed_size)
        .expect("Size estimation failed");
```

---

## get_record_batch_memory_size

`function` · `datafusion_common::utils::memory::get_record_batch_memory_size`

Also reachable as `datafusion_physical_plan::spill::get_record_batch_memory_size`

```rust
fn get_record_batch_memory_size(batch: &arrow::record_batch::RecordBatch) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.utils.memory.get_record_batch_memory_size.md).


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

---

## RecordBatchMemoryCounter

`struct` · `datafusion_common::utils::memory::RecordBatchMemoryCounter`

```rust
struct RecordBatchMemoryCounter
```

**Derives**: Debug, Default

**Methods** (3)

```rust
fn count_batch(&mut self, batch: &RecordBatch) -> usize
fn memory_usage(&self) -> usize
fn new() -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.utils.memory.RecordBatchMemoryCounter.md).


Tracks the memory used by a sequence of [`RecordBatch`]es that may share
underlying buffers, counting each buffer exactly once.

Use this instead of [`get_record_batch_memory_size`] to account for the
total memory of a sequence of batches, e.g. when buffering the batches of
an input stream. Such batches can share buffers (for example, operators
like aggregates emit one large batch as multiple zero-copy slices), and
calling [`get_record_batch_memory_size`] per batch counts the shared
buffers once per batch, while this counter counts them exactly once. A
batch's buffers are kept alive by the batch even when only a sub-range is
referenced, so counting unique buffers in full reflects the memory the
batches actually retain.

---
