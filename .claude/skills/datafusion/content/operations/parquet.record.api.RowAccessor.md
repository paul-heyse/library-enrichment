# `parquet::record::api::RowAccessor`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.record.api.RowAccessor.json).

<a id="op-e8c53e51c2a49cd3af0ebd0d"></a>
## RowAccessor

`trait` · `parquet::record::api::RowAccessor` · parquet 59.3.0

```rust
trait RowAccessor
```

Source: `src/record/api.rs:142`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Trait for type-safe convenient access to fields within a Row.

<a id="op-e6420a26d3c9692d62cc4e4c"></a>
## get_bool

`function` · `parquet::record::api::RowAccessor::get_bool` · parquet 59.3.0

```rust
fn get_bool(&self, i: usize) -> Result<bool>
```

Source: `src/record/api.rs:146`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try to get a boolean value at the given index.

<a id="op-ac479fa03b8cb0ee4c4e7b53"></a>
## get_byte

`function` · `parquet::record::api::RowAccessor::get_byte` · parquet 59.3.0

```rust
fn get_byte(&self, i: usize) -> Result<i8>
```

Source: `src/record/api.rs:148`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try to get a byte value at the given index.

<a id="op-215c63bf3a214873853a53d1"></a>
## get_bytes

`function` · `parquet::record::api::RowAccessor::get_bytes` · parquet 59.3.0

```rust
fn get_bytes(&self, i: usize) -> Result<&ByteArray>
```

Source: `src/record/api.rs:178`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try to get a bytes value at the given index.

<a id="op-f9b8af7c8ad2db8b7d743ae1"></a>
## get_decimal

`function` · `parquet::record::api::RowAccessor::get_decimal` · parquet 59.3.0

```rust
fn get_decimal(&self, i: usize) -> Result<&Decimal>
```

Source: `src/record/api.rs:174`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try to get a decimal value at the given index.

<a id="op-bdbd2bef7c8a146a832045f6"></a>
## get_double

`function` · `parquet::record::api::RowAccessor::get_double` · parquet 59.3.0

```rust
fn get_double(&self, i: usize) -> Result<f64>
```

Source: `src/record/api.rs:168`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try to get a double value at the given index.

<a id="op-479418e39ec5b8c7a6defe84"></a>
## get_float

`function` · `parquet::record::api::RowAccessor::get_float` · parquet 59.3.0

```rust
fn get_float(&self, i: usize) -> Result<f32>
```

Source: `src/record/api.rs:166`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try to get a float value at the given index.

<a id="op-3da047ef589a39569996673d"></a>
## get_float16

`function` · `parquet::record::api::RowAccessor::get_float16` · parquet 59.3.0

```rust
fn get_float16(&self, i: usize) -> Result<f16>
```

Source: `src/record/api.rs:164`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try to get a float16 value at the given index.

<a id="op-04720dfbdc3818b3af5f69c9"></a>
## get_group

`function` · `parquet::record::api::RowAccessor::get_group` · parquet 59.3.0

```rust
fn get_group(&self, i: usize) -> Result<&Row>
```

Source: `src/record/api.rs:180`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try to get a group value at the given index.

<a id="op-c59794ed25929d5e66b3de0c"></a>
## get_int

`function` · `parquet::record::api::RowAccessor::get_int` · parquet 59.3.0

```rust
fn get_int(&self, i: usize) -> Result<i32>
```

Source: `src/record/api.rs:152`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try to get a int value at the given index.

<a id="op-5f9adb397203be27ea84406e"></a>
## get_list

`function` · `parquet::record::api::RowAccessor::get_list` · parquet 59.3.0

```rust
fn get_list(&self, i: usize) -> Result<&List>
```

Source: `src/record/api.rs:182`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try to get a list value at the given index.

<a id="op-4a9d6a0d1bf633bc98bbe5bc"></a>
## get_long

`function` · `parquet::record::api::RowAccessor::get_long` · parquet 59.3.0

```rust
fn get_long(&self, i: usize) -> Result<i64>
```

Source: `src/record/api.rs:154`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try to get a long value at the given index.

<a id="op-5785746a851658b9610af353"></a>
## get_map

`function` · `parquet::record::api::RowAccessor::get_map` · parquet 59.3.0

```rust
fn get_map(&self, i: usize) -> Result<&Map>
```

Source: `src/record/api.rs:184`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try to get a map value at the given index.

<a id="op-4c3d9cbade1ef383473afcf1"></a>
## get_short

`function` · `parquet::record::api::RowAccessor::get_short` · parquet 59.3.0

```rust
fn get_short(&self, i: usize) -> Result<i16>
```

Source: `src/record/api.rs:150`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try to get a short value at the given index.

<a id="op-c06e6d0d3b7c326ad5058d76"></a>
## get_string

`function` · `parquet::record::api::RowAccessor::get_string` · parquet 59.3.0

```rust
fn get_string(&self, i: usize) -> Result<&String>
```

Source: `src/record/api.rs:176`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try to get a string value at the given index.

<a id="op-b3520692ab06b6c68064a220"></a>
## get_timestamp_micros

`function` · `parquet::record::api::RowAccessor::get_timestamp_micros` · parquet 59.3.0

```rust
fn get_timestamp_micros(&self, i: usize) -> Result<i64>
```

Source: `src/record/api.rs:172`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try to get a date value at the given index.

<a id="op-ae18156782cc85ea3d02719c"></a>
## get_timestamp_millis

`function` · `parquet::record::api::RowAccessor::get_timestamp_millis` · parquet 59.3.0

```rust
fn get_timestamp_millis(&self, i: usize) -> Result<i64>
```

Source: `src/record/api.rs:170`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try to get a date value at the given index.

<a id="op-14bd0db1d54bbd0486d0208a"></a>
## get_ubyte

`function` · `parquet::record::api::RowAccessor::get_ubyte` · parquet 59.3.0

```rust
fn get_ubyte(&self, i: usize) -> Result<u8>
```

Source: `src/record/api.rs:156`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try to get a ubyte value at the given index.

<a id="op-030e3ed5ae4cf1352da65dab"></a>
## get_uint

`function` · `parquet::record::api::RowAccessor::get_uint` · parquet 59.3.0

```rust
fn get_uint(&self, i: usize) -> Result<u32>
```

Source: `src/record/api.rs:160`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try to get a uint value at the given index.

<a id="op-a3c45d0d52d94eb196f0598d"></a>
## get_ulong

`function` · `parquet::record::api::RowAccessor::get_ulong` · parquet 59.3.0

```rust
fn get_ulong(&self, i: usize) -> Result<u64>
```

Source: `src/record/api.rs:162`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try to get a ulong value at the given index.

<a id="op-394ea12cad72f503b10711f3"></a>
## get_ushort

`function` · `parquet::record::api::RowAccessor::get_ushort` · parquet 59.3.0

```rust
fn get_ushort(&self, i: usize) -> Result<u16>
```

Source: `src/record/api.rs:158`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try to get a ushort value at the given index.

<a id="op-3616a52cfb3a6d2606278a65"></a>
## is_null

`function` · `parquet::record::api::RowAccessor::is_null` · parquet 59.3.0

```rust
fn is_null(&self, i: usize) -> Result<bool>
```

Source: `src/record/api.rs:144`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Check if the field at the index is null.
