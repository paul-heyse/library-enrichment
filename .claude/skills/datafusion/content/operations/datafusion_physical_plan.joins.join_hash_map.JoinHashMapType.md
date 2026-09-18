# `datafusion_physical_plan::joins::join_hash_map::JoinHashMapType`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.join_hash_map.JoinHashMapType.json).

<a id="op-011cdd5b55a2842180d68982"></a>
## JoinHashMapType

`trait` · `datafusion_physical_plan::joins::join_hash_map::JoinHashMapType` · datafusion-physical-plan 55.1.0

```rust
trait JoinHashMapType: Send + Sync
```

Source: `src/joins/join_hash_map.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Maps a `u64` hash value based on the build side ["on" values] to a list of indices with this key's value.

By allocating a `HashMap` with capacity for *at least* the number of rows for entries at the build side,
we make sure that we don't have to re-hash the hashmap, which needs access to the key (the hash in this case) value.

E.g. 1 -> [3, 6, 8] indicates that the column values map to rows 3, 6 and 8 for hash value 1
As the key is a hash value, we need to check possible hash collisions in the probe stage
During this stage it might be the case that a row is contained the same hashmap value,
but the values don't match. Those are checked in the `equal_rows_arr` method.

The indices (values) are stored in a separate chained list stored as `Vec<u32>` or `Vec<u64>`.

The first value (+1) is stored in the hashmap, whereas the next value is stored in array at the position value.

The chain can be followed until the value "0" has been reached, meaning the end of the list.
Also see chapter 5.3 of [Balancing vectorized query execution with bandwidth-optimized storage](https://dare.uva.nl/search?identifier=5ccbb60a-38b8-4eeb-858a-e7735dd37487)

# Example

``` text
See the example below:

Insert (10,1)            <-- insert hash value 10 with row index 1
map:
----------
| 10 | 2 |
----------
next:
---------------------
| 0 | 0 | 0 | 0 | 0 |
---------------------
Insert (20,2)
map:
----------
| 10 | 2 |
| 20 | 3 |
----------
next:
---------------------
| 0 | 0 | 0 | 0 | 0 |
---------------------
Insert (10,3)           <-- collision! row index 3 has a hash value of 10 as well
map:
----------
| 10 | 4 |
| 20 | 3 |
----------
next:
---------------------
| 0 | 0 | 0 | 2 | 0 |  <--- hash value 10 maps to 4,2 (which means indices values 3,1)
---------------------
Insert (10,4)          <-- another collision! row index 4 ALSO has a hash value of 10
map:
---------
| 10 | 5 |
| 20 | 3 |
---------
next:
---------------------
| 0 | 0 | 0 | 2 | 4 | <--- hash value 10 maps to 5,4,2 (which means indices values 4,3,1)
---------------------
```

Here we have an option between creating a `JoinHashMapType` using `u32` or `u64` indices
based on how many rows were being used for indices.

At runtime we choose between using `JoinHashMapU32` and `JoinHashMapU64` which oth implement
`JoinHashMapType`.

## Note on use of this trait as a public API
This is currently a public trait but is mainly intended for internal use within DataFusion.
For example, we may compare references to `JoinHashMapType` implementations by pointer equality
rather than deep equality of contents, as deep equality would be expensive and in our usage
patterns it is impossible for two different hash maps to have identical contents in a practical sense.

<a id="op-e9c5d998b600422ffd5c57c4"></a>
## contain_hashes

`function` · `datafusion_physical_plan::joins::join_hash_map::JoinHashMapType::contain_hashes` · datafusion-physical-plan 55.1.0

```rust
fn contain_hashes(&self, hash_values: &[u64]) -> BooleanArray
```

Source: `src/joins/join_hash_map.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns a BooleanArray indicating which of the provided hashes exist in the map.

<a id="op-fbe53216e9c27f010afcb6e6"></a>
## extend_zero

`function` · `datafusion_physical_plan::joins::join_hash_map::JoinHashMapType::extend_zero` · datafusion-physical-plan 55.1.0

```rust
fn extend_zero(&mut self, len: usize)
```

Source: `src/joins/join_hash_map.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad8522e43411c0a34c86f9ec"></a>
## get_matched_indices

`function` · `datafusion_physical_plan::joins::join_hash_map::JoinHashMapType::get_matched_indices` · datafusion-physical-plan 55.1.0

```rust
fn get_matched_indices<'a>(&self, iter: Box<dyn Iterator<Item = (usize, &'a u64)> + 'a>, deleted_offset: Option<usize>) -> (Vec<u32>, Vec<u64>)
```

Source: `src/joins/join_hash_map.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94bbbdb444352844bb6ddd35"></a>
## get_matched_indices_with_limit_offset

`function` · `datafusion_physical_plan::joins::join_hash_map::JoinHashMapType::get_matched_indices_with_limit_offset` · datafusion-physical-plan 55.1.0

```rust
fn get_matched_indices_with_limit_offset(&self, hash_values: &[u64], valid_keys: Option<&NullBuffer>, limit: usize, offset: (usize, Option<u64>), input_indices: &mut Vec<u32>, match_indices: &mut Vec<u64>) -> Option<(usize, Option<u64>)>
```

Source: `src/joins/join_hash_map.rs:124`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Probe rows marked NULL in `valid_keys` are skipped without a lookup:
their key contains a NULL, which cannot match any build row under
`NullEquality::NullEqualsNothing`. Pass `None` when every probe key is
matchable.

<a id="op-04a1541f10f746e636d669d7"></a>
## is_empty

`function` · `datafusion_physical_plan::joins::join_hash_map::JoinHashMapType::is_empty` · datafusion-physical-plan 55.1.0

```rust
fn is_empty(&self) -> bool
```

Source: `src/joins/join_hash_map.rs:138`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns `true` if the join hash map contains no entries.

<a id="op-7c5cb6b9d1f92076a78d95d3"></a>
## len

`function` · `datafusion_physical_plan::joins::join_hash_map::JoinHashMapType::len` · datafusion-physical-plan 55.1.0

```rust
fn len(&self) -> usize
```

Source: `src/joins/join_hash_map.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the number of entries in the join hash map.

<a id="op-fe2b8907033a3b77de5928a2"></a>
## update_from_iter

`function` · `datafusion_physical_plan::joins::join_hash_map::JoinHashMapType::update_from_iter` · datafusion-physical-plan 55.1.0

```rust
fn update_from_iter<'a>(&mut self, iter: Box<dyn Iterator<Item = (usize, &'a u64)> + Send + 'a>, deleted_offset: usize)
```

Source: `src/joins/join_hash_map.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
