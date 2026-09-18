# `arrow_flight::utils::flight_data_to_arrow_batch`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.utils.flight_data_to_arrow_batch.json).

<a id="op-b4a194100384be129746a3d7"></a>
## flight_data_to_arrow_batch

`function` · `arrow_flight::utils::flight_data_to_arrow_batch` · arrow-flight 59.3.0

```rust
fn flight_data_to_arrow_batch(data: &FlightData, schema: arrow_schema::SchemaRef, dictionaries_by_id: &std::collections::HashMap<i64, arrow_array::ArrayRef>) -> Result<arrow_array::RecordBatch, arrow_schema::ArrowError>
```

Source: `src/utils.rs:55`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Convert `FlightData` (with supplied schema and dictionaries) to an arrow `RecordBatch`.
