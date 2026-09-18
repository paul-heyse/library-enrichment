# `parquet::file::reader::Length`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.reader.Length.json).

<a id="op-13bbbd397dab597770f14425"></a>
## Length

`trait` · `parquet::file::reader::Length` · parquet 59.3.0

```rust
trait Length
```

Source: `src/file/reader.rs:43`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Length should return the total number of bytes in the input source.
It's mainly used to read the metadata, which is at the end of the source.

<a id="op-da984d115bd2d7468abe8eca"></a>
## len

`function` · `parquet::file::reader::Length::len` · parquet 59.3.0

```rust
fn len(&self) -> u64
```

Source: `src/file/reader.rs:45`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the amount of bytes of the inner source.
