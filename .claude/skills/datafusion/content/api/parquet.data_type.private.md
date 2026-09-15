# `parquet::data_type::private`

Crate `parquet` · 1 public items · structured records in [`model/parquet.data_type.private.json`](../model/parquet.data_type.private.json)

## ParquetValueType

`trait` · `parquet::data_type::private::ParquetValueType`

```rust
trait ParquetValueType: PartialEq + std::fmt::Debug + std::fmt::Display + Default + Clone + super::AsBytes + super::FromBytes + SliceAsBytes + PartialOrd + Send + HeapSize + encodings::decoding::private::GetDecoder + file::statistics::private::MakeStatistics
```

**Methods** (11)

```rust
fn as_any(&self) -> &dyn std::any::Any
fn as_i64(&self) -> Result<i64>
fn as_mut_any(&mut self) -> &mut dyn std::any::Any
fn as_u64(&self) -> Result<u64>
fn decode(buffer: &mut [Self], decoder: &mut PlainDecoderDetails) -> Result<usize>
fn dict_encoding_size(&self) -> (usize, usize)
fn encode<W: std::io::Write>(values: &[Self], writer: &mut W, bit_writer: &mut BitWriter) -> Result<()>
fn set_data(decoder: &mut PlainDecoderDetails, data: Bytes, num_values: usize)
fn set_from_bytes(&mut self, _data: Bytes)
fn skip(decoder: &mut PlainDecoderDetails, num_values: usize) -> Result<usize>
fn variable_length_bytes(_: &[Self]) -> Option<i64>
```

Sealed trait to start to remove specialisation from implementations

This is done to force the associated value type to be unimplementable outside of this
crate, and thus hint to the type system (and end user) traits are public for the contract
and not for extension.

---
