# `arrow_array::builder::union_builder::UnionBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.union_builder.UnionBuilder.json).

<a id="op-b141f16685d6c84bf0f6ab31"></a>
## UnionBuilder

`struct` · `arrow_array::builder::union_builder::UnionBuilder` · arrow-array 59.3.0

```rust
struct UnionBuilder
```

Source: `src/builder/union_builder.rs:148`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builder for [`UnionArray`](../operations/arrow_array.array.union_array.UnionArray.md#op-39e2f188616dc0298ba644ac)

Example: **Dense Memory Layout**

```
# use arrow_array::builder::UnionBuilder;
# use arrow_array::types::{Float64Type, Int32Type};

let mut builder = UnionBuilder::new_dense();
builder.append::<Int32Type>("a", 1).unwrap();
builder.append::<Float64Type>("b", 3.0).unwrap();
builder.append::<Int32Type>("a", 4).unwrap();
let union = builder.build().unwrap();

assert_eq!(union.type_id(0), 0);
assert_eq!(union.type_id(1), 1);
assert_eq!(union.type_id(2), 0);

assert_eq!(union.value_offset(0), 0);
assert_eq!(union.value_offset(1), 0);
assert_eq!(union.value_offset(2), 1);
```

Example: **Sparse Memory Layout**
```
# use arrow_array::builder::UnionBuilder;
# use arrow_array::types::{Float64Type, Int32Type};

let mut builder = UnionBuilder::new_sparse();
builder.append::<Int32Type>("a", 1).unwrap();
builder.append::<Float64Type>("b", 3.0).unwrap();
builder.append::<Int32Type>("a", 4).unwrap();
let union = builder.build().unwrap();

assert_eq!(union.type_id(0), 0);
assert_eq!(union.type_id(1), 1);
assert_eq!(union.type_id(2), 0);

assert_eq!(union.value_offset(0), 0);
assert_eq!(union.value_offset(1), 1);
assert_eq!(union.value_offset(2), 2);
```

<a id="op-15bb3da7d166e256ea325da4"></a>
## append

`function` · `arrow_array::builder::union_builder::UnionBuilder::append` · arrow-array 59.3.0

```rust
fn append<T: ArrowPrimitiveType>(&mut self, type_name: &str, v: T::Native) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::union_builder::UnionBuilder", "path": "UnionBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [160, 1], "end": [360, 2], "filename": "src/builder/union_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/union_builder.rs:210`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends a value to this builder.

<a id="op-18459f40e84e5918a5d4e016"></a>
## append_null

`function` · `arrow_array::builder::union_builder::UnionBuilder::append_null` · arrow-array 59.3.0

```rust
fn append_null<T: ArrowPrimitiveType>(&mut self, type_name: &str) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::union_builder::UnionBuilder", "path": "UnionBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [160, 1], "end": [360, 2], "filename": "src/builder/union_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/union_builder.rs:201`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends a null to this builder, encoding the null in the array
of the `type_name` child / field.

Since `UnionArray` encodes nulls as an entry in its children
(it doesn't have a validity bitmap itself), and where the null
is part of the final array, appending a NULL requires
specifying which field (child) to use.

<a id="op-b92a813bcceed5e182b6bd0e"></a>
## as_any

`function` · `arrow_array::builder::union_builder::UnionBuilder::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::union_builder::UnionBuilder", "path": "UnionBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [400, 2], "filename": "src/builder/union_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/union_builder.rs:387`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the builder as a non-mutable `Any` reference

<a id="op-bff2424097a1851fa605ca25"></a>
## as_any_mut

`function` · `arrow_array::builder::union_builder::UnionBuilder::as_any_mut` · arrow-array 59.3.0

```rust
fn as_any_mut(&mut self) -> &mut dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::union_builder::UnionBuilder", "path": "UnionBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [400, 2], "filename": "src/builder/union_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/union_builder.rs:392`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the builder as a mutable `Any` reference

<a id="op-9ffc85e1716896377f11fd14"></a>
## build

`function` · `arrow_array::builder::union_builder::UnionBuilder::build` · arrow-array 59.3.0

```rust
fn build(self) -> Result<UnionArray, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::union_builder::UnionBuilder", "path": "UnionBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [160, 1], "end": [360, 2], "filename": "src/builder/union_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/union_builder.rs:284`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds this builder creating a new `UnionArray`.

<a id="op-0d25b99d8e6a4d06ad73bc38"></a>
## default

`function` · `arrow_array::builder::union_builder::UnionBuilder::default` · arrow-array 59.3.0

```rust
fn default() -> UnionBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::union_builder::UnionBuilder", "path": "UnionBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 17], "end": [147, 24], "filename": "src/builder/union_builder.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/builder/union_builder.rs:147`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-780949145106d213e62436b0"></a>
## finish

`function` · `arrow_array::builder::union_builder::UnionBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::union_builder::UnionBuilder", "path": "UnionBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [400, 2], "filename": "src/builder/union_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/union_builder.rs:369`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array

<a id="op-ec2022f9a1af00cbba2cffe0"></a>
## finish_cloned

`function` · `arrow_array::builder::union_builder::UnionBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::union_builder::UnionBuilder", "path": "UnionBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [400, 2], "filename": "src/builder/union_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/union_builder.rs:378`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array without resetting the underlying builder

<a id="op-a3e06a71ddbd0c6f4f2de826"></a>
## fmt

`function` · `arrow_array::builder::union_builder::UnionBuilder::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::union_builder::UnionBuilder", "path": "UnionBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 10], "end": [147, 15], "filename": "src/builder/union_builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/builder/union_builder.rs:147`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3acdb8f0e41fb3124f23487a"></a>
## into_box_any

`function` · `arrow_array::builder::union_builder::UnionBuilder::into_box_any` · arrow-array 59.3.0

```rust
fn into_box_any(Box<self>) -> Box<dyn Any>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::union_builder::UnionBuilder", "path": "UnionBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [400, 2], "filename": "src/builder/union_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/union_builder.rs:397`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the boxed builder as a box of `Any`

<a id="op-445bb8994ddf21f7e5ee5d63"></a>
## len

`function` · `arrow_array::builder::union_builder::UnionBuilder::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::union_builder::UnionBuilder", "path": "UnionBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [400, 2], "filename": "src/builder/union_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/union_builder.rs:364`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the number of array slots in the builder

<a id="op-c2101d0c797a91ae1efdd620"></a>
## new_dense

`function` · `arrow_array::builder::union_builder::UnionBuilder::new_dense` · arrow-array 59.3.0

```rust
fn new_dense() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::union_builder::UnionBuilder", "path": "UnionBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [160, 1], "end": [360, 2], "filename": "src/builder/union_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/union_builder.rs:162`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new dense array builder.

<a id="op-9af4cd1ce277386d5de2c906"></a>
## new_sparse

`function` · `arrow_array::builder::union_builder::UnionBuilder::new_sparse` · arrow-array 59.3.0

```rust
fn new_sparse() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::union_builder::UnionBuilder", "path": "UnionBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [160, 1], "end": [360, 2], "filename": "src/builder/union_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/union_builder.rs:167`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new sparse array builder.

<a id="op-cfee3a16cdc61b7ac310adb9"></a>
## with_capacity_dense

`function` · `arrow_array::builder::union_builder::UnionBuilder::with_capacity_dense` · arrow-array 59.3.0

```rust
fn with_capacity_dense(capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::union_builder::UnionBuilder", "path": "UnionBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [160, 1], "end": [360, 2], "filename": "src/builder/union_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/union_builder.rs:172`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new dense array builder with capacity.

<a id="op-3a0344e79fca6cd6dedcd6d7"></a>
## with_capacity_sparse

`function` · `arrow_array::builder::union_builder::UnionBuilder::with_capacity_sparse` · arrow-array 59.3.0

```rust
fn with_capacity_sparse(capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::union_builder::UnionBuilder", "path": "UnionBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [160, 1], "end": [360, 2], "filename": "src/builder/union_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/union_builder.rs:183`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new sparse array builder  with capacity.
