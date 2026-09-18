# `arrow_json::reader::tape`

Crate `arrow-json` · 1 public items · structured records in [`model/arrow_json.reader.tape.json`](../model/arrow_json.reader.tape.json)

## TapeElement

`enum` · `arrow_json::reader::tape::TapeElement`

```rust
enum TapeElement
```

**Variants**: `StartObject`, `EndObject`, `StartList`, `EndList`, `String`, `Number`, `I64`, `I32`, `F64`, `F32`, `True`, `False`, `Null`

[Full member, field, variant and typed contracts](../operations/arrow_json.reader.tape.TapeElement.md).


We decode JSON to a flattened tape representation,
allowing for efficient traversal of the JSON data

This approach is inspired by [simdjson]

Uses `u32` for offsets to ensure `TapeElement` is 64-bits. A future
iteration may increase this to a custom `u56` type.

[simdjson]: https://github.com/simdjson/simdjson/blob/master/doc/tape.md

---
