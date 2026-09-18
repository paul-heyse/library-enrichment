# `arrow_flight::sql::client::arrow_data_from_flight_data`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.client.arrow_data_from_flight_data.json).

<a id="op-503420f08787d55f318139be"></a>
## arrow_data_from_flight_data

`function` · `arrow_flight::sql::client::arrow_data_from_flight_data` · arrow-flight 59.3.0

```rust
fn arrow_data_from_flight_data(flight_data: FlightData, arrow_schema_ref: &arrow_schema::SchemaRef) -> std::result::Result<ArrowFlightData, arrow_schema::ArrowError>
```

Source: `src/sql/client.rs:631`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Extract `Schema` or `RecordBatch`es from the `FlightData` wire representation
