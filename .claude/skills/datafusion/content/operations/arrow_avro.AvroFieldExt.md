# `arrow_avro::AvroFieldExt`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.AvroFieldExt.json).

<a id="op-4ee9e1d496046a46d395aa02"></a>
## AvroFieldExt

`trait` · `arrow_avro::AvroFieldExt` · arrow-avro 59.3.0

```rust
trait AvroFieldExt
```

Source: `src/lib.rs:255`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Extension trait for AvroField to add Utf8View support

This trait adds methods for working with Utf8View support to the AvroField struct.

<a id="op-1d30c307eccc7f7b25fb0816"></a>
## with_utf8view

`function` · `arrow_avro::AvroFieldExt::with_utf8view` · arrow-avro 59.3.0

```rust
fn with_utf8view(&self) -> Self
```

Source: `src/lib.rs:259`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Returns a new field with Utf8View support enabled for string data

This will convert any string data to use StringViewArray instead of StringArray.
