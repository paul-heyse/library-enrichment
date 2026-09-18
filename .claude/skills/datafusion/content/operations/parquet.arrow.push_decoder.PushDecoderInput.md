# `parquet::arrow::push_decoder::PushDecoderInput`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.push_decoder.PushDecoderInput.json).

<a id="op-2c0db2bbddf0b3767f58ab1d"></a>
## PushDecoderInput

`struct` · `parquet::arrow::push_decoder::PushDecoderInput` · parquet 59.3.0

```rust
struct PushDecoderInput
```

Source: `src/arrow/push_decoder/mod.rs:202`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The `input` of a [`ParquetPushDecoderBuilder`](../operations/parquet.arrow.push_decoder.ParquetPushDecoderBuilder.md#op-be5e1b449f8e3584201abd08).

The shared [`ArrowReaderBuilder`](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md#op-2cb4803ec228a018e60491ee) is generic over an `input`. The sync and
async builders read from a file or async reader; the push decoder has no
reader, so its input is the [`PushBuffers`] that caller-pushed bytes
accumulate in (empty for a fresh builder).

Unresolved upstream links (retained, not inferred): ``PushBuffers``.

<a id="op-fa60aa1f06b5c900c89c7e72"></a>
## default

`function` · `parquet::arrow::push_decoder::PushDecoderInput::default` · parquet 59.3.0

```rust
fn default() -> PushDecoderInput
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::push_decoder::PushDecoderInput", "path": "PushDecoderInput"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 17], "end": [201, 24], "filename": "src/arrow/push_decoder/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/arrow/push_decoder/mod.rs:201`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11fd883a79142798dec85e3c"></a>
## fmt

`function` · `parquet::arrow::push_decoder::PushDecoderInput::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::push_decoder::PushDecoderInput", "path": "PushDecoderInput"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 10], "end": [201, 15], "filename": "src/arrow/push_decoder/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow/push_decoder/mod.rs:201`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
