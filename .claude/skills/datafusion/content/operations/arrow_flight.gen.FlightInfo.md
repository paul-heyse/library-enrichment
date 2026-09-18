# `arrow_flight::gen::FlightInfo`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.gen.FlightInfo.json).

<a id="op-422a7589713733d24ed1403b"></a>
## FlightInfo

`struct` · `arrow_flight::gen::FlightInfo` · arrow-flight 59.3.0

```rust
struct FlightInfo
```

Source: `src/arrow.flight.protocol.rs:184`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


The access coordinates for retrieval of a dataset. With a FlightInfo, a
consumer is able to determine how to retrieve a dataset.

<a id="op-9b6284c831cdaf8fe6e1875c"></a>
## app_metadata

`struct_field` · `arrow_flight::gen::FlightInfo::app_metadata` · arrow-flight 59.3.0

```rust
app_metadata: ::prost::bytes::Bytes
```

Source: `src/arrow.flight.protocol.rs:238`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Application-defined metadata.

There is no inherent or required relationship between this
and the app_metadata fields in the FlightEndpoints or resulting
FlightData messages. Since this metadata is application-defined,
a given application could define there to be a relationship,
but there is none required by the spec.

<a id="op-a9521dce5bfc390f50c2ef6c"></a>
## clear

`function` · `arrow_flight::gen::FlightInfo::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightInfo", "path": "FlightInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 28], "end": [183, 44], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/arrow.flight.protocol.rs:183`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8bbe02ce37676783e10116fd"></a>
## clone

`function` · `arrow_flight::gen::FlightInfo::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> FlightInfo
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightInfo", "path": "FlightInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 10], "end": [183, 15], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/arrow.flight.protocol.rs:183`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a2fc0a82f5d7a535471a889"></a>
## default

`function` · `arrow_flight::gen::FlightInfo::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightInfo", "path": "FlightInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 28], "end": [183, 44], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/arrow.flight.protocol.rs:183`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-87d32d07cd2a1edbb7c31c09"></a>
## encoded_len

`function` · `arrow_flight::gen::FlightInfo::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightInfo", "path": "FlightInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 28], "end": [183, 44], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/arrow.flight.protocol.rs:183`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bbf600b058d3cf7c736ea994"></a>
## endpoint

`struct_field` · `arrow_flight::gen::FlightInfo::endpoint` · arrow-flight 59.3.0

```rust
endpoint: ::prost::alloc::vec::Vec<FlightEndpoint>
```

