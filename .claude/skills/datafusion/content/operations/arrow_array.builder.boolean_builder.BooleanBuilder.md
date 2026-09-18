# `arrow_array::builder::boolean_builder::BooleanBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.boolean_builder.BooleanBuilder.json).

<a id="op-0fe2b53b6507a1d09db8dce4"></a>
## BooleanBuilder

`struct` · `arrow_array::builder::boolean_builder::BooleanBuilder` · arrow-array 59.3.0

```rust
struct BooleanBuilder
```

Source: `src/builder/boolean_builder.rs:59`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builder for [`BooleanArray`](../operations/arrow_array.array.boolean_array.BooleanArray.md#op-da9041df4f2e5d0ab93f8505)

# Example

Create a `BooleanArray` from a `BooleanBuilder`

```

# use arrow_array::{Array, BooleanArray, builder::BooleanBuilder};

let mut b = BooleanBuilder::new();
b.append_value(true);
b.append_null();
b.append_value(false);
b.append_value(true);
let arr = b.finish();

assert_eq!(4, arr.len());
assert_eq!(1, arr.null_count());
assert_eq!(true, arr.value(0));
assert!(arr.is_valid(0));
assert!(!arr.is_null(0));
assert!(!arr.is_valid(1));
assert!(arr.is_null(1));
assert_eq!(false, arr.value(2));
assert!(arr.is_valid(2));
assert!(!arr.is_null(2));
assert_eq!(true, arr.value(3));
assert!(arr.is_valid(3));
assert!(!arr.is_null(3));
```

<a id="op-49ab1ea1d8b7d760d5e48263"></a>
## append_array

`function` · `arrow_array::builder::boolean_builder::BooleanBuilder::append_array` · arrow-array 59.3.0

```rust
fn append_array(&mut self, array: &BooleanArray)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::boolean_builder::BooleanBuilder", "path": "BooleanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [200, 2], "filename": "src/builder/boolean_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean_builder.rs:152`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends array values and null to this builder as is
(this means that underlying null values are copied as is).

<a id="op-186abdf3b3dcc7f8191e2f9a"></a>
## append_n

`function` · `arrow_array::builder::boolean_builder::BooleanBuilder::append_n` · arrow-array 59.3.0

```rust
fn append_n(&mut self, additional: usize, v: bool)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::boolean_builder::BooleanBuilder", "path": "BooleanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [200, 2], "filename": "src/builder/boolean_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean_builder.rs:128`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends n `additional` bits of value `v` into the buffer

<a id="op-a5f60f542bb7edcbba6ef7ce"></a>
## append_null

`function` · `arrow_array::builder::boolean_builder::BooleanBuilder::append_null` · arrow-array 59.3.0

```rust
fn append_null(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::boolean_builder::BooleanBuilder", "path": "BooleanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [200, 2], "filename": "src/builder/boolean_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean_builder.rs:98`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends a null slot into the builder

<a id="op-c5dea7633db5b9fd9fcd5247"></a>
## append_nulls

`function` · `arrow_array::builder::boolean_builder::BooleanBuilder::append_nulls` · arrow-array 59.3.0

```rust
fn append_nulls(&mut self, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::boolean_builder::BooleanBuilder", "path": "BooleanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [200, 2], "filename": "src/builder/boolean_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean_builder.rs:105`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends `n` `null`s into the builder.

<a id="op-15c61597ec2c887970bd827b"></a>
## append_option

`function` · `arrow_array::builder::boolean_builder::BooleanBuilder::append_option` · arrow-array 59.3.0

```rust
fn append_option(&mut self, v: Option<bool>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::boolean_builder::BooleanBuilder", "path": "BooleanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [200, 2], "filename": "src/builder/boolean_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean_builder.rs:112`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends an `Option<T>` into the builder

<a id="op-dc490bce95a94643dd044e3f"></a>
## append_slice

`function` · `arrow_array::builder::boolean_builder::BooleanBuilder::append_slice` · arrow-array 59.3.0

```rust
fn append_slice(&mut self, v: &[bool])
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::boolean_builder::BooleanBuilder", "path": "BooleanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [200, 2], "filename": "src/builder/boolean_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean_builder.rs:121`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends a slice of type `T` into the builder

<a id="op-8da0c79ca87267808e591b75"></a>
## append_value

`function` · `arrow_array::builder::boolean_builder::BooleanBuilder::append_value` · arrow-array 59.3.0

```rust
fn append_value(&mut self, v: bool)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::boolean_builder::BooleanBuilder", "path": "BooleanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [200, 2], "filename": "src/builder/boolean_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean_builder.rs:91`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends a value of type `T` into the builder

<a id="op-5ca09ebd2ee55f7088d87159"></a>
## append_values

`function` · `arrow_array::builder::boolean_builder::BooleanBuilder::append_values` · arrow-array 59.3.0

```rust
fn append_values(&mut self, values: &[bool], is_valid: &[bool]) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::boolean_builder::BooleanBuilder", "path": "BooleanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [200, 2], "filename": "src/builder/boolean_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean_builder.rs:137`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends values from a slice of type `T` and a validity boolean slice.

Returns an error if the slices are of different lengths

<a id="op-0e7649434a1806106a8f01d3"></a>
## as_any

`function` · `arrow_array::builder::boolean_builder::BooleanBuilder::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::boolean_builder::BooleanBuilder", "path": "BooleanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [232, 2], "filename": "src/builder/boolean_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/boolean_builder.rs:204`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the builder as a non-mutable `Any` reference.

<a id="op-2c2e8cbebec2c23a99ab374d"></a>
## as_any_mut

`function` · `arrow_array::builder::boolean_builder::BooleanBuilder::as_any_mut` · arrow-array 59.3.0

```rust
fn as_any_mut(&mut self) -> &mut dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::boolean_builder::BooleanBuilder", "path": "BooleanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [232, 2], "filename": "src/builder/boolean_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/boolean_builder.rs:209`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the builder as a mutable `Any` reference.

<a id="op-a642ba749f04de51111536d6"></a>
## capacity

`function` · `arrow_array::builder::boolean_builder::BooleanBuilder::capacity` · arrow-array 59.3.0

```rust
fn capacity(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::boolean_builder::BooleanBuilder", "path": "BooleanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [200, 2], "filename": "src/builder/boolean_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean_builder.rs:85`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the capacity of this builder measured in slots of type `T`

<a id="op-1c679dc38efde4fc59b9bd59"></a>
## default

`function` · `arrow_array::builder::boolean_builder::BooleanBuilder::default` · arrow-array 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::boolean_builder::BooleanBuilder", "path": "BooleanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [68, 2], "filename": "src/builder/boolean_builder.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/builder/boolean_builder.rs:65`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-812a44519ee547132e04b1d5"></a>
## extend

`function` · `arrow_array::builder::boolean_builder::BooleanBuilder::extend` · arrow-array 59.3.0

```rust
fn extend<T: IntoIterator<Item = Option<bool>>>(&mut self, iter: T)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::boolean_builder::BooleanBuilder", "path": "BooleanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [234, 1], "end": [244, 2], "filename": "src/builder/boolean_builder.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "bool"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::iter::traits::collect::Extend", "path": "Extend"}, "trait_path": "core::iter::traits::collect::Extend"}`

Source: `src/builder/boolean_builder.rs:236`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86302843753a9f5ee201c6e6"></a>
## finish

`function` · `arrow_array::builder::boolean_builder::BooleanBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::boolean_builder::BooleanBuilder", "path": "BooleanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [232, 2], "filename": "src/builder/boolean_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/boolean_builder.rs:224`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array and reset this builder.

<a id="op-c332713913a5b58c6330ee16"></a>
## finish

`function` · `arrow_array::builder::boolean_builder::BooleanBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> BooleanArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::boolean_builder::BooleanBuilder", "path": "BooleanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [200, 2], "filename": "src/builder/boolean_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean_builder.rs:162`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the [BooleanArray](../operations/arrow_array.array.boolean_array.BooleanArray.md#op-da9041df4f2e5d0ab93f8505) and reset this builder.

<a id="op-16a768ce37c1c8092ce38631"></a>
## finish_cloned

`function` · `arrow_array::builder::boolean_builder::BooleanBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> BooleanArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::boolean_builder::BooleanBuilder", "path": "BooleanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [200, 2], "filename": "src/builder/boolean_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean_builder.rs:175`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the [BooleanArray](../operations/arrow_array.array.boolean_array.BooleanArray.md#op-da9041df4f2e5d0ab93f8505) without resetting the builder.

<a id="op-2cbb868049318476ed388118"></a>
## finish_cloned

`function` · `arrow_array::builder::boolean_builder::BooleanBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::boolean_builder::BooleanBuilder", "path": "BooleanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [232, 2], "filename": "src/builder/boolean_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/boolean_builder.rs:229`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array without resetting the builder.

<a id="op-4ace9ad0ee8911f1039e6ab9"></a>
## fmt

`function` · `arrow_array::builder::boolean_builder::BooleanBuilder::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::boolean_builder::BooleanBuilder", "path": "BooleanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 10], "end": [58, 15], "filename": "src/builder/boolean_builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/builder/boolean_builder.rs:58`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a6acce26809ebeaf4bca9525"></a>
## into_box_any

`function` · `arrow_array::builder::boolean_builder::BooleanBuilder::into_box_any` · arrow-array 59.3.0

```rust
fn into_box_any(Box<self>) -> Box<dyn Any>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::boolean_builder::BooleanBuilder", "path": "BooleanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [232, 2], "filename": "src/builder/boolean_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/boolean_builder.rs:214`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the boxed builder as a box of `Any`.

<a id="op-82e69dd504e833656d4cdc88"></a>
## len

`function` · `arrow_array::builder::boolean_builder::BooleanBuilder::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::boolean_builder::BooleanBuilder", "path": "BooleanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [232, 2], "filename": "src/builder/boolean_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/boolean_builder.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the number of array slots in the builder

<a id="op-d4c60bc92ab1c2e14760fff7"></a>
## new

`function` · `arrow_array::builder::boolean_builder::BooleanBuilder::new` · arrow-array 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::boolean_builder::BooleanBuilder", "path": "BooleanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [200, 2], "filename": "src/builder/boolean_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean_builder.rs:72`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new boolean builder

<a id="op-834c8d0be2e6d563e5451681"></a>
## validity_slice

`function` · `arrow_array::builder::boolean_builder::BooleanBuilder::validity_slice` · arrow-array 59.3.0

```rust
fn validity_slice(&self) -> Option<&[u8]>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::boolean_builder::BooleanBuilder", "path": "BooleanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [200, 2], "filename": "src/builder/boolean_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean_builder.rs:197`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the current null buffer as a slice

<a id="op-239e70c73f26fe5e247cda08"></a>
## values_slice

`function` · `arrow_array::builder::boolean_builder::BooleanBuilder::values_slice` · arrow-array 59.3.0

```rust
fn values_slice(&self) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::boolean_builder::BooleanBuilder", "path": "BooleanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [200, 2], "filename": "src/builder/boolean_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean_builder.rs:192`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the current values buffer as a slice

Boolean values are bit-packed into bytes. To extract the i-th boolean
from the bytes, you can use `arrow_buffer::bit_util::get_bit()`.

<a id="op-6ccad1cdba0b82acadf00b2e"></a>
## with_capacity

`function` · `arrow_array::builder::boolean_builder::BooleanBuilder::with_capacity` · arrow-array 59.3.0

```rust
fn with_capacity(capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::boolean_builder::BooleanBuilder", "path": "BooleanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [200, 2], "filename": "src/builder/boolean_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean_builder.rs:77`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new boolean builder with space for `capacity` elements without re-allocating
