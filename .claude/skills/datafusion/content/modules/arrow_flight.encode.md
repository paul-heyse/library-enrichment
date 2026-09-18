# `arrow_flight::encode`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.encode.json).

<a id="op-e5f297753ca49d1fadcd9a81"></a>
## encode

`module` · `arrow_flight::encode` · arrow-flight 59.3.0

```rust
mod encode
```

Source: `src/encode.rs:18`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Encoder to create [`FlightData`](../operations/arrow_flight.gen.FlightData.md#op-d385f038797edc1454fc35a5) streams from [`RecordBatch`](arrow_array::RecordBatch) streams.
See [`FlightDataEncoderBuilder`](encode::FlightDataEncoderBuilder).
