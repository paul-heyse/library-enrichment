# `arrow_array::iterator::GenericListArrayIter`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.iterator.GenericListArrayIter.json).

<a id="op-18bb195caad7b42f6ae5535d"></a>
## GenericListArrayIter

`type_alias` · `arrow_array::iterator::GenericListArrayIter` · arrow-array 59.3.0

```rust
type GenericListArrayIter<'a, O> = ArrayIter<&'a array::GenericListArray<O>>
```

Source: `src/iterator.rs:194`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

an iterator that returns Some(T) or None, that can be used on any ListArray
