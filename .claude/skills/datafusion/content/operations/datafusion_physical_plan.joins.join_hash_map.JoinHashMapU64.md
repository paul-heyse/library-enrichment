# `datafusion_physical_plan::joins::join_hash_map::JoinHashMapU64`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.join_hash_map.JoinHashMapU64.json).

<a id="op-ad0687af5039db36567ebc69"></a>
## JoinHashMapU64

`struct` · `datafusion_physical_plan::joins::join_hash_map::JoinHashMapU64` · datafusion-physical-plan 55.1.0

```rust
struct JoinHashMapU64
```

Source: `src/joins/join_hash_map.rs:224`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fcd7d1a6f56e055ed516fb44"></a>
## contain_hashes

`function` · `datafusion_physical_plan::joins::join_hash_map::JoinHashMapU64::contain_hashes` · datafusion-physical-plan 55.1.0

```rust
fn contain_hashes(&self, hash_values: &[u64]) -> BooleanArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapU64", "path": "JoinHashMapU64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [302, 2], "filename": "src/joins/join_hash_map.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapType", "path": "JoinHashMapType"}, "trait_path": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapType"}`

Source: `src/joins/join_hash_map.rs:291`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f044c5aab9d26e116d541ee"></a>
## extend_zero

`function` · `datafusion_physical_plan::joins::join_hash_map::JoinHashMapU64::extend_zero` · datafusion-physical-plan 55.1.0

```rust
fn extend_zero(&mut self, _: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapU64", "path": "JoinHashMapU64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [302, 2], "filename": "src/joins/join_hash_map.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapType", "path": "JoinHashMapType"}, "trait_path": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapType"}`

Source: `src/joins/join_hash_map.rs:252`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64fc04ae20a356c17b2c3237"></a>
## fmt

`function` · `datafusion_physical_plan::joins::join_hash_map::JoinHashMapU64::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapU64", "path": "JoinHashMapU64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [245, 1], "end": [249, 2], "filename": "src/joins/join_hash_map.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/joins/join_hash_map.rs:246`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39080423c22db5bd78dbe9ca"></a>
## get_matched_indices

`function` · `datafusion_physical_plan::joins::join_hash_map::JoinHashMapU64::get_matched_indices` · datafusion-physical-plan 55.1.0

```rust
fn get_matched_indices<'a>(&self, iter: Box<dyn Iterator<Item = (usize, &'a u64)> + 'a>, deleted_offset: Option<usize>) -> (Vec<u32>, Vec<u64>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapU64", "path": "JoinHashMapU64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [302, 2], "filename": "src/joins/join_hash_map.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapType", "path": "JoinHashMapType"}, "trait_path": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapType"}`

Source: `src/joins/join_hash_map.rs:262`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11a573f44c32fb97558fc24b"></a>
## get_matched_indices_with_limit_offset

`function` · `datafusion_physical_plan::joins::join_hash_map::JoinHashMapU64::get_matched_indices_with_limit_offset` · datafusion-physical-plan 55.1.0

```rust
fn get_matched_indices_with_limit_offset(&self, hash_values: &[u64], valid_keys: Option<&NullBuffer>, limit: usize, offset: (usize, Option<u64>), input_indices: &mut Vec<u32>, match_indices: &mut Vec<u64>) -> Option<(usize, Option<u64>)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapU64", "path": "JoinHashMapU64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [302, 2], "filename": "src/joins/join_hash_map.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapType", "path": "JoinHashMapType"}, "trait_path": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapType"}`

Source: `src/joins/join_hash_map.rs:270`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cff49bbcad70a88cc50278a8"></a>
## is_empty

`function` · `datafusion_physical_plan::joins::join_hash_map::JoinHashMapU64::is_empty` · datafusion-physical-plan 55.1.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapU64", "path": "JoinHashMapU64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [302, 2], "filename": "src/joins/join_hash_map.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapType", "path": "JoinHashMapType"}, "trait_path": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapType"}`

Source: `src/joins/join_hash_map.rs:295`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2cf27a4294534e2371b657c"></a>
## len

`function` · `datafusion_physical_plan::joins::join_hash_map::JoinHashMapU64::len` · datafusion-physical-plan 55.1.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapU64", "path": "JoinHashMapU64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [302, 2], "filename": "src/joins/join_hash_map.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapType", "path": "JoinHashMapType"}, "trait_path": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapType"}`

Source: `src/joins/join_hash_map.rs:299`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30eb87d05c3158d272a28b07"></a>
## update_from_iter

`function` · `datafusion_physical_plan::joins::join_hash_map::JoinHashMapU64::update_from_iter` · datafusion-physical-plan 55.1.0

```rust
fn update_from_iter<'a>(&mut self, iter: Box<dyn Iterator<Item = (usize, &'a u64)> + Send + 'a>, deleted_offset: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapU64", "path": "JoinHashMapU64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [302, 2], "filename": "src/joins/join_hash_map.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapType", "path": "JoinHashMapType"}, "trait_path": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapType"}`

Source: `src/joins/join_hash_map.rs:254`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6514f1c2ce2ebfbc35fe04cb"></a>
## with_capacity

`function` · `datafusion_physical_plan::joins::join_hash_map::JoinHashMapU64::with_capacity` · datafusion-physical-plan 55.1.0

```rust
fn with_capacity(cap: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::join_hash_map::JoinHashMapU64", "path": "JoinHashMapU64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [231, 1], "end": [243, 2], "filename": "src/joins/join_hash_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/join_hash_map.rs:237`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
