# `arrow_flight::sql`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.json).

<a id="op-5c7e82b3d5fd3c5510638ec5"></a>
## sql

`module` · `arrow_flight::sql` · arrow-flight 59.3.0

```rust
mod sql
```

Source: `src/sql/mod.rs:18`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Support for execute SQL queries using [Apache Arrow] [Flight SQL].

[Flight SQL] is built on top of Arrow Flight RPC framework, by
defining specific messages, encoded using the protobuf format,
sent in the[`FlightDescriptor::cmd`] field to [`FlightService`]
endpoints such as[`get_flight_info`] and [`do_get`].

This module contains:
1. [prost] generated structs for FlightSQL messages such as [`CommandStatementQuery`](../operations/arrow_flight.sql.gen.CommandStatementQuery.md#op-27d3fede133f129d418db1cf)
2. Helpers for encoding and decoding FlightSQL messages: [`Any`](../operations/arrow_flight.sql.Any.md#op-a83fa9be583ec3b019e51a4e) and [`Command`](../operations/arrow_flight.sql.Command.md#op-cf7f20617e0f5971ea16d973)
3. A [`FlightSqlServiceClient`] for interacting with FlightSQL servers.
4. A [`FlightSqlService`] to help building FlightSQL servers from [`FlightService`].
5. Helpers to build responses for FlightSQL metadata APIs: [`metadata`]

[Flight SQL]: https://arrow.apache.org/docs/format/FlightSql.html
[Apache Arrow]: https://arrow.apache.org
[`FlightDescriptor::cmd`]: crate::FlightDescriptor::cmd
[`FlightService`]: crate::flight_service_server::FlightService
[`get_flight_info`]: crate::flight_service_server::FlightService::get_flight_info
[`do_get`]: crate::flight_service_server::FlightService::do_get
[`FlightSqlServiceClient`]: client::FlightSqlServiceClient
[`FlightSqlService`]: server::FlightSqlService
[`metadata`]: crate::sql::metadata

Unresolved upstream links (retained, not inferred): `prost`.
