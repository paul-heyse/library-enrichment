# `arrow::util::bench_util`

Crate `arrow` · 36 public items · structured records in [`model/arrow.util.bench_util.json`](../model/arrow.util.bench_util.json)

## create_array_for_type

`function` · `arrow::util::bench_util::create_array_for_type`

```rust
fn create_array_for_type(data_type: &DataType, size: usize, null_density: f32) -> ArrayRef
```

Creates a random array for the given [`DataType`], `size`, and `null_density`.

Useful for building arrays and record batches in benchmarks without
repeating per-type construction logic. Panics on unsupported types.

---

## create_binary_array

`function` · `arrow::util::bench_util::create_binary_array`

```rust
fn create_binary_array<Offset: OffsetSizeTrait>(size: usize, null_density: f32) -> GenericBinaryArray<Offset>
```

Creates an random (but fixed-seeded) binary array of a given size and null density

---

## create_binary_array_with_len_range_and_prefix_and_seed

`function` · `arrow::util::bench_util::create_binary_array_with_len_range_and_prefix_and_seed`

```rust
fn create_binary_array_with_len_range_and_prefix_and_seed<Offset: OffsetSizeTrait>(size: usize, null_density: f32, min_len: usize, max_len: usize, prefix: &[u8], seed: u64) -> GenericBinaryArray<Offset>
```

Creates a random [`GenericBinaryArray`] of a given `size` and `null_density`
filling it with random bytes with lengths in the specified range,
all starting with the provided `prefix`, generated using the provided `seed`.

---

## create_binary_array_with_seed

`function` · `arrow::util::bench_util::create_binary_array_with_seed`

```rust
fn create_binary_array_with_seed<Offset: OffsetSizeTrait>(size: usize, null_density: f32, bytes_seed: u64, bytes_length_seed: u64) -> GenericBinaryArray<Offset>
```

Creates a random [`GenericBinaryArray`] of a given `size` and `null_density`
filling it with random bytes, generated using the provided `seed`s.

the `bytes_seed` is used to seed the RNG for generating the byte values,
while the `bytes_length_seed` is used to seed the RNG for generating the length of an array item

These values can be the same as they are used to seed different RNGs internally.

---

## create_boolean_array

`function` · `arrow::util::bench_util::create_boolean_array`

```rust
fn create_boolean_array(size: usize, null_density: f32, true_density: f32) -> BooleanArray where rand::distr::StandardUniform: Distribution<bool>
```

Creates a random (but fixed-seeded) array of a given size and null density

---

## create_boolean_array_with_seed

`function` · `arrow::util::bench_util::create_boolean_array_with_seed`

```rust
fn create_boolean_array_with_seed(size: usize, null_density: f32, true_density: f32, seed: u64) -> BooleanArray where rand::distr::StandardUniform: Distribution<bool>
```

Creates a random array of a given size and null density based on the provided seed

---

## create_dict_from_values

`function` · `arrow::util::bench_util::create_dict_from_values`

```rust
fn create_dict_from_values<K>(size: usize, null_density: f32, values: &dyn Array) -> DictionaryArray<K> where K: ArrowDictionaryKeyType, rand::distr::StandardUniform: Distribution<K::Native>, K::Native: SampleUniform
```

Creates a random (but fixed-seeded) dictionary array of a given size and null density
with the provided values array

---

## create_f16_array

`function` · `arrow::util::bench_util::create_f16_array`

```rust
fn create_f16_array(size: usize, nan_density: f32) -> Float16Array
```

Creates a random (but fixed-seeded) f16 array of a given size and nan-value density

---

## create_f32_array

`function` · `arrow::util::bench_util::create_f32_array`

```rust
fn create_f32_array(size: usize, nan_density: f32) -> Float32Array
```

Creates a random (but fixed-seeded) f32 array of a given size and nan-value density

---

## create_f64_array

`function` · `arrow::util::bench_util::create_f64_array`

