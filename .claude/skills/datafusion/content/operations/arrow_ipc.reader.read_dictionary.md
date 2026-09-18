# `arrow_ipc::reader::read_dictionary`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.reader.read_dictionary.json).

<a id="op-d818681a4279d032c335242d"></a>
## read_dictionary

`function` · `arrow_ipc::reader::read_dictionary` · arrow-ipc 59.3.0

```rust
fn read_dictionary(buf: &arrow_buffer::Buffer, batch: DictionaryBatch<'_>, schema: &Schema, dictionaries_by_id: &mut std::collections::HashMap<i64, ArrayRef>, metadata: &MetadataVersion) -> Result<(), ArrowError>
```

Source: `src/reader.rs:783`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Read the dictionary from the buffer and provided metadata,
updating the `dictionaries_by_id` with the resulting dictionary
