# `arrow_row::encode_dictionary_values`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_row.encode_dictionary_values.json).

<a id="op-e8afdb843d87dfaf7891b0b8"></a>
## encode_dictionary_values

`function` · `arrow_row::encode_dictionary_values` · arrow-row 59.3.0

```rust
fn encode_dictionary_values<K: ArrowDictionaryKeyType>(data: &mut [u8], offsets: &mut [usize], column: &DictionaryArray<K>, values: &Rows, null: &Row<'_>)
```

Source: `src/lib.rs:2169`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

Encode dictionary values not preserving the dictionary encoding
