# `arrow_array::iterator::GenericBinaryIter`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.iterator.GenericBinaryIter.json).

<a id="op-00edf2d0b15750ed651780f3"></a>
## GenericBinaryIter

`type_alias` · `arrow_array::iterator::GenericBinaryIter` · arrow-array 59.3.0

```rust
type GenericBinaryIter<'a, T> = ArrayIter<&'a array::GenericBinaryArray<T>>
```

Source: `src/iterator.rs:188`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

an iterator that returns Some(T) or None, that can be used on any BinaryArray