```rust
fn create_f64_array(size: usize, nan_density: f32) -> Float64Array
```

Creates a random (but fixed-seeded) f64 array of a given size and nan-value density

---

## create_f64_array_with_seed

`function` · `arrow::util::bench_util::create_f64_array_with_seed`

```rust
fn create_f64_array_with_seed(size: usize, nan_density: f32, seed: u64) -> Float64Array
```

Creates a random f64 array of a given size and nan-value density based on a given seed

---

## create_fsb_array

`function` · `arrow::util::bench_util::create_fsb_array`

```rust
fn create_fsb_array(size: usize, null_density: f32, value_len: usize) -> FixedSizeBinaryArray
```

Creates an random (but fixed-seeded) array of a given size and null density

---

## create_longer_string_array_with_same_prefix

`function` · `arrow::util::bench_util::create_longer_string_array_with_same_prefix`

```rust
fn create_longer_string_array_with_same_prefix<Offset: OffsetSizeTrait>(size: usize, null_density: f32) -> GenericStringArray<Offset>
```

Creates longer string array with same prefix, the prefix should be larger than 4 bytes,
and the string length should be larger than 12 bytes
so that we can compare the performance with StringViewArray, because StringViewArray has 4 bytes inline for view

---

## create_longer_string_view_array_with_same_prefix

`function` · `arrow::util::bench_util::create_longer_string_view_array_with_same_prefix`

```rust
fn create_longer_string_view_array_with_same_prefix(size: usize, null_density: f32) -> StringViewArray
```

Creates longer string view array with same prefix, the prefix should be larger than 4 bytes,
and the string length should be larger than 12 bytes
so that we can compare the StringArray performance with StringViewArray, because StringViewArray has 4 bytes inline for view

---

## create_month_day_nano_array_with_seed

`function` · `arrow::util::bench_util::create_month_day_nano_array_with_seed`

```rust
fn create_month_day_nano_array_with_seed(size: usize, null_density: f32, seed: u64) -> IntervalMonthDayNanoArray
```

Creates a [`PrimitiveArray`] of a given `size` and `null_density`
filling it with random [`IntervalMonthDayNano`] generated using the provided `seed`.

---

## create_primitive_array

`function` · `arrow::util::bench_util::create_primitive_array`

```rust
fn create_primitive_array<T>(size: usize, null_density: f32) -> PrimitiveArray<T> where T: ArrowPrimitiveType, rand::distr::StandardUniform: Distribution<T::Native>
```

Creates an random (but fixed-seeded) array of a given size and null density

---

## create_primitive_array_range

`function` · `arrow::util::bench_util::create_primitive_array_range`

```rust
fn create_primitive_array_range<T>(size: usize, null_density: f32, value_range: std::ops::Range<T::Native>) -> PrimitiveArray<T> where T: ArrowPrimitiveType, rand::distr::StandardUniform: Distribution<T::Native>, T::Native: SampleUniform
```

Creates an random (but fixed-seeded) array of a given size and null density,
all the values located in the given range

---

## create_primitive_array_with_seed

`function` · `arrow::util::bench_util::create_primitive_array_with_seed`

```rust
fn create_primitive_array_with_seed<T>(size: usize, null_density: f32, seed: u64) -> PrimitiveArray<T> where T: ArrowPrimitiveType, rand::distr::StandardUniform: Distribution<T::Native>
```

Creates a [`PrimitiveArray`] of a given `size` and `null_density`
filling it with random numbers generated using the provided `seed`.

---

## create_primitive_fixed_size_list_array

`function` · `arrow::util::bench_util::create_primitive_fixed_size_list_array`

```rust
fn create_primitive_fixed_size_list_array<T>(size: usize, null_density: f32, value_null_density: f32, list_size: i32) -> FixedSizeListArray where T: ArrowPrimitiveType, rand::distr::StandardUniform: Distribution<T::Native>
```

Create a FixedSizeList array of primitive values

