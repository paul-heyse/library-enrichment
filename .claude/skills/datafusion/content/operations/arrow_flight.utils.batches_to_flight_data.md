# `arrow_flight::utils::batches_to_flight_data`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.utils.batches_to_flight_data.json).

<a id="op-bb73cd1a0d448c9697fff5e6"></a>
## batches_to_flight_data

`function` · `arrow_flight::utils::batches_to_flight_data` · arrow-flight 59.3.0

```rust
fn batches_to_flight_data(schema: &arrow_schema::Schema, batches: Vec<arrow_array::RecordBatch>) -> Result<Vec<FlightData>, arrow_schema::ArrowError>
```

Source: `src/utils.rs:84`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Convert `RecordBatch`es to wire protocol `FlightData`s
