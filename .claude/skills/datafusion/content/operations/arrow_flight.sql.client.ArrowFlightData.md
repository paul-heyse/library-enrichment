# `arrow_flight::sql::client::ArrowFlightData`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.client.ArrowFlightData.json).

<a id="op-56a18aad66ad77e284a823a1"></a>
## ArrowFlightData

`enum` · `arrow_flight::sql::client::ArrowFlightData` · arrow-flight 59.3.0

```rust
enum ArrowFlightData
```

Source: `src/sql/client.rs:623`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

A polymorphic structure to natively represent different types of data contained in `FlightData`

<a id="op-9d293ddce2bd163b45ec146e"></a>
## RecordBatch

`variant` · `arrow_flight::sql::client::ArrowFlightData::RecordBatch` · arrow-flight 59.3.0

```rust
RecordBatch
```

Source: `src/sql/client.rs:625`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

A record batch

<a id="op-af6a73136cb0f907d34c87e6"></a>
## Schema

`variant` · `arrow_flight::sql::client::ArrowFlightData::Schema` · arrow-flight 59.3.0

```rust
Schema
```

Source: `src/sql/client.rs:627`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

A schema
