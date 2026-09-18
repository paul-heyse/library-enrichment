# `parquet::arrow::arrow_reader::selection::cursor::MaskChunk`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.arrow_reader.selection.cursor.MaskChunk.json).

<a id="op-0b46facd85d87de95aa0fcab"></a>
## MaskChunk

`struct` · `parquet::arrow::arrow_reader::selection::cursor::MaskChunk` · parquet 59.3.0

```rust
struct MaskChunk
```

Source: `src/arrow/arrow_reader/selection/cursor.rs:336`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Result of computing the next chunk to read when using a [`MaskCursor`]

<a id="op-dcf114bd68314f772a88d1ad"></a>
## chunk_rows

`struct_field` · `parquet::arrow::arrow_reader::selection::cursor::MaskChunk::chunk_rows` · parquet 59.3.0

```rust
chunk_rows: usize
```

Source: `src/arrow/arrow_reader/selection/cursor.rs:340`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Total rows covered by this chunk (selected + skipped)

<a id="op-2f9f2df99bdff98388ddd362"></a>
## initial_skip

`struct_field` · `parquet::arrow::arrow_reader::selection::cursor::MaskChunk::initial_skip` · parquet 59.3.0

```rust
initial_skip: usize
```

Source: `src/arrow/arrow_reader/selection/cursor.rs:338`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Number of leading rows to skip before reaching selected rows

<a id="op-454fe6487666cca28c7041c3"></a>
## mask_start

`struct_field` · `parquet::arrow::arrow_reader::selection::cursor::MaskChunk::mask_start` · parquet 59.3.0

```rust
mask_start: usize
```

Source: `src/arrow/arrow_reader/selection/cursor.rs:344`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Starting offset within the mask where the chunk begins

<a id="op-a78e26b9c223e4ddd2600735"></a>
## selected_rows

`struct_field` · `parquet::arrow::arrow_reader::selection::cursor::MaskChunk::selected_rows` · parquet 59.3.0

```rust
selected_rows: usize
```

Source: `src/arrow/arrow_reader/selection/cursor.rs:342`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Rows actually selected within the chunk
