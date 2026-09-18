# `arrow_array::nonblocking::RecordBatchStream`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.nonblocking.RecordBatchStream.json).

<a id="op-0a453edcb63c1548b52d9a86"></a>
## RecordBatchStream

`trait` · `arrow_array::nonblocking::RecordBatchStream` · arrow-array 59.3.0

```rust
trait RecordBatchStream: futures::Stream<Item = Result<record_batch::RecordBatch, arrow_schema::ArrowError>>
```

Source: `src/nonblocking.rs:25`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Trait for an asynchronous stream of `RecordBatch`es.

<a id="op-a31d7a1f4726204a22c8d268"></a>
## schema

`function` · `arrow_array::nonblocking::RecordBatchStream::schema` · arrow-array 59.3.0

```rust
fn schema(&self) -> &SchemaRef
```

Source: `src/nonblocking.rs:30`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Return the schema of this `RecordBatchStream`.

Implementation of this trait should guarantee that all `RecordBatch`'s returned by this
reader should have the same schema as returned from this method.
