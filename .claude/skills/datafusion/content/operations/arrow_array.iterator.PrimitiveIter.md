# `arrow_array::iterator::PrimitiveIter`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.iterator.PrimitiveIter.json).

<a id="op-e77cc359542e643499429ffe"></a>
## PrimitiveIter

`type_alias` · `arrow_array::iterator::PrimitiveIter` · arrow-array 59.3.0

```rust
type PrimitiveIter<'a, T> = ArrayIter<&'a array::PrimitiveArray<T>>
```

Source: `src/iterator.rs:182`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

an iterator that returns Some(T) or None, that can be used on any PrimitiveArray
