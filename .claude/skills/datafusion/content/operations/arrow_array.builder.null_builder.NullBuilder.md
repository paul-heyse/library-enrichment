# `arrow_array::builder::null_builder::NullBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.null_builder.NullBuilder.json).

<a id="op-fb54bd68d765e7c89e7d9e07"></a>
## NullBuilder

`struct` · `arrow_array::builder::null_builder::NullBuilder` · arrow-array 59.3.0

```rust
struct NullBuilder
```

Source: `src/builder/null_builder.rs:46`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builder for [`NullArray`](../operations/arrow_array.array.null_array.NullArray.md#op-eccd805771d0b7ea8e20fabd)

# Example

Create a `NullArray` from a `NullBuilder`

```

# use arrow_array::{Array, NullArray, builder::NullBuilder};

let mut b = NullBuilder::new();
b.append_empty_value();
b.append_null();
b.append_nulls(3);
b.append_empty_values(3);
let arr = b.finish();

assert_eq!(8, arr.len());
assert_eq!(0, arr.null_count());
```

<a id="op-9719734dd96bf335b9d8e371"></a>
## append_empty_value

`function` · `arrow_array::builder::null_builder::NullBuilder::append_empty_value` · arrow-array 59.3.0

```rust
fn append_empty_value(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::null_builder::NullBuilder", "path": "NullBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [103, 2], "filename": "src/builder/null_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/null_builder.rs:76`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends a null slot into the builder

<a id="op-e453e9977bde8a57d7f5cc22"></a>
## append_empty_values

`function` · `arrow_array::builder::null_builder::NullBuilder::append_empty_values` · arrow-array 59.3.0

```rust
fn append_empty_values(&mut self, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::null_builder::NullBuilder", "path": "NullBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [103, 2], "filename": "src/builder/null_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/null_builder.rs:82`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends `n` `null`s into the builder.

<a id="op-e77242107704113177497cdd"></a>
## append_null

`function` · `arrow_array::builder::null_builder::NullBuilder::append_null` · arrow-array 59.3.0

```rust
fn append_null(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::null_builder::NullBuilder", "path": "NullBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [103, 2], "filename": "src/builder/null_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/null_builder.rs:64`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends a null slot into the builder

<a id="op-87f71caa31d1d9c4bbe501ad"></a>
## append_nulls

`function` · `arrow_array::builder::null_builder::NullBuilder::append_nulls` · arrow-array 59.3.0

```rust
fn append_nulls(&mut self, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::null_builder::NullBuilder", "path": "NullBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [103, 2], "filename": "src/builder/null_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/null_builder.rs:70`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends `n` `null`s into the builder.

<a id="op-f96671e9b914a471150c0529"></a>
## as_any

`function` · `arrow_array::builder::null_builder::NullBuilder::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::null_builder::NullBuilder", "path": "NullBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [135, 2], "filename": "src/builder/null_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/null_builder.rs:107`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the builder as a non-mutable `Any` reference.

<a id="op-6fee392a8bb6065968016644"></a>
## as_any_mut

`function` · `arrow_array::builder::null_builder::NullBuilder::as_any_mut` · arrow-array 59.3.0

```rust
fn as_any_mut(&mut self) -> &mut dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::null_builder::NullBuilder", "path": "NullBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [135, 2], "filename": "src/builder/null_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/null_builder.rs:112`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the builder as a mutable `Any` reference.

<a id="op-cf69f84573a2ebe23c85f001"></a>
## default

`function` · `arrow_array::builder::null_builder::NullBuilder::default` · arrow-array 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::null_builder::NullBuilder", "path": "NullBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [54, 2], "filename": "src/builder/null_builder.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/builder/null_builder.rs:51`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa4ebbd803b59b551a96d751"></a>
## finish

`function` · `arrow_array::builder::null_builder::NullBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::null_builder::NullBuilder", "path": "NullBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [135, 2], "filename": "src/builder/null_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/null_builder.rs:127`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array and reset this builder.

<a id="op-e312a467cc995b2aeeeb9b5d"></a>
## finish

`function` · `arrow_array::builder::null_builder::NullBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> NullArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::null_builder::NullBuilder", "path": "NullBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [103, 2], "filename": "src/builder/null_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/null_builder.rs:87`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the [NullArray](../operations/arrow_array.array.null_array.NullArray.md#op-eccd805771d0b7ea8e20fabd) and reset this builder.

<a id="op-11612b5ac6f92d5762003abb"></a>
## finish_cloned

`function` · `arrow_array::builder::null_builder::NullBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> NullArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::null_builder::NullBuilder", "path": "NullBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [103, 2], "filename": "src/builder/null_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/null_builder.rs:96`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the [NullArray](../operations/arrow_array.array.null_array.NullArray.md#op-eccd805771d0b7ea8e20fabd) without resetting the builder.

<a id="op-589f5c8ae2e565cb4c572966"></a>
## finish_cloned

`function` · `arrow_array::builder::null_builder::NullBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::null_builder::NullBuilder", "path": "NullBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [135, 2], "filename": "src/builder/null_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/null_builder.rs:132`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array without resetting the builder.

<a id="op-1a3baf0529fd12154e920938"></a>
## fmt

`function` · `arrow_array::builder::null_builder::NullBuilder::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::null_builder::NullBuilder", "path": "NullBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 10], "end": [45, 15], "filename": "src/builder/null_builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/builder/null_builder.rs:45`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3c60536f175e81bfaca74bb"></a>
## into_box_any

`function` · `arrow_array::builder::null_builder::NullBuilder::into_box_any` · arrow-array 59.3.0

```rust
fn into_box_any(Box<self>) -> Box<dyn Any>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::null_builder::NullBuilder", "path": "NullBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [135, 2], "filename": "src/builder/null_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/null_builder.rs:117`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the boxed builder as a box of `Any`.

<a id="op-7b6f1399fdcba618c70b4ec2"></a>
## len

`function` · `arrow_array::builder::null_builder::NullBuilder::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::null_builder::NullBuilder", "path": "NullBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [135, 2], "filename": "src/builder/null_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/null_builder.rs:122`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the number of array slots in the builder

<a id="op-0fc843b44a244d92f48db1a6"></a>
## new

`function` · `arrow_array::builder::null_builder::NullBuilder::new` · arrow-array 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::null_builder::NullBuilder", "path": "NullBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [103, 2], "filename": "src/builder/null_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/null_builder.rs:58`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new null builder
