# `arrow_flight::client`

Crate `arrow-flight` · 1 public items · structured records in [`model/arrow_flight.client.json`](../model/arrow_flight.client.json)

## FlightClient

`struct` · `arrow_flight::client::FlightClient`

Also reachable as `arrow_flight::FlightClient`

```rust
struct FlightClient<T = tonic::transport::Channel>
```

**Derives**: Debug

**Methods** (20)

```rust
fn add_header(&mut self, key: &str, value: &str) -> Result<()>
async fn cancel_flight_info(&mut self, request: CancelFlightInfoRequest) -> Result<CancelFlightInfoResult>
async fn do_action(&mut self, action: Action) -> Result<BoxStream<'static, Result<Bytes>>>
async fn do_exchange<S: Stream<Item = Result<FlightData>> + Send + 'static>(&mut self, request: S) -> Result<FlightRecordBatchStream>
async fn do_get(&mut self, ticket: Ticket) -> Result<FlightRecordBatchStream>
async fn do_put<S: Stream<Item = Result<FlightData>> + Send + 'static>(&mut self, request: S) -> Result<BoxStream<'static, Result<PutResult>>>
async fn get_flight_info(&mut self, descriptor: FlightDescriptor) -> Result<FlightInfo>
async fn get_schema(&mut self, flight_descriptor: FlightDescriptor) -> Result<Schema>
async fn handshake(&mut self, payload: impl Into<Bytes>) -> Result<Bytes>
fn inner(&self) -> &FlightServiceClient<T>
fn inner_mut(&mut self) -> &mut FlightServiceClient<T>
fn into_inner(self) -> FlightServiceClient<T>
async fn list_actions(&mut self) -> Result<BoxStream<'static, Result<ActionType>>>
async fn list_flights(&mut self, expression: impl Into<Bytes>) -> Result<BoxStream<'static, Result<FlightInfo>>>
fn metadata(&self) -> &MetadataMap
fn metadata_mut(&mut self) -> &mut MetadataMap
fn new(inner: T) -> Self
fn new_from_inner(inner: FlightServiceClient<T>) -> Self
async fn poll_flight_info(&mut self, descriptor: FlightDescriptor) -> Result<PollInfo>
async fn renew_flight_endpoint(&mut self, request: RenewFlightEndpointRequest) -> Result<FlightEndpoint>
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.client.FlightClient.md).


A "Mid level" [Apache Arrow Flight](https://arrow.apache.org/docs/format/Flight.html) client.

[`FlightClient`] is intended as a convenience for interactions
with Arrow Flight servers. For more direct control, such as access
to the response headers, use  [`FlightServiceClient`] directly
via methods such as [`Self::inner`] or [`Self::into_inner`].

# Example:
```no_run
# async fn run() {
# use arrow_flight::FlightClient;
# use bytes::Bytes;
use tonic::transport::Channel;
let channel = Channel::from_static("http://localhost:1234")
  .connect()
  .await
  .expect("error connecting");

let mut client = FlightClient::new(channel);

// Send 'Hi' bytes as the handshake request to the server
let response = client
  .handshake(Bytes::from("Hi"))
  .await
  .expect("error handshaking");

// Expect the server responded with 'Ho'
assert_eq!(response, Bytes::from("Ho"));
# }
```

---
