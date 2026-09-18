# `datafusion_common::utils::proxy::VecAllocExt`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.proxy.VecAllocExt.json).

<a id="op-c8a4168ef25af3e0133eb5c8"></a>
## VecAllocExt

`trait` · `datafusion_common::utils::proxy::VecAllocExt` · datafusion-common 55.1.0

```rust
trait VecAllocExt
```

Source: `src/utils/proxy.rs:24`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Extension trait for [`Vec`] to account for allocations.

Unresolved upstream links (retained, not inferred): ``Vec``.

<a id="op-b75dc9bddce8147ae98e6d73"></a>
## T

`assoc_type` · `datafusion_common::utils::proxy::VecAllocExt::T` · datafusion-common 55.1.0

```rust
T
```

Source: `src/utils/proxy.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Item type.

<a id="op-643e774f73f098e4092f72e9"></a>
## allocated_size

`function` · `datafusion_common::utils::proxy::VecAllocExt::allocated_size` · datafusion-common 55.1.0

```rust
fn allocated_size(&self) -> usize
```

Source: `src/utils/proxy.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Return the amount of memory allocated by this Vec to store elements
(`size_of<T> * capacity`).

Note this calculation is not recursive, and does not include any heap
allocations contained within the Vec's elements. Does not include the
size of `self`

# Example:
```
# use datafusion_common::utils::proxy::VecAllocExt;
let mut vec = Vec::new();
// Push data into the vec and the accounting will be updated to reflect
// memory allocation
vec.push(1);
assert_eq!(vec.allocated_size(), 16); // space for 4 u32s
vec.push(1);
assert_eq!(vec.allocated_size(), 16); // no new allocation needed

// push more data into the vec
for _ in 0..10 {
    vec.push(1);
}
assert_eq!(vec.allocated_size(), 64); // space for 64 now
```

<a id="op-92abaaffe6bd13e8ed109cdf"></a>
## push_accounted

`function` · `datafusion_common::utils::proxy::VecAllocExt::push_accounted` · datafusion-common 55.1.0

```rust
fn push_accounted(&mut self, x: Self::T, accounting: &mut usize)
```

Source: `src/utils/proxy.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

[Push](Vec::push) new element to vector and increase
`accounting` by any newly allocated bytes.

Note that allocation counts  capacity, not size

# Example:
```
# use datafusion_common::utils::proxy::VecAllocExt;
// use allocated to incrementally track how much memory is allocated in the vec
let mut allocated = 0;
let mut vec = Vec::new();
// Push data into the vec and the accounting will be updated to reflect
// memory allocation
vec.push_accounted(1, &mut allocated);
assert_eq!(allocated, 16); // space for 4 u32s
vec.push_accounted(1, &mut allocated);
assert_eq!(allocated, 16); // no new allocation needed

// push more data into the vec
for _ in 0..10 {
    vec.push_accounted(1, &mut allocated);
}
assert_eq!(allocated, 64); // underlying vec has space for 10 u32s
assert_eq!(vec.allocated_size(), 64);
```
# Example with other allocations:
```
# use datafusion_common::utils::proxy::VecAllocExt;
// You can use the same allocated size to track memory allocated by
// another source. For example
let mut allocated = 27;
let mut vec = Vec::new();
vec.push_accounted(1, &mut allocated); // allocates 16 bytes for vec
assert_eq!(allocated, 43); // 16 bytes for vec, 27 bytes for other
```

Unresolved upstream links (retained, not inferred): `Vec::push`.
