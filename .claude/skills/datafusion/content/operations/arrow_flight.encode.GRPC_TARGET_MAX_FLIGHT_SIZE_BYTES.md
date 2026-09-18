# `arrow_flight::encode::GRPC_TARGET_MAX_FLIGHT_SIZE_BYTES`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.encode.GRPC_TARGET_MAX_FLIGHT_SIZE_BYTES.json).

<a id="op-823e19844becc2ab4433a3c1"></a>
## GRPC_TARGET_MAX_FLIGHT_SIZE_BYTES

`constant` · `arrow_flight::encode::GRPC_TARGET_MAX_FLIGHT_SIZE_BYTES` · arrow-flight 59.3.0

```rust
const GRPC_TARGET_MAX_FLIGHT_SIZE_BYTES: usize = 2097152
```

Source: `src/encode.rs:166`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Default target size for encoded [`FlightData`](../operations/arrow_flight.gen.FlightData.md#op-d385f038797edc1454fc35a5).

Note this value would normally be 4MB, but the size calculation is
somewhat inexact, so we set it to 2MB.
