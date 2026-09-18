# `parquet::data_type::private::ParquetValueType`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.data_type.private.ParquetValueType.json).

<a id="op-7100c06b1a038aff2a09e63e"></a>
## ParquetValueType

`trait` · `parquet::data_type::private::ParquetValueType` · parquet 59.3.0

```rust
trait ParquetValueType: PartialEq + std::fmt::Debug + std::fmt::Display + Default + Clone + super::AsBytes + super::FromBytes + SliceAsBytes + PartialOrd + Send + HeapSize + encodings::decoding::private::GetDecoder + file::statistics::private::MakeStatistics
```

Source: `src/data_type.rs:692`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sealed trait to start to remove specialisation from implementations

This is done to force the associated value type to be unimplementable outside of this
crate, and thus hint to the type system (and end user) traits are public for the contract
and not for extension.

<a id="op-83906cb0f44259dba19fecb3"></a>
## PHYSICAL_TYPE

`assoc_const` · `parquet::data_type::private::ParquetValueType::PHYSICAL_TYPE` · parquet 59.3.0

```rust
PHYSICAL_TYPE
```

Source: `src/data_type.rs:707`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bbffae86b1de3c35a9ce586a"></a>
## as_any

`function` · `parquet::data_type::private::ParquetValueType::as_any` · parquet 59.3.0

```rust
fn as_any(&self) -> &dyn std::any::Any
```

Source: `src/data_type.rs:755`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Return the value as an Any to allow for downcasts without transmutation

<a id="op-22af91a41b48bba28c31caa4"></a>
## as_i64

`function` · `parquet::data_type::private::ParquetValueType::as_i64` · parquet 59.3.0

```rust
fn as_i64(&self) -> Result<i64>
```

Source: `src/data_type.rs:740`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Return the value as i64 if possible

This is essentially the same as `std::convert::TryInto<i64>` but can't be
implemented for `f32` and `f64`, types that would fail orphan rules

<a id="op-80d687818c0fbc8ff581a30d"></a>
## as_mut_any

`function` · `parquet::data_type::private::ParquetValueType::as_mut_any` · parquet 59.3.0

```rust
fn as_mut_any(&mut self) -> &mut dyn std::any::Any
```

Source: `src/data_type.rs:758`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Return the value as an mutable Any to allow for downcasts without transmutation

<a id="op-87c2f573640353d81de31990"></a>
## as_u64

`function` · `parquet::data_type::private::ParquetValueType::as_u64` · parquet 59.3.0

```rust
fn as_u64(&self) -> Result<u64>
```

Source: `src/data_type.rs:748`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Return the value as u64 if possible

This is essentially the same as `std::convert::TryInto<u64>` but can't be
implemented for `f32` and `f64`, types that would fail orphan rules

<a id="op-e4dbe4b73a733aa7e9fcf3f6"></a>
## decode

`function` · `parquet::data_type::private::ParquetValueType::decode` · parquet 59.3.0

```rust
fn decode(buffer: &mut [Self], decoder: &mut PlainDecoderDetails) -> Result<usize>
```

Source: `src/data_type.rs:720`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Decode the value from a given buffer for a higher level decoder

<a id="op-d8211b1ef3a7740a3f1031d0"></a>
## dict_encoding_size

`function` · `parquet::data_type::private::ParquetValueType::dict_encoding_size` · parquet 59.3.0

```rust
fn dict_encoding_size(&self) -> (usize, usize)
```

Source: `src/data_type.rs:725`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Return the encoded size for a type

<a id="op-8a27c5b547714d7f4bd674c7"></a>
## encode

`function` · `parquet::data_type::private::ParquetValueType::encode` · parquet 59.3.0

```rust
fn encode<W: std::io::Write>(values: &[Self], writer: &mut W, bit_writer: &mut BitWriter) -> Result<()>
```

Source: `src/data_type.rs:710`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Encode the value directly from a higher level encoder

<a id="op-8348c01c29f1ffa8277d267a"></a>
## set_data

`function` · `parquet::data_type::private::ParquetValueType::set_data` · parquet 59.3.0

```rust
fn set_data(decoder: &mut PlainDecoderDetails, data: Bytes, num_values: usize)
```

Source: `src/data_type.rs:717`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Establish the data that will be decoded in a buffer

<a id="op-7049c3f061b4e241063375a7"></a>
## set_from_bytes

`function` · `parquet::data_type::private::ParquetValueType::set_from_bytes` · parquet 59.3.0

```rust
fn set_from_bytes(&mut self, _data: Bytes)
```

Source: `src/data_type.rs:763`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the value of this object from the provided [`Bytes`]

Only implemented for `ByteArray` and `FixedLenByteArray`. Will panic for other types.

<a id="op-bf6bf934c2ed4ec8fcfdba54"></a>
## skip

`function` · `parquet::data_type::private::ParquetValueType::skip` · parquet 59.3.0

```rust
fn skip(decoder: &mut PlainDecoderDetails, num_values: usize) -> Result<usize>
```

Source: `src/data_type.rs:722`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c928ca2d929d1fd63863e186"></a>
## variable_length_bytes

`function` · `parquet::data_type::private::ParquetValueType::variable_length_bytes` · parquet 59.3.0

```rust
fn variable_length_bytes(_: &[Self]) -> Option<i64>
```

Source: `src/data_type.rs:732`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Return the number of variable length bytes in a given slice of data

Returns the sum of lengths for BYTE_ARRAY data, and None for all other data types
