# `arrow::util::data_gen::create_random_batch`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.data_gen.create_random_batch.json).

<a id="op-e0cb7183fe2c0c36bdc612c3"></a>
## create_random_batch

`function` · `arrow::util::data_gen::create_random_batch` · arrow 59.3.0

```rust
fn create_random_batch(schema: SchemaRef, size: usize, null_density: f32, true_density: f32) -> error::Result<RecordBatch>
```

Source: `src/util/data_gen.rs:37`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Create a random [RecordBatch](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) from a schema
