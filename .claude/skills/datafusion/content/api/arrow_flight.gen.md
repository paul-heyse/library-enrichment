# `arrow_flight::gen`

Crate `arrow-flight` · 21 public items · structured records in [`model/arrow_flight.gen.json`](../model/arrow_flight.gen.json)

## CancelStatus

`enum` · `arrow_flight::gen::CancelStatus`

Also reachable as `arrow_flight::CancelStatus`

```rust
enum CancelStatus
```

**Variants**: `Unspecified`, `Cancelled`, `Cancelling`, `NotCancellable`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<CancelStatus>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<CancelStatus, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.gen.CancelStatus.md).



The result of a cancel operation.

This is used by CancelFlightInfoResult.status.

---

## Action

`struct` · `arrow_flight::gen::Action`

Also reachable as `arrow_flight::Action`

```rust
struct Action
```

**Fields**: `type`, `body`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(action_type: impl Into<String>, body: impl Into<Bytes>) -> Self
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.gen.Action.md).



An opaque action specific for the service.

---

## ActionType

`struct` · `arrow_flight::gen::ActionType`

Also reachable as `arrow_flight::ActionType`

```rust
struct ActionType
```

**Fields**: `type`, `description`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.gen.ActionType.md).



Describes an available action, including both the name used for execution
along with a short description of the purpose of the action.

---

## BasicAuth

`struct` · `arrow_flight::gen::BasicAuth`

Also reachable as `arrow_flight::BasicAuth`

```rust
struct BasicAuth
```

**Fields**: `username`, `password`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.gen.BasicAuth.md).



A message for doing simple auth.

---

## CancelFlightInfoRequest

`struct` · `arrow_flight::gen::CancelFlightInfoRequest`

Also reachable as `arrow_flight::CancelFlightInfoRequest`

```rust
struct CancelFlightInfoRequest
```

**Fields**: `info`

**Implements**: `core::fmt::Display`, `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(info: FlightInfo) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.gen.CancelFlightInfoRequest.md).



The request of the CancelFlightInfo action.

The request should be stored in Action.body.

---

## CancelFlightInfoResult

`struct` · `arrow_flight::gen::CancelFlightInfoResult`

Also reachable as `arrow_flight::CancelFlightInfoResult`

```rust
struct CancelFlightInfoResult
```

**Fields**: `status`

**Implements**: `core::fmt::Display`, `prost::message::Message`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn new(status: CancelStatus) -> Self
fn set_status(&mut self, value: CancelStatus)
fn status(&self) -> CancelStatus
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.gen.CancelFlightInfoResult.md).



The result of the CancelFlightInfo action.

The result should be stored in Result.body.

---

## Criteria

`struct` · `arrow_flight::gen::Criteria`

Also reachable as `arrow_flight::Criteria`

```rust
struct Criteria
```

**Fields**: `expression`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.gen.Criteria.md).



A service specific expression that can be used to return a limited set
of available Arrow Flight streams.

---

## Empty

`struct` · `arrow_flight::gen::Empty`

Also reachable as `arrow_flight::Empty`

```rust
struct Empty
```

**Implements**: `prost::message::Message`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.gen.Empty.md).


---

## FlightData

`struct` · `arrow_flight::gen::FlightData`

Also reachable as `arrow_flight::FlightData`

```rust
struct FlightData
```

**Fields**: `flight_descriptor`, `data_header`, `app_metadata`, `data_body`

**Implements**: `core::convert::From`, `core::fmt::Display`, `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn new() -> Self
fn with_app_metadata(self, app_metadata: impl Into<Bytes>) -> Self
fn with_data_body(self, data_body: impl Into<Bytes>) -> Self
fn with_data_header(self, data_header: impl Into<Bytes>) -> Self
fn with_descriptor(self, flight_descriptor: FlightDescriptor) -> Self
```

**via `core::convert::From`**

```rust
fn from(data: EncodedData) -> Self
fn from(schema_ipc: SchemaAsIpc<'_>) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.gen.FlightData.md).



A batch of Arrow data as part of a stream of batches.

---

## FlightDescriptor

`struct` · `arrow_flight::gen::FlightDescriptor`

Also reachable as `arrow_flight::FlightDescriptor`

```rust
struct FlightDescriptor
```

**Fields**: `type`, `cmd`, `path`

**Implements**: `core::fmt::Display`, `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn new_cmd(cmd: impl Into<Bytes>) -> Self
fn new_path(path: Vec<String>) -> Self
fn set_type(&mut self, value: flight_descriptor::DescriptorType)
fn type(&self) -> flight_descriptor::DescriptorType
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.gen.FlightDescriptor.md).



The name or tag for a Flight. May be used as a way to retrieve or generate
a flight or be used to expose a set of previously defined flights.

---

## FlightEndpoint

`struct` · `arrow_flight::gen::FlightEndpoint`

Also reachable as `arrow_flight::FlightEndpoint`

```rust
struct FlightEndpoint
```

**Fields**: `ticket`, `location`, `expiration_time`, `app_metadata`

**Implements**: `core::fmt::Display`, `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn new() -> FlightEndpoint
fn with_app_metadata(self, app_metadata: impl Into<Bytes>) -> Self
fn with_expiration_time(self, expiration_time: Timestamp) -> Self
fn with_location(self, uri: impl Into<String>) -> Self
fn with_ticket(self, ticket: Ticket) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.gen.FlightEndpoint.md).



