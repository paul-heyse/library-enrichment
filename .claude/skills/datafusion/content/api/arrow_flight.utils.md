# `arrow_flight::utils`

Crate `arrow-flight` · 3 public items · structured records in [`model/arrow_flight.utils.json`](../model/arrow_flight.utils.json)

## batches_to_flight_data

`function` · `arrow_flight::utils::batches_to_flight_data`

```rust
fn batches_to_flight_data(schema: &arrow_schema::Schema, batches: Vec<arrow_array::RecordBatch>) -> Result<Vec<FlightData>, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.utils.batches_to_flight_data.md).


Convert `RecordBatch`es to wire protocol `FlightData`s

---

## flight_data_to_arrow_batch

`function` · `arrow_flight::utils::flight_data_to_arrow_batch`

```rust
fn flight_data_to_arrow_batch(data: &FlightData, schema: arrow_schema::SchemaRef, dictionaries_by_id: &std::collections::HashMap<i64, arrow_array::ArrayRef>) -> Result<arrow_array::RecordBatch, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.utils.flight_data_to_arrow_batch.md).


Convert `FlightData` (with supplied schema and dictionaries) to an arrow `RecordBatch`.

---

## flight_data_to_batches

`function` · `arrow_flight::utils::flight_data_to_batches`

```rust
fn flight_data_to_batches(flight_data: &[FlightData]) -> Result<Vec<arrow_array::RecordBatch>, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.utils.flight_data_to_batches.md).


Convert a slice of wire protocol `FlightData`s into a vector of `RecordBatch`es

---
