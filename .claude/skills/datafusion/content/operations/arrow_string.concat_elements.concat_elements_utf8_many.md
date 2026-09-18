# `arrow_string::concat_elements::concat_elements_utf8_many`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_string.concat_elements.concat_elements_utf8_many.json).

<a id="op-657624776c1b5164ae81cebb"></a>
## concat_elements_utf8_many

`function` · `arrow_string::concat_elements::concat_elements_utf8_many` · arrow-string 59.3.0

```rust
fn concat_elements_utf8_many<Offset: OffsetSizeTrait>(arrays: &[&GenericStringArray<Offset>]) -> Result<GenericStringArray<Offset>, arrow_schema::ArrowError>
```

Source: `src/concat_elements.rs:111`. [Exact documentation build](https://docs.rs/crate/arrow-string/59.3.0/json).

Returns the elementwise concatenation of [`StringArray`](../operations/arrow_array.array.string_array.StringArray.md#op-5d32f770159d415652a55945).
```text
e.g:
  ["a", "b"] + [None, "c"] + [None, "d"] = [None, "bcd"]
```

An error will be returned if the [`StringArray`](../operations/arrow_array.array.string_array.StringArray.md#op-5d32f770159d415652a55945) are of different lengths
