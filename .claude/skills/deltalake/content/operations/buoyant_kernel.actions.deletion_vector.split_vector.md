# `buoyant_kernel::actions::deletion_vector::split_vector`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.actions.deletion_vector.split_vector.json).

<a id="op-42b5a35c88ad1014c772d4a3"></a>
## split_vector

`function` · `buoyant_kernel::actions::deletion_vector::split_vector` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn split_vector(vector: Option<&mut Vec<bool>>, split_index: usize, extend: Option<bool>) -> Option<Vec<bool>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector.rs#L524).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector.rs:524`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

helper function to split an `Option<Vec<bool>>`. Because deletion vectors apply to a whole file,
but parquet readers can chunk the file, there is a need to split the vector up.
If the passed vector is Some(vector):
  - If `split_index < vector.len()`, split `vector` at `split_index`. The passed vector is
    modified in place, and the split off component is returned.
  - If `split_index` >= vector.len()` will return None. If `extend` is Some(b), the passed
    vector will be extended with `b` to have a length of `split_index`. If `extend` is `None`,
    do nothing and return `None`
If the passed `vector` is `None`, do nothing and return None
