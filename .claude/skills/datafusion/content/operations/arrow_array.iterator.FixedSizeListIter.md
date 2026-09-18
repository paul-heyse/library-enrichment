# `arrow_array::iterator::FixedSizeListIter`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.iterator.FixedSizeListIter.json).

<a id="op-329201a696e01bedb83848d9"></a>
## FixedSizeListIter

`type_alias` · `arrow_array::iterator::FixedSizeListIter` · arrow-array 59.3.0

```rust
type FixedSizeListIter<'a> = ArrayIter<&'a FixedSizeListArray>
```

Source: `src/iterator.rs:192`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

an iterator that returns Some(T) or None, that can be used on any FixedSizeListArray
