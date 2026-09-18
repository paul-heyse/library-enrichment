# `arrow_array::iterator::GenericListViewArrayIter`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.iterator.GenericListViewArrayIter.json).

<a id="op-87779e76a5121da8f1ae34f7"></a>
## GenericListViewArrayIter

`type_alias` · `arrow_array::iterator::GenericListViewArrayIter` · arrow-array 59.3.0

```rust
type GenericListViewArrayIter<'a, O> = ArrayIter<&'a GenericListViewArray<O>>
```

Source: `src/iterator.rs:198`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

an iterator that returns Some(T) or None, that can be used on any ListArray
