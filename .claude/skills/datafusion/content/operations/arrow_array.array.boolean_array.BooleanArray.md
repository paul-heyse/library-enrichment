# `arrow_array::array::boolean_array::BooleanArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.boolean_array.BooleanArray.json).

<a id="op-da9041df4f2e5d0ab93f8505"></a>
## BooleanArray

`struct` · `arrow_array::array::boolean_array::BooleanArray` · arrow-array 59.3.0

```rust
struct BooleanArray
```

Source: `src/array/boolean_array.rs:68`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

An array of [boolean values](https://arrow.apache.org/docs/format/Columnar.html#fixed-size-primitive-layout)

# Example: From a Vec

```
# use arrow_array::{Array, BooleanArray};
let arr: BooleanArray = vec![true, true, false].into();
```

# Example: From an optional Vec

```
# use arrow_array::{Array, BooleanArray};
let arr: BooleanArray = vec![Some(true), None, Some(false)].into();
```

# Example: From an iterator

```
# use arrow_array::{Array, BooleanArray};
let arr: BooleanArray = (0..5).map(|x| (x % 2 == 0).then(|| x % 3 == 0)).collect();
let values: Vec<_> = arr.iter().collect();
assert_eq!(&values, &[Some(true), None, Some(false), None, Some(false)])
```

# Example: Using Builder

```
# use arrow_array::Array;
# use arrow_array::builder::BooleanBuilder;
let mut builder = BooleanBuilder::new();
builder.append_value(true);
builder.append_null();
builder.append_value(false);
let array = builder.finish();
let values: Vec<_> = array.iter().collect();
assert_eq!(&values, &[Some(true), None, Some(false)])
```


<a id="op-bb0b563d0ec9f17174196501"></a>
## as_any

`function` · `arrow_array::array::boolean_array::BooleanArray::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [620, 1], "end": [687, 2], "filename": "src/array/boolean_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/boolean_array.rs:621`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a50ab4d453218d67548b7c23"></a>
## bitwise_bin_op

`function` · `arrow_array::array::boolean_array::BooleanArray::bitwise_bin_op` · arrow-array 59.3.0

```rust
fn bitwise_bin_op<F>(&self, rhs: &BooleanArray, op: F) -> BooleanArray where F: FnMut(u64, u64) -> u64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [617, 2], "filename": "src/array/boolean_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/boolean_array.rs:452`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Apply a bitwise binary operation to this array and `rhs` using u64
operations, returning a new [`BooleanArray`](../operations/arrow_array.array.boolean_array.BooleanArray.md#op-da9041df4f2e5d0ab93f8505).

Null buffers are unioned: the result is null where either input is null.

See [`BooleanBuffer::from_bitwise_binary_op`] for details on the operation.

# Panics

Panics if `self` and `rhs` have different lengths.

# Example

```
# use arrow_array::BooleanArray;
let a = BooleanArray::from(vec![true, false, true, true]);
let b = BooleanArray::from(vec![true, true, false, true]);
let result = a.bitwise_bin_op(&b, |a, b| a & b);
assert_eq!(result, BooleanArray::from(vec![true, false, false, true]));
```

Unresolved upstream links (retained, not inferred): ``BooleanBuffer::from_bitwise_binary_op``.

<a id="op-5833c7d354e164b051720775"></a>
## bitwise_bin_op_mut

`function` · `arrow_array::array::boolean_array::BooleanArray::bitwise_bin_op_mut` · arrow-array 59.3.0

```rust
fn bitwise_bin_op_mut<F>(self, rhs: &BooleanArray, op: F) -> Result<BooleanArray, BooleanArray> where F: FnMut(u64, u64) -> u64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [617, 2], "filename": "src/array/boolean_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/boolean_array.rs:492`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Try to apply a bitwise binary operation to this array and `rhs` in
place using u64 operations.

If this array's underlying buffer is uniquely owned, the operation is
applied in place and `Ok` is returned. If the buffer is shared,
`Err(self)` is returned so the caller can fall back to
[`bitwise_bin_op`](Self::bitwise_bin_op).

Null buffers are unioned: the result is null where either input is null.

# Panics

Panics if `self` and `rhs` have different lengths.

# Example

```
# use arrow_array::BooleanArray;
let a = BooleanArray::from(vec![true, false, true, true]);
let b = BooleanArray::from(vec![true, true, false, true]);
let result = a.bitwise_bin_op_mut(&b, |a, b| a & b).unwrap();
assert_eq!(result, BooleanArray::from(vec![true, false, false, true]));
```

<a id="op-10609f4371bb88804230208a"></a>
## bitwise_bin_op_mut_or_clone

`function` · `arrow_array::array::boolean_array::BooleanArray::bitwise_bin_op_mut_or_clone` · arrow-array 59.3.0

```rust
fn bitwise_bin_op_mut_or_clone<F>(self, rhs: &BooleanArray, op: F) -> BooleanArray where F: FnMut(u64, u64) -> u64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [617, 2], "filename": "src/array/boolean_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/boolean_array.rs:525`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Apply a bitwise binary operation to this array and `rhs` in place if the
buffer is uniquely owned, or clone and apply if shared.

This is a convenience wrapper around [`bitwise_bin_op_mut`](Self::bitwise_bin_op_mut)
that falls back to [`bitwise_bin_op`](Self::bitwise_bin_op) when the buffer is shared.

Null buffers are unioned: the result is null where either input is null.

# Panics

Panics if `self` and `rhs` have different lengths.

# Example

```
# use arrow_array::BooleanArray;
let a = BooleanArray::from(vec![true, false, true, true]);
let b = BooleanArray::from(vec![true, true, false, true]);
let result = a.bitwise_bin_op_mut_or_clone(&b, |a, b| a & b);
assert_eq!(result, BooleanArray::from(vec![true, false, false, true]));
```

<a id="op-443d383a094b9ee2fb63fbc3"></a>
## bitwise_unary

`function` · `arrow_array::array::boolean_array::BooleanArray::bitwise_unary` · arrow-array 59.3.0

```rust
fn bitwise_unary<F>(&self, op: F) -> BooleanArray where F: FnMut(u64) -> u64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [617, 2], "filename": "src/array/boolean_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/boolean_array.rs:345`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Apply a bitwise operation to this array's values using u64 operations,
returning a new [`BooleanArray`](../operations/arrow_array.array.boolean_array.BooleanArray.md#op-da9041df4f2e5d0ab93f8505).

The null buffer is preserved unchanged.

See [`BooleanBuffer::from_bitwise_unary_op`] for details on the operation.

# Example

```
# use arrow_array::BooleanArray;
let array = BooleanArray::from(vec![true, false, true]);
let result = array.bitwise_unary(|x| !x);
assert_eq!(result, BooleanArray::from(vec![false, true, false]));
```

Unresolved upstream links (retained, not inferred): ``BooleanBuffer::from_bitwise_unary_op``.

<a id="op-35f58783609ce45e3ce57e7a"></a>
## bitwise_unary_mut

`function` · `arrow_array::array::boolean_array::BooleanArray::bitwise_unary_mut` · arrow-array 59.3.0

```rust
fn bitwise_unary_mut<F>(self, op: F) -> Result<BooleanArray, BooleanArray> where F: FnMut(u64) -> u64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [617, 2], "filename": "src/array/boolean_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/boolean_array.rs:375`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Try to apply a bitwise operation to this array's values in place using
u64 operations.

If the underlying buffer is uniquely owned, the operation is applied
in place and `Ok` is returned. If the buffer is shared, `Err(self)` is
returned so the caller can fall back to [`bitwise_unary`](Self::bitwise_unary).

The null buffer is preserved unchanged.

# Example

```
# use arrow_array::BooleanArray;
let array = BooleanArray::from(vec![true, false, true]);
let result = array.bitwise_unary_mut(|x| !x).unwrap();
assert_eq!(result, BooleanArray::from(vec![false, true, false]));
```

<a id="op-6aefc33e7841f07c2099d0c8"></a>
## bitwise_unary_mut_or_clone

`function` · `arrow_array::array::boolean_array::BooleanArray::bitwise_unary_mut_or_clone` · arrow-array 59.3.0

```rust
fn bitwise_unary_mut_or_clone<F>(self, op: F) -> BooleanArray where F: FnMut(u64) -> u64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [617, 2], "filename": "src/array/boolean_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/boolean_array.rs:399`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Apply a bitwise operation to this array's values in place if the buffer
is uniquely owned, or clone and apply if shared.

This is a convenience wrapper around [`bitwise_unary_mut`](Self::bitwise_unary_mut)
that falls back to [`bitwise_unary`](Self::bitwise_unary) when the buffer is shared.

The null buffer is preserved unchanged.

# Example

```
# use arrow_array::BooleanArray;
let array = BooleanArray::from(vec![true, false, true]);
let result = array.bitwise_unary_mut_or_clone(|x| !x);
assert_eq!(result, BooleanArray::from(vec![false, true, false]));
```

<a id="op-dadc877777364219a0cdaeb0"></a>
## builder

`function` · `arrow_array::array::boolean_array::BooleanArray::builder` · arrow-array 59.3.0

```rust
fn builder(capacity: usize) -> BooleanBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [617, 2], "filename": "src/array/boolean_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/boolean_array.rs:161`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a new boolean array builder

<a id="op-9299f6f2e18068417f0f2f4a"></a>
## claim

`function` · `arrow_array::array::boolean_array::BooleanArray::claim` · arrow-array 59.3.0

```rust
fn claim(&self, pool: &dyn arrow_buffer::MemoryPool)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [620, 1], "end": [687, 2], "filename": "src/array/boolean_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/boolean_array.rs:681`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f42f930b07035dc6e9ec2288"></a>
## clone

`function` · `arrow_array::array::boolean_array::BooleanArray::clone` · arrow-array 59.3.0

```rust
fn clone(&self) -> BooleanArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 10], "end": [67, 15], "filename": "src/array/boolean_array.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/array/boolean_array.rs:67`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f8a7c731ad03e5ad23411df"></a>
## data_type

`function` · `arrow_array::array::boolean_array::BooleanArray::data_type` · arrow-array 59.3.0

```rust
fn data_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [620, 1], "end": [687, 2], "filename": "src/array/boolean_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/boolean_array.rs:633`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6dce70a40616f4e48df92e32"></a>
## eq

`function` · `arrow_array::array::boolean_array::BooleanArray::eq` · arrow-array 59.3.0

```rust
fn eq(&self, other: &BooleanArray) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [794, 1], "end": [798, 2], "filename": "src/array/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/array/mod.rs:795`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c0705eaa20558b8a7f5d391"></a>
## false_count

`function` · `arrow_array::array::boolean_array::BooleanArray::false_count` · arrow-array 59.3.0

```rust
fn false_count(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [617, 2], "filename": "src/array/boolean_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/boolean_array.rs:188`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the number of non null, false values within this array.
If you only need to check if there is at least one false value, consider using `has_false()` which can short-circuit and be more efficient.

<a id="op-e640347ed46fa07c12e087f3"></a>
## fmt

`function` · `arrow_array::array::boolean_array::BooleanArray::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [81, 2], "filename": "src/array/boolean_array.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/array/boolean_array.rs:74`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41889355dab0689d05e98d9b"></a>
## from

`function` · `arrow_array::array::boolean_array::BooleanArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<bool>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [701, 1], "end": [719, 2], "filename": "src/array/boolean_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "bool"}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/boolean_array.rs:702`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e7ee5bf7ebe70b06e5c94ea"></a>
## from

`function` · `arrow_array::array::boolean_array::BooleanArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<Option<bool>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [721, 1], "end": [725, 2], "filename": "src/array/boolean_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "bool"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/boolean_array.rs:722`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f8aafd602494db46c32dc25"></a>
## from

`function` · `arrow_array::array::boolean_array::BooleanArray::from` · arrow-array 59.3.0

```rust
fn from(values: BooleanBuffer) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [880, 1], "end": [887, 2], "filename": "src/array/boolean_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/boolean_array.rs:881`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bdd73d39003c914cd3a8a65f"></a>
## from

`function` · `arrow_array::array::boolean_array::BooleanArray::from` · arrow-array 59.3.0

```rust
fn from(data: ArrayData) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [727, 1], "end": [745, 2], "filename": "src/array/boolean_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/boolean_array.rs:728`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57888aa93a425c1d88551237"></a>
## from_binary

`function` · `arrow_array::array::boolean_array::BooleanArray::from_binary` · arrow-array 59.3.0

```rust
fn from_binary<T: ArrayAccessor, S: ArrayAccessor, F>(left: T, right: S, op: F) -> Self where F: FnMut(T::Item, S::Item) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [617, 2], "filename": "src/array/boolean_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/boolean_array.rs:313`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a [`BooleanArray`](../operations/arrow_array.array.boolean_array.BooleanArray.md#op-da9041df4f2e5d0ab93f8505) by evaluating the binary operation for
each element of the provided arrays

```
# use arrow_array::{BooleanArray, Int32Array};

let a = Int32Array::from(vec![1, 2, 3, 4, 5]);
let b = Int32Array::from(vec![1, 2, 0, 2, 5]);
let r = BooleanArray::from_binary(&a, &b, |a, b| a == b);
assert_eq!(&r, &BooleanArray::from(vec![true, true, false, false, true]));
```

# Panics

This function panics if left and right are not the same length


<a id="op-c15306a71b767b4495de56d8"></a>
## from_iter

`function` · `arrow_array::array::boolean_array::BooleanArray::from_iter` · arrow-array 59.3.0

```rust
fn from_iter<I: IntoIterator<Item = Ptr>>(iter: I) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "unresolved", "path": "BooleanAdapter"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "default": null, "is_synthetic": false}}, "name": "Ptr"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [817, 1], "end": [828, 2], "filename": "src/array/boolean_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Ptr"}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/array/boolean_array.rs:818`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b013f434371ed9aec2d0b8c"></a>
## from_trusted_len_iter

`function` · `arrow_array::array::boolean_array::BooleanArray::from_trusted_len_iter` · arrow-array 59.3.0

```rust
unsafe fn from_trusted_len_iter<I, P>(iter: I) -> Self where P: Into<BooleanAdapter>, I: ExactSizeIterator<Item = P>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [830, 1], "end": [878, 2], "filename": "src/array/boolean_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/boolean_array.rs:847`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a [`BooleanArray`](../operations/arrow_array.array.boolean_array.BooleanArray.md#op-da9041df4f2e5d0ab93f8505) from an iterator of trusted length.

# Safety

The iterator must be [`TrustedLen`](https://doc.rust-lang.org/std/iter/trait.TrustedLen.html).
I.e. that `size_hint().1` correctly reports its length. Note that this is a stronger
guarantee that `ExactSizeIterator` provides which could still report a wrong length.

# Panics

Panics if the iterator does not report an upper bound on `size_hint()`.

<a id="op-a8efe743b6ce15f1e33e9f8c"></a>
## from_unary

`function` · `arrow_array::array::boolean_array::BooleanArray::from_unary` · arrow-array 59.3.0

```rust
fn from_unary<T: ArrayAccessor, F>(left: T, op: F) -> Self where F: FnMut(T::Item) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [617, 2], "filename": "src/array/boolean_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/boolean_array.rs:285`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a [`BooleanArray`](../operations/arrow_array.array.boolean_array.BooleanArray.md#op-da9041df4f2e5d0ab93f8505) by evaluating the operation for
each element of the provided array

```
# use arrow_array::{BooleanArray, Int32Array};

let array = Int32Array::from(vec![1, 2, 3, 4, 5]);
let r = BooleanArray::from_unary(&array, |x| x > 2);
assert_eq!(&r, &BooleanArray::from(vec![false, false, true, true, true]));
```

<a id="op-f8fa720fba6a372bc3e102c6"></a>
## get_array_memory_size

`function` · `arrow_array::array::boolean_array::BooleanArray::get_array_memory_size` · arrow-array 59.3.0

```rust
fn get_array_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [620, 1], "end": [687, 2], "filename": "src/array/boolean_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/boolean_array.rs:676`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b01dcb91994c1a69fb40f0db"></a>
## get_buffer_memory_size

`function` · `arrow_array::array::boolean_array::BooleanArray::get_buffer_memory_size` · arrow-array 59.3.0

```rust
fn get_buffer_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [620, 1], "end": [687, 2], "filename": "src/array/boolean_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/boolean_array.rs:668`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-07868c4d818b602a31b85e6b"></a>
## has_false

`function` · `arrow_array::array::boolean_array::BooleanArray::has_false` · arrow-array 59.3.0

```rust
fn has_false(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [617, 2], "filename": "src/array/boolean_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/boolean_array.rs:215`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns whether there is at least one non-null `false` value in this array.

This is more efficient than `false_count() > 0` because it can short-circuit
as soon as a `false` value is found, without counting all set bits.

Null values are not counted as `false`. Returns `false` for empty arrays.

<a id="op-d100f3cee7c834100fb30ce8"></a>
## has_true

`function` · `arrow_array::array::boolean_array::BooleanArray::has_true` · arrow-array 59.3.0

```rust
fn has_true(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [617, 2], "filename": "src/array/boolean_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/boolean_array.rs:198`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns whether there is at least one non-null `true` value in this array.

This is more efficient than `true_count() > 0` because it can short-circuit
as soon as a `true` value is found, without counting all set bits.

Null values are not counted as `true`. Returns `false` for empty arrays.

<a id="op-0cb974151db7915fb352d955"></a>
## into_data

`function` · `arrow_array::array::boolean_array::BooleanArray::into_data` · arrow-array 59.3.0

```rust
fn into_data(self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [620, 1], "end": [687, 2], "filename": "src/array/boolean_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/boolean_array.rs:629`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c8698cf34e48c5b01bd8aa6"></a>
## into_parts

`function` · `arrow_array::array::boolean_array::BooleanArray::into_parts` · arrow-array 59.3.0

```rust
fn into_parts(self) -> (BooleanBuffer, Option<NullBuffer>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [617, 2], "filename": "src/array/boolean_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/boolean_array.rs:614`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Deconstruct this array into its constituent parts

<a id="op-44b845e50866403869718ee1"></a>
## is_empty

`function` · `arrow_array::array::boolean_array::BooleanArray::is_empty` · arrow-array 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [620, 1], "end": [687, 2], "filename": "src/array/boolean_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/boolean_array.rs:645`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c43b63a1510170724283f9b"></a>
## is_empty

`function` · `arrow_array::array::boolean_array::BooleanArray::is_empty` · arrow-array 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [617, 2], "filename": "src/array/boolean_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/boolean_array.rs:148`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns whether this array is empty.

<a id="op-4ccf10556ca7b5c5fb3f257e"></a>
## iter

`function` · `arrow_array::array::boolean_array::BooleanArray::iter` · arrow-array 59.3.0

```rust
fn iter(&'a self) -> BooleanIter<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [768, 1], "end": [773, 2], "filename": "src/array/boolean_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/boolean_array.rs:770`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

constructs a new iterator

<a id="op-a4389e314374452adf619afc"></a>
## len

`function` · `arrow_array::array::boolean_array::BooleanArray::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [620, 1], "end": [687, 2], "filename": "src/array/boolean_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/boolean_array.rs:641`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dce606be3f7663a7c6ee9d9d"></a>
## len

`function` · `arrow_array::array::boolean_array::BooleanArray::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [617, 2], "filename": "src/array/boolean_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/boolean_array.rs:143`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the length of this array.

<a id="op-d77f2035d51de7f30c7a494a"></a>
## logical_null_count

`function` · `arrow_array::array::boolean_array::BooleanArray::logical_null_count` · arrow-array 59.3.0

```rust
fn logical_null_count(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [620, 1], "end": [687, 2], "filename": "src/array/boolean_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/boolean_array.rs:664`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee1b456241fdb806ff40c239"></a>
## new

`function` · `arrow_array::array::boolean_array::BooleanArray::new` · arrow-array 59.3.0

```rust
fn new(values: BooleanBuffer, nulls: Option<NullBuffer>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [617, 2], "filename": "src/array/boolean_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/boolean_array.rs:89`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`BooleanArray`](../operations/arrow_array.array.boolean_array.BooleanArray.md#op-da9041df4f2e5d0ab93f8505) from the provided values and nulls

# Panics

Panics if `values.len() != nulls.len()`

<a id="op-fdde8d22383201d196d629cf"></a>
## new_from_packed

`function` · `arrow_array::array::boolean_array::BooleanArray::new_from_packed` · arrow-array 59.3.0

```rust
fn new_from_packed(buffer: impl Into<Buffer>, offset: usize, len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [617, 2], "filename": "src/array/boolean_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/boolean_array.rs:129`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`BooleanArray`](../operations/arrow_array.array.boolean_array.BooleanArray.md#op-da9041df4f2e5d0ab93f8505) from a [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b) specified by `offset` and `len`, the `offset` and `len` in bits
Logically convert each bit in [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b) to boolean and use it to build [`BooleanArray`](../operations/arrow_array.array.boolean_array.BooleanArray.md#op-da9041df4f2e5d0ab93f8505).
using this method will make the following points self-evident:
* there is no `null` in the constructed [`BooleanArray`](../operations/arrow_array.array.boolean_array.BooleanArray.md#op-da9041df4f2e5d0ab93f8505);
* without considering `buffer.into()`, this method is efficient because there is no need to perform pack and unpack operations on boolean;

<a id="op-8662505c00fac9df9aae2485"></a>
## new_from_u8

`function` · `arrow_array::array::boolean_array::BooleanArray::new_from_u8` · arrow-array 59.3.0

```rust
fn new_from_u8(value: &[u8]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [617, 2], "filename": "src/array/boolean_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/boolean_array.rs:138`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`BooleanArray`](../operations/arrow_array.array.boolean_array.BooleanArray.md#op-da9041df4f2e5d0ab93f8505) from `&[u8]`
This method uses `new_from_packed` and constructs a [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b) using `value`, and offset is set to 0 and len is set to `value.len() * 8`
using this method will make the following points self-evident:
* there is no `null` in the constructed [`BooleanArray`](../operations/arrow_array.array.boolean_array.BooleanArray.md#op-da9041df4f2e5d0ab93f8505);
* the length of the constructed [`BooleanArray`](../operations/arrow_array.array.boolean_array.BooleanArray.md#op-da9041df4f2e5d0ab93f8505) is always a multiple of 8;

<a id="op-84c91168b5254b63c2d62541"></a>
## new_null

`function` · `arrow_array::array::boolean_array::BooleanArray::new_null` · arrow-array 59.3.0

```rust
fn new_null(len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [617, 2], "filename": "src/array/boolean_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/boolean_array.rs:108`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`BooleanArray`](../operations/arrow_array.array.boolean_array.BooleanArray.md#op-da9041df4f2e5d0ab93f8505) with length `len` consisting only of nulls

<a id="op-3e32334c929a327f62ab7765"></a>
## new_scalar

`function` · `arrow_array::array::boolean_array::BooleanArray::new_scalar` · arrow-array 59.3.0

```rust
fn new_scalar(value: bool) -> Scalar<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [617, 2], "filename": "src/array/boolean_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/boolean_array.rs:116`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`Scalar`](../operations/arrow_array.scalar.Scalar.md#op-0ca10f05b262b3afc7077257) from `value`

<a id="op-4402613f522a939d68f47d27"></a>
## new_unchecked

`function` · `arrow_array::array::boolean_array::BooleanArray::new_unchecked` · arrow-array 59.3.0

```rust
unsafe fn new_unchecked(values: BooleanBuffer, nulls: Option<NullBuffer>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [617, 2], "filename": "src/array/boolean_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/boolean_array.rs:100`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`BooleanArray`](../operations/arrow_array.array.boolean_array.BooleanArray.md#op-da9041df4f2e5d0ab93f8505) from the provided values and nulls without validation.

# Safety
- `values.len() == nulls.len()` if `nulls` is `Some`

<a id="op-c547439ecc799cdbc3fb679d"></a>
## nulls

`function` · `arrow_array::array::boolean_array::BooleanArray::nulls` · arrow-array 59.3.0

```rust
fn nulls(&self) -> Option<&NullBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [620, 1], "end": [687, 2], "filename": "src/array/boolean_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/boolean_array.rs:660`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b793b3c046f0c443a45e112"></a>
## offset

`function` · `arrow_array::array::boolean_array::BooleanArray::offset` · arrow-array 59.3.0

```rust
fn offset(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [620, 1], "end": [687, 2], "filename": "src/array/boolean_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/boolean_array.rs:656`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8b1e5188ddf032fbdd80770"></a>
## shrink_to_fit

`function` · `arrow_array::array::boolean_array::BooleanArray::shrink_to_fit` · arrow-array 59.3.0

```rust
fn shrink_to_fit(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [620, 1], "end": [687, 2], "filename": "src/array/boolean_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/boolean_array.rs:649`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0550d9199b471fa6b7e54a73"></a>
## slice

`function` · `arrow_array::array::boolean_array::BooleanArray::slice` · arrow-array 59.3.0

```rust
fn slice(&self, offset: usize, length: usize) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [620, 1], "end": [687, 2], "filename": "src/array/boolean_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/boolean_array.rs:637`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-990caeaa8fca2e5fb52facc5"></a>
## slice

`function` · `arrow_array::array::boolean_array::BooleanArray::slice` · arrow-array 59.3.0

```rust
fn slice(&self, offset: usize, length: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [617, 2], "filename": "src/array/boolean_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/boolean_array.rs:153`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a zero-copy slice of this array with the indicated offset and length.

<a id="op-3a0d6bb7a542e624006d8ccb"></a>
## take_iter

`function` · `arrow_array::array::boolean_array::BooleanArray::take_iter` · arrow-array 59.3.0

```rust
fn take_iter<'a>(&'a self, indexes: impl Iterator<Item = Option<usize>> + 'a) -> impl Iterator<Item = Option<bool>> + 'a
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [617, 2], "filename": "src/array/boolean_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/boolean_array.rs:257`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns an iterator that returns the values of `array.value(i)` for an iterator with each element `i`

<a id="op-546d62834b4c3b88fe10d491"></a>
## take_iter_unchecked

`function` · `arrow_array::array::boolean_array::BooleanArray::take_iter_unchecked` · arrow-array 59.3.0

```rust
unsafe fn take_iter_unchecked<'a>(&'a self, indexes: impl Iterator<Item = Option<usize>> + 'a) -> impl Iterator<Item = Option<bool>> + 'a
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [617, 2], "filename": "src/array/boolean_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/boolean_array.rs:268`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns an iterator that returns the values of `array.value(i)` for an iterator with each element `i`
# Safety

caller must ensure that the offsets in the iterator are less than the array len()

<a id="op-face03aead92e46bafa90aef"></a>
## take_n_true

`function` · `arrow_array::array::boolean_array::BooleanArray::take_n_true` · arrow-array 59.3.0

```rust
fn take_n_true(self, n: usize) -> BooleanArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [617, 2], "filename": "src/array/boolean_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/boolean_array.rs:591`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a new [`BooleanArray`](../operations/arrow_array.array.boolean_array.BooleanArray.md#op-da9041df4f2e5d0ab93f8505) of the same length where only the first
`n` non-null `true` positions remain `true`; any `true` positions
beyond the first `n` are replaced with `false`. The null buffer is
preserved unchanged.

If this array has at most `n` non-null `true` values, `self` is
returned unchanged.

# Example

```
# use arrow_array::BooleanArray;
let a = BooleanArray::from(vec![true, false, true, true, false, true]);
// Keep only the first 2 `true` positions; later trues become false.
let r = a.take_n_true(2);
assert_eq!(r, BooleanArray::from(vec![true, false, true, false, false, false]));
```

<a id="op-84ffcc773772c7486e64d645"></a>
## to_data

`function` · `arrow_array::array::boolean_array::BooleanArray::to_data` · arrow-array 59.3.0

```rust
fn to_data(&self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [620, 1], "end": [687, 2], "filename": "src/array/boolean_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/boolean_array.rs:625`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95946afc88976d9f0bb3ddc9"></a>
## true_count

`function` · `arrow_array::array::boolean_array::BooleanArray::true_count` · arrow-array 59.3.0

```rust
fn true_count(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [617, 2], "filename": "src/array/boolean_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/boolean_array.rs:172`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the number of non null, true values within this array.
If you only need to check if there is at least one true value, consider using `has_true()` which can short-circuit and be more efficient.

<a id="op-affc095fdef5dfe61ca50a63"></a>
## value

`function` · `arrow_array::array::boolean_array::BooleanArray::value` · arrow-array 59.3.0

```rust
fn value(&self, i: usize) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [617, 2], "filename": "src/array/boolean_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/boolean_array.rs:244`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the boolean value at index `i`.

Note: This method does not check for nulls and the value is arbitrary
if [`is_null`](Self::is_null) returns true for the index.

# Panics
Panics if index `i` is out of bounds

<a id="op-f1e8f57360ad3dad15f0676f"></a>
## value_unchecked

`function` · `arrow_array::array::boolean_array::BooleanArray::value_unchecked` · arrow-array 59.3.0

```rust
unsafe fn value_unchecked(&self, i: usize) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [617, 2], "filename": "src/array/boolean_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/boolean_array.rs:233`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the boolean value at index `i`.

Note: This method does not check for nulls and the value is arbitrary
if [`is_null`](Self::is_null) returns true for the index.

# Safety
This doesn't check bounds, the caller must ensure that index < self.len()

<a id="op-30a83aac6350e56e883ef791"></a>
## values

`function` · `arrow_array::array::boolean_array::BooleanArray::values` · arrow-array 59.3.0

```rust
fn values(&self) -> &BooleanBuffer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "BooleanArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [617, 2], "filename": "src/array/boolean_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/boolean_array.rs:166`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the underlying [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5) holding all the values of this array
