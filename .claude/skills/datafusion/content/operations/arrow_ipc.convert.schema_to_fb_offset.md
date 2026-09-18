# `arrow_ipc::convert::schema_to_fb_offset`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.convert.schema_to_fb_offset.json).

<a id="op-8f67a23b7c6547b5c3ca6e1f"></a>
## schema_to_fb_offset

`function` · `arrow_ipc::convert::schema_to_fb_offset` · arrow-ipc 59.3.0

```rust
fn schema_to_fb_offset<'a>(fbb: &mut flatbuffers::FlatBufferBuilder<'a>, schema: &Schema) -> flatbuffers::WIPOffset<Schema<'a>>
```

Source: `src/convert.rs:155`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Adds a [Schema](../operations/arrow_schema.schema.Schema.md#op-144709aa539d6163483c2050) to a flatbuffer and returns the offset
