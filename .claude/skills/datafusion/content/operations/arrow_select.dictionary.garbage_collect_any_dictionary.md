# `arrow_select::dictionary::garbage_collect_any_dictionary`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_select.dictionary.garbage_collect_any_dictionary.json).

<a id="op-35fc0df2ab43eacbf4150ee0"></a>
## garbage_collect_any_dictionary

`function` · `arrow_select::dictionary::garbage_collect_any_dictionary` · arrow-select 59.3.0

```rust
fn garbage_collect_any_dictionary(dictionary: &dyn AnyDictionaryArray) -> Result<arrow_array::ArrayRef, arrow_schema::ArrowError>
```

Source: `src/dictionary.rs:82`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Equivalent to [`garbage_collect_dictionary`](../operations/arrow_select.dictionary.garbage_collect_dictionary.md#op-43c77353e0d2ba4c79da9017) but without requiring casting to a specific key type.
