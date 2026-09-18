# `arrow_array::builder::generic_bytes_dictionary_builder::LargeBinaryDictionaryBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.generic_bytes_dictionary_builder.LargeBinaryDictionaryBuilder.json).

<a id="op-327b91e691179d787b7fd2ac"></a>
## LargeBinaryDictionaryBuilder

`type_alias` · `arrow_array::builder::generic_bytes_dictionary_builder::LargeBinaryDictionaryBuilder` · arrow-array 59.3.0

```rust
type LargeBinaryDictionaryBuilder<K> = GenericByteDictionaryBuilder<K, types::GenericBinaryType<i64>>
```

Source: `src/builder/generic_bytes_dictionary_builder.rs:602`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builder for [`DictionaryArray`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47) of [`LargeBinaryArray`](crate::array::LargeBinaryArray)
