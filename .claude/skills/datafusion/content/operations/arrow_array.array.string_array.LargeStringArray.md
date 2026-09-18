# `arrow_array::array::string_array::LargeStringArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.string_array.LargeStringArray.json).

<a id="op-829f12b9a45fbfe752a7ee52"></a>
## LargeStringArray

`type_alias` · `arrow_array::array::string_array::LargeStringArray` · arrow-array 59.3.0

```rust
type LargeStringArray = GenericStringArray<i64>
```

Source: `src/array/string_array.rs:154`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`GenericStringArray`](../operations/arrow_array.array.string_array.GenericStringArray.md#op-86b79a80b65c0571602ced76) of `str` using `i64` offsets

# Examples

Construction

```
# use arrow_array::LargeStringArray;
// Create from Vec<Option<&str>>
let arr = LargeStringArray::from(vec![Some("foo"), Some("bar"), None, Some("baz")]);
// Create from Vec<&str>
let arr = LargeStringArray::from(vec!["foo", "bar", "baz"]);
// Create from iter/collect (requires Option<&str>)
let arr: LargeStringArray = std::iter::repeat(Some("foo")).take(10).collect();
```

Construction and Access

```
use arrow_array::LargeStringArray;
let array = LargeStringArray::from(vec![Some("foo"), None, Some("bar")]);
assert_eq!(array.value(2), "bar");
```

See [`GenericByteArray`](../operations/arrow_array.array.byte_array.GenericByteArray.md#op-e39e3ecbe5a4126397f47443) for more information and examples
