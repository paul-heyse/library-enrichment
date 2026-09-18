# `datafusion_common::utils::memory::estimate_memory_size`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.memory.estimate_memory_size.json).

<a id="op-79acd21252d94afe33c7aeca"></a>
## estimate_memory_size

`function` · `datafusion_common::utils::memory::estimate_memory_size` · datafusion-common 55.1.0

```rust
fn estimate_memory_size<T>(num_elements: usize, fixed_size: usize) -> Result<usize>
```

Source: `src/utils/memory.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

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
- If the estimation overflows, we return a [`crate::error::DataFusionError`](../operations/datafusion_common.error.DataFusionError.md#op-d335eb18d3c98363748854bb)

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
