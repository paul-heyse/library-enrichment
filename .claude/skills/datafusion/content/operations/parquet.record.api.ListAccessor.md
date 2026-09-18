# `parquet::record::api::ListAccessor`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.record.api.ListAccessor.json).

<a id="op-924c472f4c8da504651e9209"></a>
## ListAccessor

`trait` · `parquet::record::api::ListAccessor` · parquet 59.3.0

```rust
trait ListAccessor
```

Source: `src/record/api.rs:350`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Trait for type-safe access of an index for a `List`.
Note that the get_XXX methods do not do bound checking.

<a id="op-81789a22485e5216b117fe84"></a>
## get_bool

`function` · `parquet::record::api::ListAccessor::get_bool` · parquet 59.3.0

```rust
fn get_bool(&self, i: usize) -> Result<bool>
```

Source: `src/record/api.rs:352`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try getting a `boolean` value at the given index.

<a id="op-0622d38c3ee97b86a875293a"></a>
## get_byte

`function` · `parquet::record::api::ListAccessor::get_byte` · parquet 59.3.0

```rust
fn get_byte(&self, i: usize) -> Result<i8>
```

Source: `src/record/api.rs:354`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try getting a `byte` value at the given index.

<a id="op-a1675ed04c46f92e9ca83416"></a>
## get_bytes

`function` · `parquet::record::api::ListAccessor::get_bytes` · parquet 59.3.0

```rust
fn get_bytes(&self, i: usize) -> Result<&ByteArray>
```

Source: `src/record/api.rs:386`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try getting a `bytes` value at the given index.

<a id="op-a68a3c9cca4eea447f4107de"></a>
## get_decimal

`function` · `parquet::record::api::ListAccessor::get_decimal` · parquet 59.3.0

```rust
fn get_decimal(&self, i: usize) -> Result<&Decimal>
```

Source: `src/record/api.rs:382`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try getting a `decimal` value at the given index.

<a id="op-87385718774ef11a93ba1d10"></a>
## get_double

`function` · `parquet::record::api::ListAccessor::get_double` · parquet 59.3.0

```rust
fn get_double(&self, i: usize) -> Result<f64>
```

Source: `src/record/api.rs:374`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try getting a `f64` value at the given index.

<a id="op-59fb3e03df496fe39b40bc50"></a>
## get_float

`function` · `parquet::record::api::ListAccessor::get_float` · parquet 59.3.0

```rust
fn get_float(&self, i: usize) -> Result<f32>
```

Source: `src/record/api.rs:372`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try getting a `f32` value at the given index.

<a id="op-6ded6493d811a4b28f8afb10"></a>
## get_float16

`function` · `parquet::record::api::ListAccessor::get_float16` · parquet 59.3.0

```rust
fn get_float16(&self, i: usize) -> Result<f16>
```

Source: `src/record/api.rs:370`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try getting a `f16` value at the given index.

<a id="op-16a0c4541bfac1511234eb4f"></a>
## get_group

`function` · `parquet::record::api::ListAccessor::get_group` · parquet 59.3.0

```rust
fn get_group(&self, i: usize) -> Result<&Row>
```

Source: `src/record/api.rs:388`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try getting a `group` value at the given index.

<a id="op-c083706113379468817ffecc"></a>
## get_int

`function` · `parquet::record::api::ListAccessor::get_int` · parquet 59.3.0

```rust
fn get_int(&self, i: usize) -> Result<i32>
```

Source: `src/record/api.rs:358`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try getting an `i32` value at the given index.

<a id="op-c5feb5664eddd4f06d1cc98f"></a>
## get_list

`function` · `parquet::record::api::ListAccessor::get_list` · parquet 59.3.0

```rust
fn get_list(&self, i: usize) -> Result<&List>
```

Source: `src/record/api.rs:390`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try getting a `list` value at the given index.

<a id="op-90026413b64680ab1e77bd99"></a>
## get_long

`function` · `parquet::record::api::ListAccessor::get_long` · parquet 59.3.0

```rust
fn get_long(&self, i: usize) -> Result<i64>
```

Source: `src/record/api.rs:360`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try getting an `i64` value at the given index.

<a id="op-f3238feb44f65f31a9f7c29c"></a>
## get_map

`function` · `parquet::record::api::ListAccessor::get_map` · parquet 59.3.0

```rust
fn get_map(&self, i: usize) -> Result<&Map>
```

Source: `src/record/api.rs:392`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try getting a `map` value at the given index.

<a id="op-288b725842a36f70fb81c582"></a>
## get_short

`function` · `parquet::record::api::ListAccessor::get_short` · parquet 59.3.0

```rust
fn get_short(&self, i: usize) -> Result<i16>
```

Source: `src/record/api.rs:356`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try getting an `i16` value at the given index.

<a id="op-4a0c25bda732e5dcec344bc3"></a>
## get_string

`function` · `parquet::record::api::ListAccessor::get_string` · parquet 59.3.0

```rust
fn get_string(&self, i: usize) -> Result<&String>
```

Source: `src/record/api.rs:384`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try getting a `string` value at the given index.

<a id="op-b7f735ba7f6ddf65417d81c0"></a>
## get_timestamp_micros

`function` · `parquet::record::api::ListAccessor::get_timestamp_micros` · parquet 59.3.0

```rust
fn get_timestamp_micros(&self, i: usize) -> Result<i64>
```

Source: `src/record/api.rs:380`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try getting a `timestamp` as microseconds value
encoded as `i64` at the given index.

<a id="op-ea0670d325b1d77ada3a719e"></a>
## get_timestamp_millis

`function` · `parquet::record::api::ListAccessor::get_timestamp_millis` · parquet 59.3.0

```rust
fn get_timestamp_millis(&self, i: usize) -> Result<i64>
```

Source: `src/record/api.rs:377`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try getting a `timestamp` as milliseconds value
encoded as `i64` at the given index.

<a id="op-a634263480f0d45463bda08d"></a>
## get_ubyte

`function` · `parquet::record::api::ListAccessor::get_ubyte` · parquet 59.3.0

```rust
fn get_ubyte(&self, i: usize) -> Result<u8>
```

Source: `src/record/api.rs:362`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try getting a `u8` value at the given index.

<a id="op-28c50950588c29968a004fac"></a>
## get_uint

`function` · `parquet::record::api::ListAccessor::get_uint` · parquet 59.3.0

```rust
fn get_uint(&self, i: usize) -> Result<u32>
```

Source: `src/record/api.rs:366`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try getting a `u32` value at the given index.

<a id="op-9e56f2c0a87c82198619fbb0"></a>
## get_ulong

`function` · `parquet::record::api::ListAccessor::get_ulong` · parquet 59.3.0

```rust
fn get_ulong(&self, i: usize) -> Result<u64>
```

Source: `src/record/api.rs:368`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try getting a `u64` value at the given index.

<a id="op-ced468756e06e4023bb06db5"></a>
## get_ushort

`function` · `parquet::record::api::ListAccessor::get_ushort` · parquet 59.3.0

```rust
fn get_ushort(&self, i: usize) -> Result<u16>
```

Source: `src/record/api.rs:364`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try getting a `u16` value at the given index.
