# `arrow_array::array::BinaryArrayType`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.BinaryArrayType.json).

<a id="op-09d1839ef7529a23a2c36ead"></a>
## BinaryArrayType

`trait` · `arrow_array::array::BinaryArrayType` · arrow-array 59.3.0

```rust
trait BinaryArrayType<'a>: ArrayAccessor<Item = &'a [u8]> + Sized
```

Source: `src/array/mod.rs:728`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A trait for Arrow Binary Arrays, currently four types are supported:
- `BinaryArray`
- `LargeBinaryArray`
- `BinaryViewArray`
- `FixedSizeBinaryArray`

This trait helps to abstract over the different types of binary arrays
so that we don't need to duplicate the implementation for each type.

<a id="op-3ad40bce30df918dd31a4d9c"></a>
## iter

`function` · `arrow_array::array::BinaryArrayType::iter` · arrow-array 59.3.0

```rust
fn iter(&self) -> ArrayIter<Self>
```

Source: `src/array/mod.rs:730`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Constructs a new iterator
