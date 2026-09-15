# `datafusion_physical_plan::joins::join_hash_map`

Crate `datafusion-physical-plan` · 7 public items · structured records in [`model/datafusion_physical_plan.joins.join_hash_map.json`](../model/datafusion_physical_plan.joins.join_hash_map.json)

## contain_hashes

`function` · `datafusion_physical_plan::joins::join_hash_map::contain_hashes`

```rust
fn contain_hashes<T>(map: &hashbrown::HashTable<(u64, T)>, hash_values: &[u64]) -> arrow::array::BooleanArray
```

---

## get_matched_indices

`function` · `datafusion_physical_plan::joins::join_hash_map::get_matched_indices`

```rust
fn get_matched_indices<'a, T>(map: &hashbrown::HashTable<(u64, T)>, next: &[T], iter: Box<dyn Iterator<Item = (usize, &'a u64)> + 'a>, deleted_offset: Option<usize>) -> (Vec<u32>, Vec<u64>) where T: Copy + TryFrom<usize> + PartialOrd + Into<u64> + Sub<Output = T>, <T as TryFrom>::Error: Debug
```

---

## get_matched_indices_with_limit_offset

`function` · `datafusion_physical_plan::joins::join_hash_map::get_matched_indices_with_limit_offset`

```rust
fn get_matched_indices_with_limit_offset<T>(map: &hashbrown::HashTable<(u64, T)>, next_chain: &[T], hash_values: &[u64], valid_keys: Option<&arrow::buffer::NullBuffer>, limit: usize, offset: (usize, Option<u64>), input_indices: &mut Vec<u32>, match_indices: &mut Vec<u64>) -> Option<(usize, Option<u64>)> where T: Copy + TryFrom<usize> + PartialOrd + Into<u64> + Sub<Output = T> + ArrowNativeType, <T as TryFrom>::Error: Debug
```

---

## update_from_iter

`function` · `datafusion_physical_plan::joins::join_hash_map::update_from_iter`

```rust
fn update_from_iter<'a, T>(map: &mut hashbrown::HashTable<(u64, T)>, next: &mut [T], iter: Box<dyn Iterator<Item = (usize, &'a u64)> + Send + 'a>, deleted_offset: usize) where T: Copy + TryFrom<usize> + PartialOrd, <T as TryFrom>::Error: Debug
```

---

## JoinHashMapU32

`struct` · `datafusion_physical_plan::joins::join_hash_map::JoinHashMapU32`

```rust
struct JoinHashMapU32
```

**Implements**: `datafusion_physical_plan::joins::join_hash_map::JoinHashMapType`

**Derives**: Debug

**Methods** (1)

```rust
fn with_capacity(cap: usize) -> Self
```

**via `datafusion_physical_plan::joins::join_hash_map::JoinHashMapType`**

```rust
fn contain_hashes(&self, hash_values: &[u64]) -> BooleanArray
fn extend_zero(&mut self, _: usize)
fn get_matched_indices<'a>(&self, iter: Box<dyn Iterator<Item = (usize, &'a u64)> + 'a>, deleted_offset: Option<usize>) -> (Vec<u32>, Vec<u64>)
fn get_matched_indices_with_limit_offset(&self, hash_values: &[u64], valid_keys: Option<&NullBuffer>, limit: usize, offset: (usize, Option<u64>), input_indices: &mut Vec<u32>, match_indices: &mut Vec<u64>) -> Option<(usize, Option<u64>)>
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn update_from_iter<'a>(&mut self, iter: Box<dyn Iterator<Item = (usize, &'a u64)> + Send + 'a>, deleted_offset: usize)
```

---

## JoinHashMapU64

`struct` · `datafusion_physical_plan::joins::join_hash_map::JoinHashMapU64`

```rust
struct JoinHashMapU64
```

**Implements**: `datafusion_physical_plan::joins::join_hash_map::JoinHashMapType`

**Derives**: Debug

**Methods** (1)

```rust
fn with_capacity(cap: usize) -> Self
```

**via `datafusion_physical_plan::joins::join_hash_map::JoinHashMapType`**

```rust
fn contain_hashes(&self, hash_values: &[u64]) -> BooleanArray
fn extend_zero(&mut self, _: usize)
fn get_matched_indices<'a>(&self, iter: Box<dyn Iterator<Item = (usize, &'a u64)> + 'a>, deleted_offset: Option<usize>) -> (Vec<u32>, Vec<u64>)
fn get_matched_indices_with_limit_offset(&self, hash_values: &[u64], valid_keys: Option<&NullBuffer>, limit: usize, offset: (usize, Option<u64>), input_indices: &mut Vec<u32>, match_indices: &mut Vec<u64>) -> Option<(usize, Option<u64>)>
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn update_from_iter<'a>(&mut self, iter: Box<dyn Iterator<Item = (usize, &'a u64)> + Send + 'a>, deleted_offset: usize)
```

---

## JoinHashMapType

`trait` · `datafusion_physical_plan::joins::join_hash_map::JoinHashMapType`

Also reachable as `datafusion_physical_plan::joins::utils::JoinHashMapType`

```rust
trait JoinHashMapType: Send + Sync
```

**Implementors** (2)

- `datafusion_physical_plan::joins::join_hash_map::JoinHashMapU32`
- `datafusion_physical_plan::joins::join_hash_map::JoinHashMapU64`

**Methods** (7)

```rust
fn contain_hashes(&self, hash_values: &[u64]) -> BooleanArray
fn extend_zero(&mut self, len: usize)
fn get_matched_indices<'a>(&self, iter: Box<dyn Iterator<Item = (usize, &'a u64)> + 'a>, deleted_offset: Option<usize>) -> (Vec<u32>, Vec<u64>)
fn get_matched_indices_with_limit_offset(&self, hash_values: &[u64], valid_keys: Option<&NullBuffer>, limit: usize, offset: (usize, Option<u64>), input_indices: &mut Vec<u32>, match_indices: &mut Vec<u64>) -> Option<(usize, Option<u64>)>
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn update_from_iter<'a>(&mut self, iter: Box<dyn Iterator<Item = (usize, &'a u64)> + Send + 'a>, deleted_offset: usize)
```

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

---