Source: `src/arrow.flight.protocol.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


A list of endpoints associated with the flight. To consume the
whole flight, all endpoints (and hence all Tickets) must be
consumed. Endpoints can be consumed in any order.

In other words, an application can use multiple endpoints to
represent partitioned data.

If the returned data has an ordering, an application can use
"FlightInfo.ordered = true" or should return the all data in a
single endpoint. Otherwise, there is no ordering defined on
endpoints or the data within.

A client can read ordered data by reading data from returned
endpoints, in order, from front to back.

Note that a client may ignore "FlightInfo.ordered = true". If an
ordering is important for an application, an application must
choose one of them:

* An application requires that all clients must read data in
   returned endpoints order.
* An application must return the all data in a single endpoint.

<a id="op-2434f39731c53e80d6460ef5"></a>
## eq

`function` · `arrow_flight::gen::FlightInfo::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &FlightInfo) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightInfo", "path": "FlightInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 17], "end": [183, 26], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/arrow.flight.protocol.rs:183`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-147beb01064f4c24532583de"></a>
## flight_descriptor

`struct_field` · `arrow_flight::gen::FlightInfo::flight_descriptor` · arrow-flight 59.3.0

```rust
flight_descriptor: ::core::option::Option<FlightDescriptor>
```

Source: `src/arrow.flight.protocol.rs:194`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


The descriptor associated with this info.

<a id="op-0858057bbd614523acb1ede1"></a>
## fmt

`function` · `arrow_flight::gen::FlightInfo::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightInfo", "path": "FlightInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [263, 1], "end": [287, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/lib.rs:264`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d387e77dbd9a89a5c33dd1b7"></a>
## fmt

`function` · `arrow_flight::gen::FlightInfo::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightInfo", "path": "FlightInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 28], "end": [183, 44], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow.flight.protocol.rs:183`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bfa4462317ba0263b6c15130"></a>
## new

`function` · `arrow_flight::gen::FlightInfo::new` · arrow-flight 59.3.0

```rust
fn new() -> FlightInfo
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightInfo", "path": "FlightInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [540, 1], "end": [643, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:566`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Create a new, empty `FlightInfo`, describing where to fetch flight data


# Example:
```
# use arrow_flight::{FlightInfo, Ticket, FlightDescriptor, FlightEndpoint};
# use arrow_schema::{Schema, Field, DataType};
# fn get_schema() -> Schema {
#   Schema::new(vec![
#     Field::new("a", DataType::Utf8, false),
#   ])
# }
#
// Create a new FlightInfo
let flight_info = FlightInfo::new()
  // Encode the Arrow schema
  .try_with_schema(&get_schema())
  .expect("encoding failed")
  .with_endpoint(
     FlightEndpoint::new()
       .with_ticket(Ticket::new("ticket contents")
     )
   )
  .with_descriptor(FlightDescriptor::new_cmd("RUN QUERY"));
```

<a id="op-46bc34913042c41fa86aba64"></a>
## ordered

`struct_field` · `arrow_flight::gen::FlightInfo::ordered` · arrow-flight 59.3.0

```rust
ordered: bool
```

Source: `src/arrow.flight.protocol.rs:228`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


FlightEndpoints are in the same order as the data.

<a id="op-438cc287e8aa221391593bb7"></a>
## schema

`struct_field` · `arrow_flight::gen::FlightInfo::schema` · arrow-flight 59.3.0

```rust
schema: ::prost::bytes::Bytes
```

Source: `src/arrow.flight.protocol.rs:190`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The schema of the dataset in its IPC form:
   4 bytes - an optional IPC_CONTINUATION_TOKEN prefix
   4 bytes - the byte length of the payload
   a flatbuffer Message whose header is the Schema

<a id="op-45e23715dbbe87f6ffef9a62"></a>
## total_bytes

`struct_field` · `arrow_flight::gen::FlightInfo::total_bytes` · arrow-flight 59.3.0

```rust
total_bytes: i64
```

Source: `src/arrow.flight.protocol.rs:224`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a6e3b085460e99458279d427"></a>
## total_records

`struct_field` · `arrow_flight::gen::FlightInfo::total_records` · arrow-flight 59.3.0

```rust
total_records: i64
```

Source: `src/arrow.flight.protocol.rs:222`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Set these to -1 if unknown.

<a id="op-993707bdd702fc64b0d658a5"></a>
## try_decode_schema

`function` · `arrow_flight::gen::FlightInfo::try_decode_schema` · arrow-flight 59.3.0

```rust
fn try_decode_schema(self) -> std::result::Result<Schema, arrow_schema::ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightInfo", "path": "FlightInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [540, 1], "end": [643, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:582`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Try and convert the data in this  `FlightInfo` into a [`Schema`](../operations/arrow_schema.schema.Schema.md#op-144709aa539d6163483c2050)

<a id="op-2bce28edad1453493207bf62"></a>
## try_with_schema

`function` · `arrow_flight::gen::FlightInfo::try_with_schema` · arrow-flight 59.3.0

```rust
fn try_with_schema(self, schema: &Schema) -> std::result::Result<Self, arrow_schema::ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightInfo", "path": "FlightInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [540, 1], "end": [643, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:593`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Specify the schema for the response.

Note this takes the arrow [`Schema`](../operations/arrow_schema.schema.Schema.md#op-144709aa539d6163483c2050) (not the IPC schema) and
encodes it using the default IPC options.

Returns an error if `schema` can not be encoded into IPC form.

<a id="op-74248d78195b133c30ac882b"></a>
## with_app_metadata

`function` · `arrow_flight::gen::FlightInfo::with_app_metadata` · arrow-flight 59.3.0

```rust
fn with_app_metadata(self, app_metadata: impl Into<Bytes>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightInfo", "path": "FlightInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [540, 1], "end": [643, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:639`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Add optional application specific metadata to the message

<a id="op-2613f1ac7224154556a87235"></a>
## with_descriptor

`function` · `arrow_flight::gen::FlightInfo::with_descriptor` · arrow-flight 59.3.0

```rust
fn with_descriptor(self, flight_descriptor: FlightDescriptor) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightInfo", "path": "FlightInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [540, 1], "end": [643, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:613`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Add a [`FlightDescriptor`](../operations/arrow_flight.gen.FlightDescriptor.md#op-87317d059af665e5e2fe8326) describing what this data is

<a id="op-425f2433f0cdb039e4db804d"></a>
## with_endpoint

`function` · `arrow_flight::gen::FlightInfo::with_endpoint` · arrow-flight 59.3.0

```rust
fn with_endpoint(self, endpoint: FlightEndpoint) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightInfo", "path": "FlightInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [540, 1], "end": [643, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:601`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Add specific a endpoint for fetching the data

<a id="op-2cd836c0ffe8383bbf42ae68"></a>
## with_endpoints

`function` · `arrow_flight::gen::FlightInfo::with_endpoints` · arrow-flight 59.3.0

```rust
fn with_endpoints(self, endpoints: Vec<FlightEndpoint>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightInfo", "path": "FlightInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [540, 1], "end": [643, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:607`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Add endpoints for fetching all data

<a id="op-73922c0a92b32faace33755f"></a>
## with_ordered

`function` · `arrow_flight::gen::FlightInfo::with_ordered` · arrow-flight 59.3.0

```rust
fn with_ordered(self, ordered: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightInfo", "path": "FlightInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [540, 1], "end": [643, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:633`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Specify if the response is [ordered] across endpoints

[ordered]: https://github.com/apache/arrow-rs/blob/17ca4d51d0490f9c65f5adde144f677dbc8300e7/format/Flight.proto#L269-L275

<a id="op-6afd2be96bf986d2f6aaf0bd"></a>
## with_total_bytes

`function` · `arrow_flight::gen::FlightInfo::with_total_bytes` · arrow-flight 59.3.0

```rust
fn with_total_bytes(self, total_bytes: i64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightInfo", "path": "FlightInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [540, 1], "end": [643, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:625`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Set the number of bytes in the result, if known

<a id="op-8fb612937e4a576a3f33df68"></a>
## with_total_records

`function` · `arrow_flight::gen::FlightInfo::with_total_records` · arrow-flight 59.3.0

```rust
fn with_total_records(self, total_records: i64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::FlightInfo", "path": "FlightInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [540, 1], "end": [643, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:619`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Set the number of records in the result, if known
