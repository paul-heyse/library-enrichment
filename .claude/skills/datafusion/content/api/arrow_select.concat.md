# `arrow_select::concat`

Crate `arrow-select` · 2 public items · structured records in [`model/arrow_select.concat.json`](../model/arrow_select.concat.json)

## concat

`function` · `arrow_select::concat::concat`

Also reachable as `arrow::compute::concat`, `arrow::compute::kernels::concat::concat`

```rust
fn concat(arrays: &[&dyn Array]) -> Result<ArrayRef, arrow_schema::ArrowError>
```

Concatenate multiple [Array] of the same type into a single [ArrayRef].

---

## concat_batches

`function` · `arrow_select::concat::concat_batches`

Also reachable as `arrow::compute::concat_batches`, `arrow::compute::kernels::concat::concat_batches`

```rust
fn concat_batches<'a>(schema: &arrow_schema::SchemaRef, input_batches: impl IntoIterator<Item = &'a RecordBatch>) -> Result<RecordBatch, arrow_schema::ArrowError>
```

Concatenates `batches` together into a single [`RecordBatch`].

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

---
