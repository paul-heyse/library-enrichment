# `arrow_flight::gen::flight_service_server::FlightService`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.gen.flight_service_server.FlightService.json).

<a id="op-7ca457340a3720a3eb5a2a31"></a>
## FlightService

`trait` · `arrow_flight::gen::flight_service_server::FlightService` · arrow-flight 59.3.0

```rust
trait FlightService: std::marker::Send + std::marker::Sync + 'static
```

Source: `src/arrow.flight.protocol.rs:850`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Generated trait containing gRPC methods that should be implemented for use with FlightServiceServer.

<a id="op-adfcf07cde456676bb034942"></a>
## DoActionStream

`assoc_type` · `arrow_flight::gen::flight_service_server::FlightService::DoActionStream` · arrow-flight 59.3.0

```rust
DoActionStream
```

Source: `src/arrow.flight.protocol.rs:986`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Server streaming response type for the DoAction method.

<a id="op-1f7bdf743728c6a25856d9b6"></a>
## DoExchangeStream

`assoc_type` · `arrow_flight::gen::flight_service_server::FlightService::DoExchangeStream` · arrow-flight 59.3.0

```rust
DoExchangeStream
```

Source: `src/arrow.flight.protocol.rs:970`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Server streaming response type for the DoExchange method.

<a id="op-b00d1b2666280baa77498af9"></a>
## DoGetStream

`assoc_type` · `arrow_flight::gen::flight_service_server::FlightService::DoGetStream` · arrow-flight 59.3.0

```rust
DoGetStream
```

Source: `src/arrow.flight.protocol.rs:938`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Server streaming response type for the DoGet method.

<a id="op-d08bc48dd62bea63f2b2decb"></a>
## DoPutStream

`assoc_type` · `arrow_flight::gen::flight_service_server::FlightService::DoPutStream` · arrow-flight 59.3.0

```rust
DoPutStream
```

Source: `src/arrow.flight.protocol.rs:953`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Server streaming response type for the DoPut method.

<a id="op-3c56f86a9fa47d64cf1fa928"></a>
## HandshakeStream

`assoc_type` · `arrow_flight::gen::flight_service_server::FlightService::HandshakeStream` · arrow-flight 59.3.0

```rust
HandshakeStream
```

Source: `src/arrow.flight.protocol.rs:852`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Server streaming response type for the Handshake method.

<a id="op-e5c6e5b5a9f21e2a559e8c58"></a>
## ListActionsStream

`assoc_type` · `arrow_flight::gen::flight_service_server::FlightService::ListActionsStream` · arrow-flight 59.3.0

```rust
ListActionsStream
```

Source: `src/arrow.flight.protocol.rs:1003`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Server streaming response type for the ListActions method.

<a id="op-2def430fe300f8d8eb868c07"></a>
## ListFlightsStream

`assoc_type` · `arrow_flight::gen::flight_service_server::FlightService::ListFlightsStream` · arrow-flight 59.3.0

```rust
ListFlightsStream
```

Source: `src/arrow.flight.protocol.rs:867`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Server streaming response type for the ListFlights method.

<a id="op-f00bfdb492adebfbc6c30fa3"></a>
## do_action

`function` · `arrow_flight::gen::flight_service_server::FlightService::do_action` · arrow-flight 59.3.0

```rust
async fn do_action(&self, request: tonic::Request<super::Action>) -> std::result::Result<tonic::Response<Self::DoActionStream>, tonic::Status>
```

Source: `src/arrow.flight.protocol.rs:998`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Flight services can support an arbitrary number of simple actions in
addition to the possible ListFlights, GetFlightInfo, DoGet, DoPut
operations that are potentially available. DoAction allows a flight client
to do a specific action against a flight service. An action includes
opaque request and response objects that are specific to the type action
being undertaken.

<a id="op-0c2f30ce7d94d926839a8c1d"></a>
## do_exchange

`function` · `arrow_flight::gen::flight_service_server::FlightService::do_exchange` · arrow-flight 59.3.0

```rust
async fn do_exchange(&self, request: tonic::Request<tonic::Streaming<super::FlightData>>) -> std::result::Result<tonic::Response<Self::DoExchangeStream>, tonic::Status>
```

Source: `src/arrow.flight.protocol.rs:981`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Open a bidirectional data channel for a given descriptor. This
allows clients to send and receive arbitrary Arrow data and
application-specific metadata in a single logical stream. In
contrast to DoGet/DoPut, this is more suited for clients
offloading computation (rather than storage) to a Flight service.

<a id="op-d786997033d959acce74b29e"></a>
## do_get

`function` · `arrow_flight::gen::flight_service_server::FlightService::do_get` · arrow-flight 59.3.0

```rust
async fn do_get(&self, request: tonic::Request<super::Ticket>) -> std::result::Result<tonic::Response<Self::DoGetStream>, tonic::Status>
```

Source: `src/arrow.flight.protocol.rs:948`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieve a single stream associated with a particular descriptor
associated with the referenced ticket. A Flight can be composed of one or
more streams where each stream can be retrieved using a separate opaque
ticket that the flight service uses for managing a collection of streams.

<a id="op-2a1ee242282c582ba4a1cfb3"></a>
## do_put

`function` · `arrow_flight::gen::flight_service_server::FlightService::do_put` · arrow-flight 59.3.0

