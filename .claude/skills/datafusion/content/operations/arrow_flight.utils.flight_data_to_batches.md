# `arrow_flight::utils::flight_data_to_batches`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.utils.flight_data_to_batches.json).

<a id="op-44f9c985d22368019befb6ec"></a>
## flight_data_to_batches

`function` · `arrow_flight::utils::flight_data_to_batches` · arrow-flight 59.3.0

```rust
fn flight_data_to_batches(flight_data: &[FlightData]) -> Result<Vec<arrow_array::RecordBatch>, arrow_schema::ArrowError>
```

Source: `src/utils.rs:32`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Convert a slice of wire protocol `FlightData`s into a vector of `RecordBatch`es