Arguments:
- `size`: number of fixed-size lists in the array
- `null_density`: density of nulls in the fixed-size list array (row-level nulls)
- `value_null_density`: density of nulls in the primitive values inside each list
- `list_size`: fixed size of each list element

---

## create_primitive_list_array

`function` · `arrow::util::bench_util::create_primitive_list_array`

```rust
fn create_primitive_list_array<O, T>(size: usize, null_density: f32, list_null_density: f32, max_list_size: usize) -> GenericListArray<O> where O: OffsetSizeTrait, T: ArrowPrimitiveType, rand::distr::StandardUniform: Distribution<T::Native>
```

Create a List/LargeList Array of primitive values using a fixed seed

See [`create_primitive_list_array_with_seed`] for details on arguments.

---

## create_primitive_list_array_with_seed

`function` · `arrow::util::bench_util::create_primitive_list_array_with_seed`

```rust
fn create_primitive_list_array_with_seed<O, T>(size: usize, null_density: f32, list_null_density: f32, max_list_size: usize, seed: u64) -> GenericListArray<O> where O: OffsetSizeTrait, T: ArrowPrimitiveType, rand::distr::StandardUniform: Distribution<T::Native>
```

Create a List/LargeList Array  of primitive values

Arguments:
- `size`: number of lists in the array
- `null_density`: density of nulls in the list array
- `list_null_density`: density of nulls in the primitive arrays inside the lists
- `max_list_size`: maximum size of each list (actual size is random between 0 and max_list_size)
- `seed`: seed for the random number generator

---

## create_primitive_list_view_array

`function` · `arrow::util::bench_util::create_primitive_list_view_array`

```rust
fn create_primitive_list_view_array<O, T>(size: usize, null_density: f32, list_null_density: f32, max_list_size: usize) -> GenericListViewArray<O> where T: ArrowPrimitiveType, rand::distr::StandardUniform: Distribution<T::Native>, O: OffsetSizeTrait
```

Create a ListViewArray of primitive values using a fixed seed

See [`create_primitive_list_array_with_seed`] for details on arguments.

---

## create_primitive_run_array

`function` · `arrow::util::bench_util::create_primitive_run_array`

```rust
fn create_primitive_run_array<R: RunEndIndexType, V: ArrowPrimitiveType>(logical_array_len: usize, physical_array_len: usize) -> RunArray<R>
```

Create primitive run array for given logical and physical array lengths

---

## create_sparse_dict_from_values

`function` · `arrow::util::bench_util::create_sparse_dict_from_values`

```rust
fn create_sparse_dict_from_values<K>(size: usize, null_density: f32, values: &dyn Array, key_range: std::ops::Range<K::Native>) -> DictionaryArray<K> where K: ArrowDictionaryKeyType, rand::distr::StandardUniform: Distribution<K::Native>, K::Native: SampleUniform
```

Creates a random (but fixed-seeded) dictionary array of a given size and null density
with the provided values array and key range

---

## create_string_array

`function` · `arrow::util::bench_util::create_string_array`

```rust
fn create_string_array<Offset: OffsetSizeTrait>(size: usize, null_density: f32) -> GenericStringArray<Offset>
```

Creates a random (but fixed-seeded) string array of a given size and null density.

Strings have a random length
between 0 and 400 alphanumeric characters. `0..400` is chosen to cover a wide range of common string lengths,
which have a dramatic impact on performance of some queries, e.g. LIKE/ILIKE/regex.

---

## create_string_array_for_runs

`function` · `arrow::util::bench_util::create_string_array_for_runs`

```rust
fn create_string_array_for_runs(physical_array_len: usize, logical_array_len: usize, string_len: usize) -> Vec<String>
```

Create string array to be used by run array builder. The string array
will result in run array with physical length of `physical_array_len`
and logical length of `logical_array_len`

---

## create_string_array_with_len

`function` · `arrow::util::bench_util::create_string_array_with_len`