```rust
async fn do_put(&self, request: tonic::Request<tonic::Streaming<super::FlightData>>) -> std::result::Result<tonic::Response<Self::DoPutStream>, tonic::Status>
```

Source: `src/arrow.flight.protocol.rs:965`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Push a stream to the flight service associated with a particular
flight stream. This allows a client of a flight service to upload a stream
of data. Depending on the particular flight service, a client consumer
could be allowed to upload a single stream per descriptor or an unlimited
number. In the latter, the service might implement a 'seal' action that
can be applied to a descriptor once all streams are uploaded.

<a id="op-9b1519fb9305192026115b8d"></a>
## get_flight_info

`function` · `arrow_flight::gen::flight_service_server::FlightService::get_flight_info` · arrow-flight 59.3.0

```rust
async fn get_flight_info(&self, request: tonic::Request<super::FlightDescriptor>) -> std::result::Result<tonic::Response<super::FlightInfo>, tonic::Status>
```

Source: `src/arrow.flight.protocol.rs:897`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


For a given FlightDescriptor, get information about how the flight can be
consumed. This is a useful interface if the consumer of the interface
already can identify the specific flight to consume. This interface can
also allow a consumer to generate a flight stream through a specified
descriptor. For example, a flight descriptor might be something that
includes a SQL statement or a Pickled Python operation that will be
executed. In those cases, the descriptor will not be previously available
within the list of available streams provided by ListFlights but will be
available for consumption for the duration defined by the specific flight
service.

<a id="op-31489d0c1a0e7723fe31b5d6"></a>
## get_schema

`function` · `arrow_flight::gen::flight_service_server::FlightService::get_schema` · arrow-flight 59.3.0

```rust
async fn get_schema(&self, request: tonic::Request<super::FlightDescriptor>) -> std::result::Result<tonic::Response<super::SchemaResult>, tonic::Status>
```

Source: `src/arrow.flight.protocol.rs:933`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


For a given FlightDescriptor, get the Schema as described in Schema.fbs::Schema
This is used when a consumer needs the Schema of flight stream. Similar to
GetFlightInfo this interface may generate a new flight that was not previously
available in ListFlights.

<a id="op-54ddab2c1816e4e26037bb6f"></a>
## handshake

`function` · `arrow_flight::gen::flight_service_server::FlightService::handshake` · arrow-flight 59.3.0

```rust
async fn handshake(&self, request: tonic::Request<tonic::Streaming<super::HandshakeRequest>>) -> std::result::Result<tonic::Response<Self::HandshakeStream>, tonic::Status>
```

Source: `src/arrow.flight.protocol.rs:862`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Handshake between client and server. Depending on the server, the
handshake may be required to determine the token that should be used for
future operations. Both request and response are streams to allow multiple
round-trips depending on auth mechanism.

<a id="op-56cd2853636f195003f0e695"></a>
## list_actions

`function` · `arrow_flight::gen::flight_service_server::FlightService::list_actions` · arrow-flight 59.3.0

```rust
async fn list_actions(&self, request: tonic::Request<super::Empty>) -> std::result::Result<tonic::Response<Self::ListActionsStream>, tonic::Status>
```

Source: `src/arrow.flight.protocol.rs:1012`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


A flight service exposes all of the available action types that it has
along with descriptions. This allows different flight consumers to
understand the capabilities of the flight service.

<a id="op-3fb52436a010e8857f62af87"></a>
## list_flights

`function` · `arrow_flight::gen::flight_service_server::FlightService::list_flights` · arrow-flight 59.3.0

```rust
async fn list_flights(&self, request: tonic::Request<super::Criteria>) -> std::result::Result<tonic::Response<Self::ListFlightsStream>, tonic::Status>
```

Source: `src/arrow.flight.protocol.rs:879`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Get a list of available streams given a particular criteria. Most flight
services will expose one or more streams that are readily available for
retrieval. This api allows listing the streams available for
consumption. A user can also provide a criteria. The criteria can limit
the subset of streams that can be listed via this interface. Each flight
service allows its own definition of how to consume criteria.

<a id="op-9e33494e270ffebed5a3d741"></a>
## poll_flight_info

`function` · `arrow_flight::gen::flight_service_server::FlightService::poll_flight_info` · arrow-flight 59.3.0

```rust
async fn poll_flight_info(&self, request: tonic::Request<super::FlightDescriptor>) -> std::result::Result<tonic::Response<super::PollInfo>, tonic::Status>
```

Source: `src/arrow.flight.protocol.rs:924`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


For a given FlightDescriptor, start a query and get information
to poll its execution status. This is a useful interface if the
query may be a long-running query. The first PollFlightInfo call
should return as quickly as possible. (GetFlightInfo doesn't
return until the query is complete.)

A client can consume any available results before
the query is completed. See PollInfo.info for details.

A client can poll the updated query status by calling
PollFlightInfo() with PollInfo.flight_descriptor. A server
should not respond until the result would be different from last
time. That way, the client can "long poll" for updates
without constantly making requests. Clients can set a short timeout
to avoid blocking calls if desired.

A client can't use PollInfo.flight_descriptor after
PollInfo.expiration_time passes. A server might not accept the
retry descriptor anymore and the query may be cancelled.

A client may use the CancelFlightInfo action with
PollInfo.info to cancel the running query.
