# `arrow_select::dictionary`

Crate `arrow-select` · 2 public items · structured records in [`model/arrow_select.dictionary.json`](../model/arrow_select.dictionary.json)

## garbage_collect_any_dictionary

`function` · `arrow_select::dictionary::garbage_collect_any_dictionary`

```rust
fn garbage_collect_any_dictionary(dictionary: &dyn AnyDictionaryArray) -> Result<arrow_array::ArrayRef, arrow_schema::ArrowError>
```

Equivalent to [`garbage_collect_dictionary`] but without requiring casting to a specific key type.

---

## garbage_collect_dictionary

`function` · `arrow_select::dictionary::garbage_collect_dictionary`

```rust
fn garbage_collect_dictionary<K: ArrowDictionaryKeyType>(dictionary: &arrow_array::DictionaryArray<K>) -> Result<arrow_array::DictionaryArray<K>, arrow_schema::ArrowError>
```

Garbage collects a [DictionaryArray] by removing unreferenced values.

Returns a new [DictionaryArray] such that there are no values
that are not referenced by at least one key. There may still be duplicate
values.

See also [`garbage_collect_any_dictionary`] if you need to handle multiple dictionary types

---
