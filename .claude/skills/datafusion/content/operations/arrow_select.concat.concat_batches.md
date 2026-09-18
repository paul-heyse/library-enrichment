# `arrow_select::concat::concat_batches`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_select.concat.concat_batches.json).

<a id="op-7310cab72665168768fae19e"></a>
## concat_batches

`function` · `arrow_select::concat::concat_batches` · arrow-select 59.3.0

```rust
fn concat_batches<'a>(schema: &arrow_schema::SchemaRef, input_batches: impl IntoIterator<Item = &'a RecordBatch>) -> Result<RecordBatch, arrow_schema::ArrowError>
```

Source: `src/concat.rs:613`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Concatenates `batches` together into a single [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34).

The output batch has the specified `schemas`; The schema of the
input are ignored.

# Notes

- Callers should budget for peak memory use to approach 2x the input
  size, as the input batches and output arrays co-exist during construction.
- Arrays with `i32` offsets, such as `StringArray` and `BinaryArray`, only
  support up to ~2GiB of payloads. Concatenating large arrays of these types
  can cause offset overflows.

# Errors

Returns an error if the types of underlying arrays are different.
