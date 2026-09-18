# `arrow_ipc::reader::read_dictionary_impl`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.reader.read_dictionary_impl.json).

<a id="op-7f3e5921ab9fb85d18571e11"></a>
## read_dictionary_impl

`function` · `arrow_ipc::reader::read_dictionary_impl` · arrow-ipc 59.3.0

```rust
fn read_dictionary_impl(buf: &arrow_buffer::Buffer, batch: DictionaryBatch<'_>, schema: &Schema, dictionaries_by_id: &mut std::collections::HashMap<i64, ArrayRef>, metadata: &MetadataVersion, require_alignment: bool, skip_validation: arrow_data::UnsafeFlag) -> Result<(), ArrowError>
```

Source: `src/reader.rs:802`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Low-level version of [`read_dictionary`](../operations/arrow_ipc.reader.read_dictionary.md#op-d818681a4279d032c335242d) with alignment and validation controls
