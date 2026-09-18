# `arrow_array::iterator::MapArrayIter`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.iterator.MapArrayIter.json).

<a id="op-28361c2f1a47d9fa5ee087d1"></a>
## MapArrayIter

`type_alias` · `arrow_array::iterator::MapArrayIter` · arrow-array 59.3.0

```rust
type MapArrayIter<'a> = ArrayIter<&'a MapArray>
```

Source: `src/iterator.rs:196`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

an iterator that returns Some(T) or None, that can be used on any MapArray
