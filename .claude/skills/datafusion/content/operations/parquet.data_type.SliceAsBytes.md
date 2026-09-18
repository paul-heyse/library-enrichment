# `parquet::data_type::SliceAsBytes`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.data_type.SliceAsBytes.json).

<a id="op-c58701ced8cbc45094c967d6"></a>
## SliceAsBytes

`trait` · `parquet::data_type::SliceAsBytes` · parquet 59.3.0

```rust
trait SliceAsBytes: Sized
```

Source: `src/data_type.rs:531`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Converts an slice of a data type to a slice of bytes.

<a id="op-500279b35e6e739e5323aaaf"></a>
## slice_as_bytes

`function` · `parquet::data_type::SliceAsBytes::slice_as_bytes` · parquet 59.3.0

```rust
fn slice_as_bytes(self_: &[Self]) -> &[u8]
```

Source: `src/data_type.rs:533`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns slice of bytes for a slice of this data type.

<a id="op-5ea0236c7349e28b6f24134b"></a>
## slice_as_bytes_mut

`function` · `parquet::data_type::SliceAsBytes::slice_as_bytes_mut` · parquet 59.3.0

```rust
unsafe fn slice_as_bytes_mut(self_: &mut [Self]) -> &mut [u8]
```

Source: `src/data_type.rs:539`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Return the internal representation as a mutable slice

# Safety
If modified you are _required_ to ensure the internal representation
is valid and correct for the actual raw data
