# `arrow_array::array::StringArrayType`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.StringArrayType.json).

<a id="op-3a4a581f06cbd9f6f86913ba"></a>
## StringArrayType

`trait` · `arrow_array::array::StringArrayType` · arrow-array 59.3.0

```rust
trait StringArrayType<'a>: ArrayAccessor<Item = &'a str> + Sized
```

Source: `src/array/mod.rs:693`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A trait for Arrow String Arrays, currently three types are supported:
- `StringArray`
- `LargeStringArray`
- `StringViewArray`

This trait helps to abstract over the different types of string arrays
so that we don't need to duplicate the implementation for each type.

<a id="op-346f79c8c09f068d036689b3"></a>
## is_ascii

`function` · `arrow_array::array::StringArrayType::is_ascii` · arrow-array 59.3.0

```rust
fn is_ascii(&self) -> bool
```

Source: `src/array/mod.rs:695`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns true if all data within this string array is ASCII

<a id="op-fcdda29015984f33a45817b8"></a>
## iter

`function` · `arrow_array::array::StringArrayType::iter` · arrow-array 59.3.0

```rust
fn iter(&self) -> ArrayIter<Self>
```

Source: `src/array/mod.rs:698`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Constructs a new iterator
