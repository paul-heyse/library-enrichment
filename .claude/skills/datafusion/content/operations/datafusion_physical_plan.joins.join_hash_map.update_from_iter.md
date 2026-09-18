# `datafusion_physical_plan::joins::join_hash_map::update_from_iter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.join_hash_map.update_from_iter.json).

<a id="op-daddb359831b329d1c0f45c9"></a>
## update_from_iter

`function` · `datafusion_physical_plan::joins::join_hash_map::update_from_iter` · datafusion-physical-plan 55.1.0

```rust
fn update_from_iter<'a, T>(map: &mut hashbrown::HashTable<(u64, T)>, next: &mut [T], iter: Box<dyn Iterator<Item = (usize, &'a u64)> + Send + 'a>, deleted_offset: usize) where T: Copy + TryFrom<usize> + PartialOrd, <T as TryFrom>::Error: Debug
```

Source: `src/joins/join_hash_map.rs:307`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
