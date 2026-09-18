# `arrow_array::nonblocking`

Crate `arrow-array` · 1 public items · structured records in [`model/arrow_array.nonblocking.json`](../model/arrow_array.nonblocking.json)

## RecordBatchStream

`trait` · `arrow_array::nonblocking::RecordBatchStream`

```rust
trait RecordBatchStream: futures::Stream<Item = Result<record_batch::RecordBatch, arrow_schema::ArrowError>>
```

**Methods** (1)

```rust
fn schema(&self) -> &SchemaRef
```

[Full member, field, variant and typed contracts](../operations/arrow_array.nonblocking.RecordBatchStream.md).


Trait for an asynchronous stream of `RecordBatch`es.

---