A particular stream or split associated with a flight.

---

## FlightInfo

`struct` · `arrow_flight::gen::FlightInfo`

Also reachable as `arrow_flight::FlightInfo`

```rust
struct FlightInfo
```

**Fields**: `schema`, `flight_descriptor`, `endpoint`, `total_records`, `total_bytes`, `ordered`, `app_metadata`

**Implements**: `core::fmt::Display`, `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (10)

```rust
fn new() -> FlightInfo
fn try_decode_schema(self) -> std::result::Result<Schema, arrow_schema::ArrowError>
fn try_with_schema(self, schema: &Schema) -> std::result::Result<Self, arrow_schema::ArrowError>
fn with_app_metadata(self, app_metadata: impl Into<Bytes>) -> Self
fn with_descriptor(self, flight_descriptor: FlightDescriptor) -> Self
fn with_endpoint(self, endpoint: FlightEndpoint) -> Self
fn with_endpoints(self, endpoints: Vec<FlightEndpoint>) -> Self
fn with_ordered(self, ordered: bool) -> Self
fn with_total_bytes(self, total_bytes: i64) -> Self
fn with_total_records(self, total_records: i64) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.gen.FlightInfo.md).



The access coordinates for retrieval of a dataset. With a FlightInfo, a
consumer is able to determine how to retrieve a dataset.

---

## HandshakeRequest

`struct` · `arrow_flight::gen::HandshakeRequest`

Also reachable as `arrow_flight::HandshakeRequest`

```rust
struct HandshakeRequest
```

**Fields**: `protocol_version`, `payload`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.gen.HandshakeRequest.md).



The request that a client provides to a server on handshake.

---

## HandshakeResponse

`struct` · `arrow_flight::gen::HandshakeResponse`

Also reachable as `arrow_flight::HandshakeResponse`

```rust
struct HandshakeResponse
```

**Fields**: `protocol_version`, `payload`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.gen.HandshakeResponse.md).


---

## Location

`struct` · `arrow_flight::gen::Location`

Also reachable as `arrow_flight::Location`

```rust
struct Location
```

**Fields**: `uri`

**Implements**: `core::fmt::Display`, `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.gen.Location.md).



A location where a Flight service will accept retrieval of a particular
stream given a ticket.

---

## PollInfo

`struct` · `arrow_flight::gen::PollInfo`

Also reachable as `arrow_flight::PollInfo`

```rust
struct PollInfo
```

**Fields**: `info`, `flight_descriptor`, `progress`, `expiration_time`

**Implements**: `core::fmt::Display`, `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (6)

```rust
fn new() -> Self
fn progress(&self) -> f64
fn try_with_progress(self, progress: f64) -> std::result::Result<Self, arrow_schema::ArrowError>
fn with_descriptor(self, flight_descriptor: FlightDescriptor) -> Self
fn with_expiration_time(self, expiration_time: Timestamp) -> Self
fn with_info(self, info: FlightInfo) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.gen.PollInfo.md).



The information to process a long-running query.

---

## PutResult

`struct` · `arrow_flight::gen::PutResult`

Also reachable as `arrow_flight::PutResult`

```rust
struct PutResult
```

**Fields**: `app_metadata`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.gen.PutResult.md).


*
The response message associated with the submission of a DoPut.

---

## RenewFlightEndpointRequest

`struct` · `arrow_flight::gen::RenewFlightEndpointRequest`

Also reachable as `arrow_flight::RenewFlightEndpointRequest`

```rust
struct RenewFlightEndpointRequest
```

**Fields**: `endpoint`

**Implements**: `core::fmt::Display`, `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(endpoint: FlightEndpoint) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.gen.RenewFlightEndpointRequest.md).



The request of the RenewFlightEndpoint action.

The request should be stored in Action.body.

---

## Result

`struct` · `arrow_flight::gen::Result`

Also reachable as `arrow_flight::Result`

```rust
struct Result
```

**Fields**: `body`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(body: impl Into<Bytes>) -> Self
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.gen.Result.md).



An opaque result returned after executing an action.

---

## SchemaResult

`struct` · `arrow_flight::gen::SchemaResult`

Also reachable as `arrow_flight::SchemaResult`

```rust
struct SchemaResult
```

**Fields**: `schema`

**Implements**: `core::convert::TryFrom`, `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::TryFrom`**

```rust
fn try_from(schema_ipc: SchemaAsIpc<'_>) -> std::result::Result<Self, arrow_schema::ArrowError>
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.gen.SchemaResult.md).



Wrap the result of a getSchema call

---

## Ticket

`struct` · `arrow_flight::gen::Ticket`

Also reachable as `arrow_flight::Ticket`

```rust
struct Ticket
```

**Fields**: `ticket`

**Implements**: `core::fmt::Display`, `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(ticket: impl Into<Bytes>) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.gen.Ticket.md).



An opaque identifier that the service can use to retrieve a particular
portion of a stream.

Tickets are meant to be single use. It is an error/application-defined
behavior to reuse a ticket.

---
