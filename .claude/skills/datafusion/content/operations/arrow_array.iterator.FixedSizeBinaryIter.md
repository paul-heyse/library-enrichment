# `arrow_array::iterator::FixedSizeBinaryIter`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.iterator.FixedSizeBinaryIter.json).

<a id="op-18751fffbefce9091f439f7c"></a>
## FixedSizeBinaryIter

`type_alias` · `arrow_array::iterator::FixedSizeBinaryIter` · arrow-array 59.3.0

```rust
type FixedSizeBinaryIter<'a> = ArrayIter<&'a array::FixedSizeBinaryArray>
```

Source: `src/iterator.rs:190`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

an iterator that returns Some(T) or None, that can be used on any FixedSizeBinaryArray
