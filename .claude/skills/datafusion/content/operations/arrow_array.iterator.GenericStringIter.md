# `arrow_array::iterator::GenericStringIter`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.iterator.GenericStringIter.json).

<a id="op-f30a0283868020a6973e1f8e"></a>
## GenericStringIter

`type_alias` · `arrow_array::iterator::GenericStringIter` · arrow-array 59.3.0

```rust
type GenericStringIter<'a, T> = ArrayIter<&'a array::GenericStringArray<T>>
```

Source: `src/iterator.rs:186`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

an iterator that returns Some(T) or None, that can be used on any Utf8Array
