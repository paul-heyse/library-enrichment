# `arrow_array::array::string_array::StringArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.string_array.StringArray.json).

<a id="op-5d32f770159d415652a55945"></a>
## StringArray

`type_alias` · `arrow_array::array::string_array::StringArray` · arrow-array 59.3.0

```rust
type StringArray = GenericStringArray<i32>
```

Source: `src/array/string_array.rs:127`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`GenericStringArray`](../operations/arrow_array.array.string_array.GenericStringArray.md#op-86b79a80b65c0571602ced76) of `str` using `i32` offsets

# Examples

Construction

```
# use arrow_array::StringArray;
// Create from Vec<Option<&str>>
let arr = StringArray::from(vec![Some("foo"), Some("bar"), None, Some("baz")]);
// Create from Vec<&str>
let arr = StringArray::from(vec!["foo", "bar", "baz"]);
// Create from iter/collect (requires Option<&str>)
let arr: StringArray = std::iter::repeat(Some("foo")).take(10).collect();
```

Construction and Access

```
# use arrow_array::StringArray;
let array = StringArray::from(vec![Some("foo"), None, Some("bar")]);
assert_eq!(array.value(0), "foo");
```

See [`GenericByteArray`](../operations/arrow_array.array.byte_array.GenericByteArray.md#op-e39e3ecbe5a4126397f47443) for more information and examples
