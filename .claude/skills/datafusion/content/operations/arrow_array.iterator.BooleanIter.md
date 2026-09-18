# `arrow_array::iterator::BooleanIter`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.iterator.BooleanIter.json).

<a id="op-6faddd413b717882c0b850ce"></a>
## BooleanIter

`type_alias` · `arrow_array::iterator::BooleanIter` · arrow-array 59.3.0

```rust
type BooleanIter<'a> = ArrayIter<&'a array::BooleanArray>
```

Source: `src/iterator.rs:184`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

an iterator that returns Some(T) or None, that can be used on any BooleanArray
