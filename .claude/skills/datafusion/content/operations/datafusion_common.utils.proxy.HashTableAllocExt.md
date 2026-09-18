# `datafusion_common::utils::proxy::HashTableAllocExt`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.proxy.HashTableAllocExt.json).

<a id="op-78bfc3846dcbc2f689031070"></a>
## HashTableAllocExt

`trait` · `datafusion_common::utils::proxy::HashTableAllocExt` · datafusion-common 55.1.0

```rust
trait HashTableAllocExt
```

Source: `src/utils/proxy.rs:115`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Extension trait for hash browns [`HashTable`] to account for allocations.

Unresolved upstream links (retained, not inferred): ``HashTable``.

<a id="op-f92f682ae9dbd49dbca55527"></a>
## T

`assoc_type` · `datafusion_common::utils::proxy::HashTableAllocExt::T` · datafusion-common 55.1.0

```rust
T
```

Source: `src/utils/proxy.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Item type.

<a id="op-4086f509b06ae3a3be9cd583"></a>
## insert_accounted

`function` · `datafusion_common::utils::proxy::HashTableAllocExt::insert_accounted` · datafusion-common 55.1.0

```rust
fn insert_accounted(&mut self, x: Self::T, hasher: impl Fn(&Self::T) -> u64, accounting: &mut usize)
```

Source: `src/utils/proxy.rs:144`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Insert new element into table and increase
`accounting` by any newly allocated bytes.

Returns the bucket where the element was inserted.
Note that allocation counts capacity, not size.
Panics:
    Assumes the element is not already present, and may panic if it does

# Example:
```
# use datafusion_common::utils::proxy::HashTableAllocExt;
# use hashbrown::hash_table::HashTable;
let mut table = HashTable::new();
let mut allocated = 0;
let hash_fn = |x: &u32| (*x as u64) % 1000;
// pretend 0x3117 is the hash value for 1
table.insert_accounted(1, hash_fn, &mut allocated);
assert_eq!(allocated, 64);

// insert more values
for i in 2..100 {
    table.insert_accounted(i, hash_fn, &mut allocated);
}
assert_eq!(allocated, 400);
```
