# `arrow_flight::gen::flight_service_client`

Crate `arrow-flight` · 1 public items · structured records in [`model/arrow_flight.gen.flight_service_client.json`](../model/arrow_flight.gen.flight_service_client.json)

## FlightServiceClient

`struct` · `arrow_flight::gen::flight_service_client::FlightServiceClient`

Also reachable as `arrow_flight::flight_service_client::FlightServiceClient`

```rust
struct FlightServiceClient<T>
```

**Derives**: Clone, Debug

**Methods** (17)

```rust
fn accept_compressed(self, encoding: CompressionEncoding) -> Self
async fn do_action(&mut self, request: impl tonic::IntoRequest<super::Action>) -> std::result::Result<tonic::Response<tonic::codec::Streaming<super::Result>>, tonic::Status>
async fn do_exchange(&mut self, request: impl tonic::IntoStreamingRequest<Message = super::FlightData>) -> std::result::Result<tonic::Response<tonic::codec::Streaming<super::FlightData>>, tonic::Status>
async fn do_get(&mut self, request: impl tonic::IntoRequest<super::Ticket>) -> std::result::Result<tonic::Response<tonic::codec::Streaming<super::FlightData>>, tonic::Status>
async fn do_put(&mut self, request: impl tonic::IntoStreamingRequest<Message = super::FlightData>) -> std::result::Result<tonic::Response<tonic::codec::Streaming<super::PutResult>>, tonic::Status>
async fn get_flight_info(&mut self, request: impl tonic::IntoRequest<super::FlightDescriptor>) -> std::result::Result<tonic::Response<super::FlightInfo>, tonic::Status>
async fn get_schema(&mut self, request: impl tonic::IntoRequest<super::FlightDescriptor>) -> std::result::Result<tonic::Response<super::SchemaResult>, tonic::Status>
async fn handshake(&mut self, request: impl tonic::IntoStreamingRequest<Message = super::HandshakeRequest>) -> std::result::Result<tonic::Response<tonic::codec::Streaming<super::HandshakeResponse>>, tonic::Status>
async fn list_actions(&mut self, request: impl tonic::IntoRequest<super::Empty>) -> std::result::Result<tonic::Response<tonic::codec::Streaming<super::ActionType>>, tonic::Status>
async fn list_flights(&mut self, request: impl tonic::IntoRequest<super::Criteria>) -> std::result::Result<tonic::Response<tonic::codec::Streaming<super::FlightInfo>>, tonic::Status>
fn max_decoding_message_size(self, limit: usize) -> Self
fn max_encoding_message_size(self, limit: usize) -> Self
fn new(inner: T) -> Self
async fn poll_flight_info(&mut self, request: impl tonic::IntoRequest<super::FlightDescriptor>) -> std::result::Result<tonic::Response<super::PollInfo>, tonic::Status>
fn send_compressed(self, encoding: CompressionEncoding) -> Self
fn with_interceptor<F>(inner: T, interceptor: F) -> FlightServiceClient<InterceptedService<T, F>> where F: tonic::service::Interceptor, T::ResponseBody: Default, T: tonic::codegen::Service<http::Request<tonic::body::Body>, Response = http::Response<<T as tonic::client::GrpcService>::ResponseBody>>, <T as tonic::codegen::Service>::Error: Into<StdError> + std::marker::Send + std::marker::Sync
fn with_origin(inner: T, origin: Uri) -> Self
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.gen.flight_service_client.FlightServiceClient.md).



A flight service is an endpoint for retrieving or storing Arrow data. A
flight service can expose one or more predefined endpoints that can be
accessed using the Arrow Flight Protocol. Additionally, a flight service
can expose a set of actions that are available.

---
