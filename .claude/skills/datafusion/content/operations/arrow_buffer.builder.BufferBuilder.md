# `arrow_buffer::builder::BufferBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.builder.BufferBuilder.json).

<a id="op-d23128d312d259aad0f09763"></a>
## BufferBuilder

`struct` · `arrow_buffer::builder::BufferBuilder` · arrow-buffer 59.3.0

```rust
struct BufferBuilder<T: ArrowNativeType>
```

Source: `src/builder/mod.rs:59`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Builder for creating Arrow [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b) objects

A [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b) is the underlying data structure of Arrow's Arrays.

For all supported types, there are type definitions for the
generic version of `BufferBuilder<T>`, e.g. `BufferBuilder`.

**Note it is typically faster to create buffers directly from `Vec`**.
See example on [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b).

# See Also
* [`BooleanBufferBuilder`](../operations/arrow_buffer.builder.boolean.BooleanBufferBuilder.md#op-3b603b0bd38714441734f47c): for packing bits in [`BooleanBuffer`]s
* [`NullBufferBuilder`](../operations/arrow_buffer.builder.null.NullBufferBuilder.md#op-f629e701548be729fb00ab9e): for creating [`NullBuffer`]s of null values

[`BooleanBuffer`]: crate::BooleanBuffer
[`NullBuffer`]: crate::NullBuffer

# Example:

```
# use arrow_buffer::builder::BufferBuilder;
let mut builder = BufferBuilder::<u8>::new(100);
builder.append_slice(&[42, 43, 44]);
builder.append(45);
let buffer = builder.finish();
assert_eq!(unsafe { buffer.typed_data::<u8>() }, &[42, 43, 44, 45]);
```

<a id="op-aa111d7699bc82c2240129ac"></a>
## advance

`function` · `arrow_buffer::builder::BufferBuilder::advance` · arrow-buffer 59.3.0

```rust
fn advance(&mut self, i: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::builder::BufferBuilder", "path": "BufferBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [370, 2], "filename": "src/builder/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/mod.rs:163`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Increases the number of elements in the internal buffer by `n`
and resizes the buffer as needed.

The values of the newly added elements are 0.
This method is usually used when appending `NULL` values to the buffer
as they still require physical memory space.

# Example:

```
# use arrow_buffer::builder::BufferBuilder;
let mut builder = BufferBuilder::<u8>::new(10);
builder.advance(2);

assert_eq!(builder.len(), 2);
```

<a id="op-6e4751748befe9d3358bcc2f"></a>
## append

`function` · `arrow_buffer::builder::BufferBuilder::append` · arrow-buffer 59.3.0

```rust
fn append(&mut self, v: T)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::builder::BufferBuilder", "path": "BufferBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [370, 2], "filename": "src/builder/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/mod.rs:196`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Appends a value of type `T` into the builder,
growing the internal buffer as needed.

# Example:

```
# use arrow_buffer::builder::BufferBuilder;
let mut builder = BufferBuilder::<u8>::new(10);
builder.append(42);

assert_eq!(builder.len(), 1);
```

<a id="op-4705f9f82ad32e6b404478fe"></a>
## append_n

`function` · `arrow_buffer::builder::BufferBuilder::append_n` · arrow-buffer 59.3.0

```rust
fn append_n(&mut self, n: usize, v: T)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::builder::BufferBuilder", "path": "BufferBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [370, 2], "filename": "src/builder/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/mod.rs:214`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Appends a value of type `T` into the builder N times,
growing the internal buffer as needed.

# Example:

```
# use arrow_buffer::builder::BufferBuilder;
let mut builder = BufferBuilder::<u8>::new(10);
builder.append_n(10, 42);

assert_eq!(builder.len(), 10);
```

<a id="op-6b666c78996f02f104ae4fcc"></a>
## append_n_zeroed

`function` · `arrow_buffer::builder::BufferBuilder::append_n_zeroed` · arrow-buffer 59.3.0

```rust
fn append_n_zeroed(&mut self, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::builder::BufferBuilder", "path": "BufferBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [370, 2], "filename": "src/builder/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/mod.rs:232`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Appends `n`, zero-initialized values

# Example:

```
# use arrow_buffer::builder::BufferBuilder;
let mut builder = BufferBuilder::<u32>::new(10);
builder.append_n_zeroed(3);

assert_eq!(builder.len(), 3);
assert_eq!(builder.as_slice(), &[0, 0, 0])
```

<a id="op-28eb6cfc62c411d15b334bfa"></a>
## append_slice

`function` · `arrow_buffer::builder::BufferBuilder::append_slice` · arrow-buffer 59.3.0

```rust
fn append_slice(&mut self, slice: &[T])
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::builder::BufferBuilder", "path": "BufferBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [370, 2], "filename": "src/builder/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/mod.rs:248`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Appends a slice of type `T`, growing the internal buffer as needed.

# Example:

```
# use arrow_buffer::builder::BufferBuilder;
let mut builder = BufferBuilder::<u8>::new(10);
builder.append_slice(&[42, 44, 46]);

assert_eq!(builder.len(), 3);
```

<a id="op-145478c191b94884adc17165"></a>
## append_trusted_len_iter

`function` · `arrow_buffer::builder::BufferBuilder::append_trusted_len_iter` · arrow-buffer 59.3.0

```rust
unsafe fn append_trusted_len_iter(&mut self, iter: impl IntoIterator<Item = T>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::builder::BufferBuilder", "path": "BufferBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [370, 2], "filename": "src/builder/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/mod.rs:324`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

# Safety
This requires the iterator be a trusted length. This could instead require
the iterator implement `TrustedLen` once that is stabilized.

<a id="op-82a33939a2dd28e77fc5ccb8"></a>
## as_slice

`function` · `arrow_buffer::builder::BufferBuilder::as_slice` · arrow-buffer 59.3.0

```rust
fn as_slice(&self) -> &[T]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::builder::BufferBuilder", "path": "BufferBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [370, 2], "filename": "src/builder/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/mod.rs:263`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

View the contents of this buffer as a slice

```
# use arrow_buffer::builder::BufferBuilder;
let mut builder = BufferBuilder::<f64>::new(10);
builder.append(1.3);
builder.append_n(2, 2.3);

assert_eq!(builder.as_slice(), &[1.3, 2.3, 2.3]);
```

<a id="op-e6b395866335b721c1d628f4"></a>
## as_slice_mut

`function` · `arrow_buffer::builder::BufferBuilder::as_slice_mut` · arrow-buffer 59.3.0

```rust
fn as_slice_mut(&mut self) -> &mut [T]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::builder::BufferBuilder", "path": "BufferBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [370, 2], "filename": "src/builder/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/mod.rs:287`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

View the contents of this buffer as a mutable slice

# Example:

```
# use arrow_buffer::builder::BufferBuilder;
let mut builder = BufferBuilder::<f32>::new(10);

builder.append_slice(&[1., 2., 3.4]);
assert_eq!(builder.as_slice(), &[1., 2., 3.4]);

builder.as_slice_mut()[1] = 4.2;
assert_eq!(builder.as_slice(), &[1., 4.2, 3.4]);
```

<a id="op-92edc3acde55851057589bb0"></a>
## build

`function` · `arrow_buffer::builder::BufferBuilder::build` · arrow-buffer 59.3.0

```rust
fn build(self) -> Buffer
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::builder::BufferBuilder", "path": "BufferBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [370, 2], "filename": "src/builder/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/mod.rs:367`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Builds an immutable [Buffer](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b) without resetting the builder.

This consumes the builder. Use [`Self::finish`](../operations/arrow_buffer.builder.BufferBuilder.md#op-117882d41f14f4ed91f23d22) to reuse it.

# Example:

```
# use arrow_buffer::builder::BufferBuilder;
let mut builder = BufferBuilder::<u8>::new(10);
builder.append_slice(&[42, 44, 46]);
let buffer = builder.build();
assert_eq!(unsafe { buffer.typed_data::<u8>() }, &[42, 44, 46]);
```

<a id="op-aa99c6ed476a67c7107de7af"></a>
## capacity

`function` · `arrow_buffer::builder::BufferBuilder::capacity` · arrow-buffer 59.3.0

```rust
fn capacity(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::builder::BufferBuilder", "path": "BufferBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [370, 2], "filename": "src/builder/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/mod.rs:141`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the actual capacity (number of elements) of the internal buffer.

Note: the internal capacity returned by this method might be larger than
what you'd expect after setting the capacity in the `new()` or `reserve()`
functions.

<a id="op-7aa075d5d5af7802c09f54d4"></a>
## default

`function` · `arrow_buffer::builder::BufferBuilder::default` · arrow-buffer 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::builder::BufferBuilder", "path": "BufferBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [372, 1], "end": [376, 2], "filename": "src/builder/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/builder/mod.rs:373`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db7073c17b0e6f7b0dcd3aae"></a>
## extend

`function` · `arrow_buffer::builder::BufferBuilder::extend` · arrow-buffer 59.3.0

```rust
fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::builder::BufferBuilder", "path": "BufferBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [378, 1], "end": [382, 2], "filename": "src/builder/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "core::iter::traits::collect::Extend", "path": "Extend"}, "trait_path": "core::iter::traits::collect::Extend"}`

Source: `src/builder/mod.rs:379`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-117882d41f14f4ed91f23d22"></a>
## finish

`function` · `arrow_buffer::builder::BufferBuilder::finish` · arrow-buffer 59.3.0

```rust
fn finish(&mut self) -> Buffer
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::builder::BufferBuilder", "path": "BufferBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [370, 2], "filename": "src/builder/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/mod.rs:348`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Resets this builder and returns an immutable [Buffer](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b).

Use [`Self::build`](../operations/arrow_buffer.builder.BufferBuilder.md#op-92edc3acde55851057589bb0) when you don't need to reuse this builder.

# Example:

```
# use arrow_buffer::builder::BufferBuilder;
let mut builder = BufferBuilder::<u8>::new(10);
builder.append_slice(&[42, 44, 46]);
let buffer = builder.finish();
assert_eq!(unsafe { buffer.typed_data::<u8>() }, &[42, 44, 46]);
```

<a id="op-1bf3bfaaf759464a6969bb4c"></a>
## fmt

`function` · `arrow_buffer::builder::BufferBuilder::fmt` · arrow-buffer 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::builder::BufferBuilder", "path": "BufferBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 10], "end": [58, 15], "filename": "src/builder/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/builder/mod.rs:58`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6833a869d34eb305196766da"></a>
## from

`function` · `arrow_buffer::builder::BufferBuilder::from` · arrow-buffer 59.3.0

```rust
fn from(value: Vec<T>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::builder::BufferBuilder", "path": "BufferBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [384, 1], "end": [391, 2], "filename": "src/builder/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/builder/mod.rs:385`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81516d7a2ca49dfa64bab924"></a>
## from_iter

`function` · `arrow_buffer::builder::BufferBuilder::from_iter` · arrow-buffer 59.3.0

```rust
fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::builder::BufferBuilder", "path": "BufferBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [393, 1], "end": [399, 2], "filename": "src/builder/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/builder/mod.rs:394`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ea504fa45e1b438c0c945b8"></a>
## is_empty

`function` · `arrow_buffer::builder::BufferBuilder::is_empty` · arrow-buffer 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::builder::BufferBuilder", "path": "BufferBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [370, 2], "filename": "src/builder/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/mod.rs:132`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns whether the internal buffer is empty.

# Example:

```
# use arrow_buffer::builder::BufferBuilder;
let mut builder = BufferBuilder::<u8>::new(10);
builder.append(42);

assert_eq!(builder.is_empty(), false);
```

<a id="op-7e361686a482963be0127573"></a>
## len

`function` · `arrow_buffer::builder::BufferBuilder::len` · arrow-buffer 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::builder::BufferBuilder", "path": "BufferBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [370, 2], "filename": "src/builder/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/mod.rs:117`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the current number of array elements in the internal buffer.

# Example:

```
# use arrow_buffer::builder::BufferBuilder;
let mut builder = BufferBuilder::<u8>::new(10);
builder.append(42);

assert_eq!(builder.len(), 1);
```

<a id="op-ba1b352fdd04a57ff811dd8d"></a>
## new

`function` · `arrow_buffer::builder::BufferBuilder::new` · arrow-buffer 59.3.0

```rust
fn new(capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::builder::BufferBuilder", "path": "BufferBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [370, 2], "filename": "src/builder/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/mod.rs:85`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Creates a new builder with initial capacity for _at least_ `capacity`
elements of type `T`.

The capacity can later be manually adjusted with the
[`reserve()`](BufferBuilder::reserve) method.
Also the
[`append()`](BufferBuilder::append),
[`append_slice()`](BufferBuilder::append_slice) and
[`advance()`](BufferBuilder::advance)
methods automatically increase the capacity if needed.

# Example:

```
# use arrow_buffer::builder::BufferBuilder;
let mut builder = BufferBuilder::<u8>::new(10);

assert!(builder.capacity() >= 10);
```

<a id="op-965135cfde77554a8600fe95"></a>
## new_from_buffer

`function` · `arrow_buffer::builder::BufferBuilder::new_from_buffer` · arrow-buffer 59.3.0

```rust
unsafe fn new_from_buffer(buffer: MutableBuffer) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::builder::BufferBuilder", "path": "BufferBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [370, 2], "filename": "src/builder/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/mod.rs:99`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Creates a new builder from a [`MutableBuffer`](../operations/arrow_buffer.buffer.mutable.MutableBuffer.md#op-c6098a137f542fbc611c6673)

# Safety

- `buffer` bytes must be aligned to type `T`

<a id="op-846bed9026e6e86d5ef8959c"></a>
## reserve

`function` · `arrow_buffer::builder::BufferBuilder::reserve` · arrow-buffer 59.3.0

```rust
fn reserve(&mut self, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::builder::BufferBuilder", "path": "BufferBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [370, 2], "filename": "src/builder/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/mod.rs:179`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Reserves memory for _at least_ `n` more elements of type `T`.

# Example:

```
# use arrow_buffer::builder::BufferBuilder;
let mut builder = BufferBuilder::<u8>::new(10);
builder.reserve(10);

assert!(builder.capacity() >= 20);
```

<a id="op-463696634e1fc4ea9fc8f24a"></a>
## truncate

`function` · `arrow_buffer::builder::BufferBuilder::truncate` · arrow-buffer 59.3.0

```rust
fn truncate(&mut self, len: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::builder::BufferBuilder", "path": "BufferBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [370, 2], "filename": "src/builder/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/mod.rs:316`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Shorten this BufferBuilder to `len` items

If `len` is greater than the builder's current length, this has no effect

# Example:

```
# use arrow_buffer::builder::BufferBuilder;
let mut builder = BufferBuilder::<u16>::new(10);

builder.append_slice(&[42, 44, 46]);
assert_eq!(builder.as_slice(), &[42, 44, 46]);

builder.truncate(2);
assert_eq!(builder.as_slice(), &[42, 44]);

builder.append(12);
assert_eq!(builder.as_slice(), &[42, 44, 12]);
```
