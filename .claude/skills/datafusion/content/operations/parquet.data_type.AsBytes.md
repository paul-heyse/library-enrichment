# `parquet::data_type::AsBytes`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.data_type.AsBytes.json).

<a id="op-df19664d53a774fa5f1d6462"></a>
## AsBytes

`trait` · `parquet::data_type::AsBytes` · parquet 59.3.0

```rust
trait AsBytes
```

Source: `src/data_type.rs:525`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Converts an instance of data type to a slice of bytes as `u8`.

<a id="op-6b40a8c5fefa6a63fc4de0bb"></a>
## as_bytes

`function` · `parquet::data_type::AsBytes::as_bytes` · parquet 59.3.0

```rust
fn as_bytes(&self) -> &[u8]
```

Source: `src/data_type.rs:527`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns slice of bytes for this data type.
