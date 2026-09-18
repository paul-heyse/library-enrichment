# `arrow_array::builder::generic_bytes_dictionary_builder::LargeStringDictionaryBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.generic_bytes_dictionary_builder.LargeStringDictionaryBuilder.json).

<a id="op-ad90419017a5d74e35b254fa"></a>
## LargeStringDictionaryBuilder

`type_alias` · `arrow_array::builder::generic_bytes_dictionary_builder::LargeStringDictionaryBuilder` · arrow-array 59.3.0

```rust
type LargeStringDictionaryBuilder<K> = GenericByteDictionaryBuilder<K, types::GenericStringType<i64>>
```

Source: `src/builder/generic_bytes_dictionary_builder.rs:564`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builder for [`DictionaryArray`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47) of [`LargeStringArray`](crate::array::LargeStringArray)
