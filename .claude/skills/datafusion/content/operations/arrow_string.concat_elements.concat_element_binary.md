# `arrow_string::concat_elements::concat_element_binary`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_string.concat_elements.concat_element_binary.json).

<a id="op-8a95200e25921cc63e0d6604"></a>
## concat_element_binary

`function` · `arrow_string::concat_elements::concat_element_binary` · arrow-string 59.3.0

```rust
fn concat_element_binary<Offset: OffsetSizeTrait>(left: &GenericBinaryArray<Offset>, right: &GenericBinaryArray<Offset>) -> Result<GenericBinaryArray<Offset>, arrow_schema::ArrowError>
```

Source: `src/concat_elements.rs:97`. [Exact documentation build](https://docs.rs/crate/arrow-string/59.3.0/json).

Returns the elementwise concatenation of a [`GenericBinaryArray`](../operations/arrow_array.array.binary_array.GenericBinaryArray.md#op-faf606314f632d89aa2152d6).
