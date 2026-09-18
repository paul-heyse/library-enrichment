# `arrow_flight`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.json).

<a id="op-001a2f178a6b02b1d2f5e3f6"></a>
## arrow_flight

`module` · `arrow_flight` · arrow-flight 59.3.0

```rust
mod arrow_flight
```

Source: `src/lib.rs:18`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

A native Rust implementation of [Apache Arrow Flight](https://arrow.apache.org/docs/format/Flight.html)
for exchanging [Arrow](https://arrow.apache.org) data between processes.

Please see the [arrow-flight crates.io](https://crates.io/crates/arrow-flight)
page for feature flags and more information.

# Overview

This crate contains:

1. Low level [prost] generated structs
   for Flight gRPC protobuf messages, such as [`FlightData`](../operations/arrow_flight.gen.FlightData.md#op-d385f038797edc1454fc35a5), [`FlightInfo`](../operations/arrow_flight.gen.FlightInfo.md#op-422a7589713733d24ed1403b),
   [`Location`](../operations/arrow_flight.gen.Location.md#op-49255fbc894f83f915066c74) and [`Ticket`](../operations/arrow_flight.gen.Ticket.md#op-284d5d60124d450a179a6175).

2. Low level [tonic] generated [`flight_service_client`](../modules/arrow_flight.flight_service_client.md#op-7a3337c388dee1bbd8ddebff) and
   [`flight_service_server`](../modules/arrow_flight.flight_service_server.md#op-1bf3e017f87b7fa1f1bc16d6).

3. Support for [Flight SQL] in [`sql`](../modules/arrow_flight.sql.md#op-5c7e82b3d5fd3c5510638ec5). Requires the
   `flight-sql` feature of this crate to be activated.

[Flight SQL]: https://arrow.apache.org/docs/format/FlightSql.html

Unresolved upstream links (retained, not inferred): `tonic`, `prost`.