```rust
fn create_string_array_with_len<Offset: OffsetSizeTrait>(size: usize, null_density: f32, str_len: usize) -> GenericStringArray<Offset>
```

Creates a random (but fixed-seeded) array of a given size, null density and length

---

## create_string_array_with_len_range_and_prefix_and_seed

`function` · `arrow::util::bench_util::create_string_array_with_len_range_and_prefix_and_seed`

```rust
fn create_string_array_with_len_range_and_prefix_and_seed<Offset: OffsetSizeTrait>(size: usize, null_density: f32, min_str_len: usize, max_str_len: usize, prefix: &str, seed: u64) -> GenericStringArray<Offset>
```

Creates a random [`GenericStringArray`] of a given `size` and `null_density`
filling it with random strings with lengths in the specified range,
all starting with the provided `prefix`, generated using the provided `seed`.

---

## create_string_array_with_max_len

`function` · `arrow::util::bench_util::create_string_array_with_max_len`

```rust
fn create_string_array_with_max_len<Offset: OffsetSizeTrait>(size: usize, null_density: f32, max_str_len: usize) -> GenericStringArray<Offset>
```

Creates a random (but fixed-seeded) array of rand size with a given max size, null density and length

---

## create_string_dict_array

`function` · `arrow::util::bench_util::create_string_dict_array`

```rust
fn create_string_dict_array<K: ArrowDictionaryKeyType>(size: usize, null_density: f32, str_len: usize) -> DictionaryArray<K>
```

Creates an random (but fixed-seeded) array of a given size and null density
consisting of random 4 character alphanumeric strings

---

## create_string_map_array

`function` · `arrow::util::bench_util::create_string_map_array`

```rust
fn create_string_map_array<T>(size: usize, null_density: f32, max_map_size: usize, key_len: usize) -> MapArray where T: ArrowPrimitiveType, rand::distr::StandardUniform: Distribution<T::Native>
```

Create a Map array with string keys and primitive values

Arguments:
- `size`: number of map entries in the array
- `null_density`: density of nulls in the map array (row-level nulls)
- `max_map_size`: maximum number of key-value pairs per map entry
  (actual size is random between 0 and max_map_size)
- `key_len`: length of each random string key

---

## create_string_view_array

`function` · `arrow::util::bench_util::create_string_view_array`

```rust
fn create_string_view_array(size: usize, null_density: f32) -> StringViewArray
```

Creates a random (but fixed-seeded) string view array of a given size and null density.

See `create_string_array` above for more details.

---

## create_string_view_array_with_fixed_len

`function` · `arrow::util::bench_util::create_string_view_array_with_fixed_len`

```rust
fn create_string_view_array_with_fixed_len(size: usize, null_density: f32, str_len: usize) -> StringViewArray
```

Creates a random (but fixed-seeded) array of a given size, null density and length

---

## create_string_view_array_with_len

`function` · `arrow::util::bench_util::create_string_view_array_with_len`

```rust
fn create_string_view_array_with_len(size: usize, null_density: f32, str_len: usize, mixed: bool) -> StringViewArray
```

Creates a random (but fixed-seeded) array of a given size, null density and length

---

## create_string_view_array_with_len_range_and_seed

`function` · `arrow::util::bench_util::create_string_view_array_with_len_range_and_seed`

```rust
fn create_string_view_array_with_len_range_and_seed(size: usize, null_density: f32, range: std::ops::Range<usize>, seed: u64) -> StringViewArray
```

Creates a string view array of a given range, null density and length

Arguments:
- `size`: number of  string view array
- `null_density`: density of nulls in the string view array
- `range`: range size of each string in the string view array
- `seed`: seed for the random number generator

---

## create_string_view_array_with_max_len

`function` · `arrow::util::bench_util::create_string_view_array_with_max_len`

```rust
fn create_string_view_array_with_max_len(size: usize, null_density: f32, max_str_len: usize) -> StringViewArray
```

Creates a random (but fixed-seeded) array of rand size with a given max size, null density and length

---
