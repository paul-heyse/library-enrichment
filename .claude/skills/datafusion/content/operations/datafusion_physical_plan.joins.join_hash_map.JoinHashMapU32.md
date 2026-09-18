# `datafusion_physical_plan::joins::join_hash_map::JoinHashMapU32`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.join_hash_map.JoinHashMapU32.json).

<a id="op-f31909db3be3248d3f6cc657"></a>
## JoinHashMapU32

`struct` · `datafusion_physical_plan::joins::join_hash_map::JoinHashMapU32` · datafusion-physical-plan 55.1.0

```rust
struct JoinHashMapU32
```

Source: `src/joins/join_hash_map.rs:144`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dadf5137bdc609def2a099e8"></a>
## contain_hashes

`function` · `datafusion_physical_plan::joins::join_hash_map::JoinHashMapU32::contain_hashes` · datafusion-physical-plan 55.1.0

```rust
fn contain_hashes(&self, hash_values: &[u64]) -> BooleanArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapU32", "path": "JoinHashMapU32"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 1], "end": [222, 2], "filename": "src/joins/join_hash_map.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapType", "path": "JoinHashMapType"}, "trait_path": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapType"}`

Source: `src/joins/join_hash_map.rs:211`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d67be60cd38b6eb52e867b7"></a>
## extend_zero

`function` · `datafusion_physical_plan::joins::join_hash_map::JoinHashMapU32::extend_zero` · datafusion-physical-plan 55.1.0

```rust
fn extend_zero(&mut self, _: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapU32", "path": "JoinHashMapU32"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 1], "end": [222, 2], "filename": "src/joins/join_hash_map.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapType", "path": "JoinHashMapType"}, "trait_path": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapType"}`

Source: `src/joins/join_hash_map.rs:172`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b0cd60aed24b7a252c905c0"></a>
## fmt

`function` · `datafusion_physical_plan::joins::join_hash_map::JoinHashMapU32::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapU32", "path": "JoinHashMapU32"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 1], "end": [169, 2], "filename": "src/joins/join_hash_map.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/joins/join_hash_map.rs:166`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-975d710f46919b6ab7695fd9"></a>
## get_matched_indices

`function` · `datafusion_physical_plan::joins::join_hash_map::JoinHashMapU32::get_matched_indices` · datafusion-physical-plan 55.1.0

```rust
fn get_matched_indices<'a>(&self, iter: Box<dyn Iterator<Item = (usize, &'a u64)> + 'a>, deleted_offset: Option<usize>) -> (Vec<u32>, Vec<u64>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapU32", "path": "JoinHashMapU32"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 1], "end": [222, 2], "filename": "src/joins/join_hash_map.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapType", "path": "JoinHashMapType"}, "trait_path": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapType"}`

Source: `src/joins/join_hash_map.rs:182`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3335ff2b181ed5795cd302d"></a>
## get_matched_indices_with_limit_offset

`function` · `datafusion_physical_plan::joins::join_hash_map::JoinHashMapU32::get_matched_indices_with_limit_offset` · datafusion-physical-plan 55.1.0

```rust
fn get_matched_indices_with_limit_offset(&self, hash_values: &[u64], valid_keys: Option<&NullBuffer>, limit: usize, offset: (usize, Option<u64>), input_indices: &mut Vec<u32>, match_indices: &mut Vec<u64>) -> Option<(usize, Option<u64>)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapU32", "path": "JoinHashMapU32"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 1], "end": [222, 2], "filename": "src/joins/join_hash_map.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapType", "path": "JoinHashMapType"}, "trait_path": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapType"}`

Source: `src/joins/join_hash_map.rs:190`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89090eee64b853c8e8a4a9ea"></a>
## is_empty

`function` · `datafusion_physical_plan::joins::join_hash_map::JoinHashMapU32::is_empty` · datafusion-physical-plan 55.1.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapU32", "path": "JoinHashMapU32"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 1], "end": [222, 2], "filename": "src/joins/join_hash_map.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapType", "path": "JoinHashMapType"}, "trait_path": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapType"}`

Source: `src/joins/join_hash_map.rs:215`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6478a5f86207f97dff7bd979"></a>
## len

`function` · `datafusion_physical_plan::joins::join_hash_map::JoinHashMapU32::len` · datafusion-physical-plan 55.1.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapU32", "path": "JoinHashMapU32"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 1], "end": [222, 2], "filename": "src/joins/join_hash_map.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapType", "path": "JoinHashMapType"}, "trait_path": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapType"}`

Source: `src/joins/join_hash_map.rs:219`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11e395079f564883c715c0f0"></a>
## update_from_iter

`function` · `datafusion_physical_plan::joins::join_hash_map::JoinHashMapU32::update_from_iter` · datafusion-physical-plan 55.1.0

```rust
fn update_from_iter<'a>(&mut self, iter: Box<dyn Iterator<Item = (usize, &'a u64)> + Send + 'a>, deleted_offset: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapU32", "path": "JoinHashMapU32"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 1], "end": [222, 2], "filename": "src/joins/join_hash_map.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapType", "path": "JoinHashMapType"}, "trait_path": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapType"}`

Source: `src/joins/join_hash_map.rs:174`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10353e516aedcb1ace4e2568"></a>
## with_capacity

`function` · `datafusion_physical_plan::joins::join_hash_map::JoinHashMapU32::with_capacity` · datafusion-physical-plan 55.1.0

```rust
fn with_capacity(cap: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapU32", "path": "JoinHashMapU32"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [151, 1], "end": [163, 2], "filename": "src/joins/join_hash_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/join_hash_map.rs:157`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
