# `arrow_json::writer::encoder::make_encoder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_json.writer.encoder.make_encoder.json).

<a id="op-680f06d8b419fe4fbe39040c"></a>
## make_encoder

`function` · `arrow_json::writer::encoder::make_encoder` · arrow-json 59.3.0

```rust
fn make_encoder<'a>(field: &'a arrow_schema::FieldRef, array: &'a dyn Array, options: &'a EncoderOptions) -> Result<NullableEncoder<'a>, arrow_schema::ArrowError>
```

Source: `src/writer/encoder.rs:307`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Creates an encoder for the given array and field.

This first calls the EncoderFactory if one is provided, and then falls back to the default encoders.
