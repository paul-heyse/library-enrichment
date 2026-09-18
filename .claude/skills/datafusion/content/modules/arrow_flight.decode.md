# `arrow_flight::decode`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.decode.json).

<a id="op-e9c919514391f6af762b120d"></a>
## decode

`module` · `arrow_flight::decode` · arrow-flight 59.3.0

```rust
mod decode
```

Source: `src/decode.rs:18`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Decoder to create [`RecordBatch`](arrow_array::RecordBatch) streams from [`FlightData`](../operations/arrow_flight.gen.FlightData.md#op-d385f038797edc1454fc35a5) streams.
See [`FlightRecordBatchStream`](decode::FlightRecordBatchStream).
