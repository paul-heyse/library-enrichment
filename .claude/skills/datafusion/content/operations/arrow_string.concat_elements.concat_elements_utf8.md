# `arrow_string::concat_elements::concat_elements_utf8`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_string.concat_elements.concat_elements_utf8.json).

<a id="op-80cf20396d713e416030c1ec"></a>
## concat_elements_utf8

`function` · `arrow_string::concat_elements::concat_elements_utf8` · arrow-string 59.3.0

```rust
fn concat_elements_utf8<Offset: OffsetSizeTrait>(left: &GenericStringArray<Offset>, right: &GenericStringArray<Offset>) -> Result<GenericStringArray<Offset>, arrow_schema::ArrowError>
```

Source: `src/concat_elements.rs:89`. [Exact documentation build](https://docs.rs/crate/arrow-string/59.3.0/json).

Returns the elementwise concatenation of a [`GenericStringArray`](../operations/arrow_array.array.string_array.GenericStringArray.md#op-86b79a80b65c0571602ced76).

An index of the resulting [`GenericStringArray`](../operations/arrow_array.array.string_array.GenericStringArray.md#op-86b79a80b65c0571602ced76) is null if any of
`StringArray` are null at that location.

```text
e.g:

  ["Hello"] + ["World"] = ["HelloWorld"]

  ["a", "b"] + [None, "c"] = [None, "bc"]
```

An error will be returned if `left` and `right` have different lengths
