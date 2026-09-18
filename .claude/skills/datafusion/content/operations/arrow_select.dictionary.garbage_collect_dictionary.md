# `arrow_select::dictionary::garbage_collect_dictionary`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_select.dictionary.garbage_collect_dictionary.json).

<a id="op-43c77353e0d2ba4c79da9017"></a>
## garbage_collect_dictionary

`function` · `arrow_select::dictionary::garbage_collect_dictionary` · arrow-select 59.3.0

```rust
fn garbage_collect_dictionary<K: ArrowDictionaryKeyType>(dictionary: &arrow_array::DictionaryArray<K>) -> Result<arrow_array::DictionaryArray<K>, arrow_schema::ArrowError>
```

Source: `src/dictionary.rs:45`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Garbage collects a [DictionaryArray](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47) by removing unreferenced values.

Returns a new [DictionaryArray](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47) such that there are no values
that are not referenced by at least one key. There may still be duplicate
values.

See also [`garbage_collect_any_dictionary`](../operations/arrow_select.dictionary.garbage_collect_any_dictionary.md#op-35fc0df2ab43eacbf4150ee0) if you need to handle multiple dictionary types
