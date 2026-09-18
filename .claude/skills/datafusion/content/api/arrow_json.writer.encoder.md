# `arrow_json::writer::encoder`

Crate `arrow-json` · 5 public items · structured records in [`model/arrow_json.writer.encoder.json`](../model/arrow_json.writer.encoder.json)

## make_encoder

`function` · `arrow_json::writer::encoder::make_encoder`

Also reachable as `arrow_json::writer::make_encoder`

```rust
fn make_encoder<'a>(field: &'a arrow_schema::FieldRef, array: &'a dyn Array, options: &'a EncoderOptions) -> Result<NullableEncoder<'a>, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_json.writer.encoder.make_encoder.md).


Creates an encoder for the given array and field.

This first calls the EncoderFactory if one is provided, and then falls back to the default encoders.

---

## EncoderOptions

`struct` · `arrow_json::writer::encoder::EncoderOptions`

Also reachable as `arrow::json::EncoderOptions`, `arrow_json::EncoderOptions`, `arrow_json::writer::EncoderOptions`

```rust
struct EncoderOptions
```

**Derives**: Clone, Debug, Default

**Methods** (16)

```rust
fn date_format(&self) -> Option<&str>
fn datetime_format(&self) -> Option<&str>
fn encoder_factory(&self) -> Option<&Arc<dyn EncoderFactory>>
fn explicit_nulls(&self) -> bool
fn struct_mode(&self) -> StructMode
fn time_format(&self) -> Option<&str>
fn timestamp_format(&self) -> Option<&str>
fn timestamp_tz_format(&self) -> Option<&str>
fn with_date_format(self, format: String) -> Self
fn with_datetime_format(self, format: String) -> Self
fn with_encoder_factory(self, encoder_factory: Arc<dyn EncoderFactory>) -> Self
fn with_explicit_nulls(self, explicit_nulls: bool) -> Self
fn with_struct_mode(self, struct_mode: StructMode) -> Self
fn with_time_format(self, format: String) -> Self
fn with_timestamp_format(self, format: String) -> Self
fn with_timestamp_tz_format(self, tz_format: String) -> Self
```

[Full member, field, variant and typed contracts](../operations/arrow_json.writer.encoder.EncoderOptions.md).


Configuration options for the JSON encoder.

---

## NullableEncoder

`struct` · `arrow_json::writer::encoder::NullableEncoder`

Also reachable as `arrow_json::writer::NullableEncoder`

```rust
struct NullableEncoder<'a>
```

**Implements**: `arrow_json::writer::encoder::Encoder`

**Methods** (4)

```rust
fn encode(&mut self, idx: usize, out: &mut Vec<u8>)
fn has_nulls(&self) -> bool
fn is_null(&self, idx: usize) -> bool
fn new(encoder: Box<dyn Encoder + 'a>, nulls: Option<NullBuffer>) -> Self
```

**via `arrow_json::writer::encoder::Encoder`**

```rust
fn encode(&mut self, idx: usize, out: &mut Vec<u8>)
```

[Full member, field, variant and typed contracts](../operations/arrow_json.writer.encoder.NullableEncoder.md).


An encoder + a null buffer.
This is packaged together into a wrapper struct to minimize dynamic dispatch for null checks.

---

## Encoder

`trait` · `arrow_json::writer::encoder::Encoder`

Also reachable as `arrow::json::Encoder`, `arrow_json::Encoder`, `arrow_json::writer::Encoder`

```rust
trait Encoder
```

**Implementors** (1)

- `arrow_json::writer::encoder::NullableEncoder`

**Methods** (1)

```rust
fn encode(&mut self, idx: usize, out: &mut Vec<u8>)
```

[Full member, field, variant and typed contracts](../operations/arrow_json.writer.encoder.Encoder.md).


A trait to format array values as JSON values

Nullability is handled by the caller to allow encoding nulls implicitly, i.e. `{}` instead of `{"a": null}`

---

## EncoderFactory

`trait` · `arrow_json::writer::encoder::EncoderFactory`

Also reachable as `arrow::json::EncoderFactory`, `arrow_json::EncoderFactory`, `arrow_json::writer::EncoderFactory`

```rust
trait EncoderFactory: std::fmt::Debug + Send + Sync
```

**Methods** (1)

```rust
fn make_default_encoder<'a>(&self, _field: &'a FieldRef, _array: &'a dyn Array, _options: &'a EncoderOptions) -> Result<Option<NullableEncoder<'a>>, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_json.writer.encoder.EncoderFactory.md).


A trait to create custom encoders for specific data types.

This allows overriding the default encoders for specific data types,
or adding new encoders for custom data types.

# Examples

```
use std::io::Write;
use arrow_array::{ArrayAccessor, Array, BinaryArray, Float64Array, RecordBatch};
use arrow_array::cast::AsArray;
use arrow_schema::{DataType, Field, Schema, FieldRef};
use arrow_json::{writer::{WriterBuilder, JsonArray, NullableEncoder}, StructMode};
use arrow_json::{Encoder, EncoderFactory, EncoderOptions};
use arrow_schema::ArrowError;
use std::sync::Arc;
use serde_json::json;
use serde_json::Value;

struct IntArrayBinaryEncoder<B> {
    array: B,
}

impl<'a, B> Encoder for IntArrayBinaryEncoder<B>
where
    B: ArrayAccessor<Item = &'a [u8]>,
{
    fn encode(&mut self, idx: usize, out: &mut Vec<u8>) {
        out.push(b'[');
        let child = self.array.value(idx);
        for (idx, byte) in child.iter().enumerate() {
            write!(out, "{byte}").unwrap();
            if idx < child.len() - 1 {
                out.push(b',');
            }
        }
        out.push(b']');
    }
}

#[derive(Debug)]
struct IntArayBinaryEncoderFactory;

impl EncoderFactory for IntArayBinaryEncoderFactory {
    fn make_default_encoder<'a>(
        &self,
        _field: &'a FieldRef,
        array: &'a dyn Array,
        _options: &'a EncoderOptions,
    ) -> Result<Option<NullableEncoder<'a>>, ArrowError> {
        match array.data_type() {
            DataType::Binary => {
                let array = array.as_binary::<i32>();
                let encoder = IntArrayBinaryEncoder { array };
                let array_encoder = Box::new(encoder) as Box<dyn Encoder + 'a>;
                let nulls = array.nulls().cloned();
                Ok(Some(NullableEncoder::new(array_encoder, nulls)))
            }
            _ => Ok(None),
        }
    }
}

let binary_array = BinaryArray::from_iter([Some(b"a".as_slice()), None, Some(b"b".as_slice())]);
let float_array = Float64Array::from(vec![Some(1.0), Some(2.3), None]);
let fields = vec![
    Field::new("bytes", DataType::Binary, true),
    Field::new("float", DataType::Float64, true),
];
let batch = RecordBatch::try_new(
    Arc::new(Schema::new(fields)),
    vec![
        Arc::new(binary_array) as Arc<dyn Array>,
        Arc::new(float_array) as Arc<dyn Array>,
    ],
)
.unwrap();

let json_value: Value = {
    let mut buf = Vec::new();
    let mut writer = WriterBuilder::new()
        .with_encoder_factory(Arc::new(IntArayBinaryEncoderFactory))
        .build::<_, JsonArray>(&mut buf);
    writer.write_batches(&[&batch]).unwrap();
    writer.finish().unwrap();
    serde_json::from_slice(&buf).unwrap()
};

let expected = json!([
    {"bytes": [97], "float": 1.0},
    {"float": 2.3},
    {"bytes": [98]},
]);

assert_eq!(json_value, expected);
```

---
