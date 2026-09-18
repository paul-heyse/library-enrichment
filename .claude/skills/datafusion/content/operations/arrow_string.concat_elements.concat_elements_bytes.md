# `arrow_string::concat_elements::concat_elements_bytes`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_string.concat_elements.concat_elements_bytes.json).

<a id="op-87d8d5c41c0606509f38613d"></a>
## concat_elements_bytes

`function` · `arrow_string::concat_elements::concat_elements_bytes` · arrow-string 59.3.0

```rust
fn concat_elements_bytes<T: ByteArrayType>(left: &GenericByteArray<T>, right: &GenericByteArray<T>) -> Result<GenericByteArray<T>, arrow_schema::ArrowError>
```

Source: `src/concat_elements.rs:31`. [Exact documentation build](https://docs.rs/crate/arrow-string/59.3.0/json).

Returns the elementwise concatenation of a [`GenericByteArray`](../operations/arrow_array.array.byte_array.GenericByteArray.md#op-e39e3ecbe5a4126397f47443).
