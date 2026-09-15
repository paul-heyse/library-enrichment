# `arrow_flight::gen::flight_service_server`

Crate `arrow-flight` · 2 public items · structured records in [`model/arrow_flight.gen.flight_service_server.json`](../model/arrow_flight.gen.flight_service_server.json)

## FlightServiceServer

`struct` · `arrow_flight::gen::flight_service_server::FlightServiceServer`

Also reachable as `arrow_flight::flight_service_server::FlightServiceServer`

```rust
struct FlightServiceServer<T>
```

**Implements**: `tonic::server::NamedService`, `tower_service::Service`

**Derives**: Clone, Debug

**Methods** (7)

```rust
fn accept_compressed(self, encoding: CompressionEncoding) -> Self
fn from_arc(inner: Arc<T>) -> Self
fn max_decoding_message_size(self, limit: usize) -> Self
fn max_encoding_message_size(self, limit: usize) -> Self
fn new(inner: T) -> Self
fn send_compressed(self, encoding: CompressionEncoding) -> Self
fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F> where F: tonic::service::Interceptor
```

**via `tower_service::Service`**

```rust
fn call(&mut self, req: http::Request<B>) -> Self::Future
fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<std::result::Result<(), Self::Error>>
```


A flight service is an endpoint for retrieving or storing Arrow data. A
flight service can expose one or more predefined endpoints that can be
accessed using the Arrow Flight Protocol. Additionally, a flight service
can expose a set of actions that are available.

---

## FlightService

`trait` · `arrow_flight::gen::flight_service_server::FlightService`

Also reachable as `arrow_flight::flight_service_server::FlightService`

```rust
trait FlightService: std::marker::Send + std::marker::Sync + 'static
```

**Methods** (10)

```rust
async fn do_action(&self, request: tonic::Request<super::Action>) -> std::result::Result<tonic::Response<Self::DoActionStream>, tonic::Status>
async fn do_exchange(&self, request: tonic::Request<tonic::Streaming<super::FlightData>>) -> std::result::Result<tonic::Response<Self::DoExchangeStream>, tonic::Status>
async fn do_get(&self, request: tonic::Request<super::Ticket>) -> std::result::Result<tonic::Response<Self::DoGetStream>, tonic::Status>
async fn do_put(&self, request: tonic::Request<tonic::Streaming<super::FlightData>>) -> std::result::Result<tonic::Response<Self::DoPutStream>, tonic::Status>
async fn get_flight_info(&self, request: tonic::Request<super::FlightDescriptor>) -> std::result::Result<tonic::Response<super::FlightInfo>, tonic::Status>
async fn get_schema(&self, request: tonic::Request<super::FlightDescriptor>) -> std::result::Result<tonic::Response<super::SchemaResult>, tonic::Status>
async fn handshake(&self, request: tonic::Request<tonic::Streaming<super::HandshakeRequest>>) -> std::result::Result<tonic::Response<Self::HandshakeStream>, tonic::Status>
async fn list_actions(&self, request: tonic::Request<super::Empty>) -> std::result::Result<tonic::Response<Self::ListActionsStream>, tonic::Status>
async fn list_flights(&self, request: tonic::Request<super::Criteria>) -> std::result::Result<tonic::Response<Self::ListFlightsStream>, tonic::Status>
async fn poll_flight_info(&self, request: tonic::Request<super::FlightDescriptor>) -> std::result::Result<tonic::Response<super::PollInfo>, tonic::Status>
```

Generated trait containing gRPC methods that should be implemented for use with FlightServiceServer.

---
