# `arrow_flight::sql::server::PeekableFlightDataStream`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.server.PeekableFlightDataStream.json).

<a id="op-6ac704d27e389b61fde39fce"></a>
## PeekableFlightDataStream

`struct` · `arrow_flight::sql::server::PeekableFlightDataStream` · arrow-flight 59.3.0

```rust
struct PeekableFlightDataStream
```

Source: `src/sql/server.rs:1064`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

A wrapper around [`Streaming<FlightData>`] that allows "peeking" at the
message at the front of the stream without consuming it.

This is needed because sometimes the first message in the stream will contain
a [`FlightDescriptor`](../operations/arrow_flight.gen.FlightDescriptor.md#op-87317d059af665e5e2fe8326) in addition to potentially any data, and the dispatch logic
must inspect this information.

# Example

[`PeekableFlightDataStream::peek`](../operations/arrow_flight.sql.server.PeekableFlightDataStream.md#op-c6759869200f442a8f61de5b) can be used to peek at the first message without
discarding it; otherwise, `PeekableFlightDataStream` can be used as a regular stream.
See the following example:

```no_run
use arrow_array::RecordBatch;
use arrow_flight::decode::FlightRecordBatchStream;
use arrow_flight::FlightDescriptor;
use arrow_flight::error::FlightError;
use arrow_flight::sql::server::PeekableFlightDataStream;
use tonic::{Request, Status};
use futures::TryStreamExt;

#[tokio::main]
async fn main() -> Result<(), Status> {
    let request: Request<PeekableFlightDataStream> = todo!();
    let stream: PeekableFlightDataStream = request.into_inner();

    // The first message contains the flight descriptor and the schema.
    // Read the flight descriptor without discarding the schema:
    let flight_descriptor: FlightDescriptor = stream
        .peek()
        .await
        .cloned()
        .transpose()?
        .and_then(|data| data.flight_descriptor)
        .expect("first message should contain flight descriptor");

    // Pass the stream through a decoder
    let batches: Vec<RecordBatch> = FlightRecordBatchStream::new_from_flight_data(
        request.into_inner().map_err(|e| e.into()),
    )
    .try_collect()
    .await?;
}
```

Unresolved upstream links (retained, not inferred): ``Streaming<FlightData>``.

<a id="op-9c8f288a691aeb65705f9db1"></a>
## Item

`assoc_type` · `arrow_flight::sql::server::PeekableFlightDataStream::Item` · arrow-flight 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::server::PeekableFlightDataStream", "path": "PeekableFlightDataStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1095, 1], "end": [1104, 2], "filename": "src/sql/server.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/sql/server.rs:1096`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-16e8e82014174c50361baacd"></a>
## into_inner

`function` · `arrow_flight::sql::server::PeekableFlightDataStream::into_inner` · arrow-flight 59.3.0

```rust
fn into_inner(self) -> Streaming<FlightData>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::server::PeekableFlightDataStream", "path": "PeekableFlightDataStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1068, 1], "end": [1093, 2], "filename": "src/sql/server.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/server.rs:1078`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Convert this stream into a `Streaming<FlightData>`.
Any messages observed through [`Self::peek`](../operations/arrow_flight.sql.server.PeekableFlightDataStream.md#op-c6759869200f442a8f61de5b) will be lost
after the conversion.

<a id="op-1289581fc57c721b2b5bdd98"></a>
## into_peekable

`function` · `arrow_flight::sql::server::PeekableFlightDataStream::into_peekable` · arrow-flight 59.3.0

```rust
fn into_peekable(self) -> Peekable<Streaming<FlightData>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::server::PeekableFlightDataStream", "path": "PeekableFlightDataStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1068, 1], "end": [1093, 2], "filename": "src/sql/server.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/server.rs:1085`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Convert this stream into a `Peekable<Streaming<FlightData>>`.
Preserves the state of the stream, so that calls to [`Self::peek`](../operations/arrow_flight.sql.server.PeekableFlightDataStream.md#op-c6759869200f442a8f61de5b)
and [`Self::poll_next`](../operations/arrow_flight.sql.server.PeekableFlightDataStream.md#op-29f410d20cfa84e15ec7726a) are the same.

<a id="op-c6759869200f442a8f61de5b"></a>
## peek

`function` · `arrow_flight::sql::server::PeekableFlightDataStream::peek` · arrow-flight 59.3.0

```rust
async fn peek(&mut self) -> Option<&Result<FlightData, Status>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::server::PeekableFlightDataStream", "path": "PeekableFlightDataStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1068, 1], "end": [1093, 2], "filename": "src/sql/server.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/server.rs:1090`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Peek at the head of this stream without advancing it.

<a id="op-29f410d20cfa84e15ec7726a"></a>
## poll_next

`function` · `arrow_flight::sql::server::PeekableFlightDataStream::poll_next` · arrow-flight 59.3.0

```rust
fn poll_next(Pin<&mut self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::server::PeekableFlightDataStream", "path": "PeekableFlightDataStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1095, 1], "end": [1104, 2], "filename": "src/sql/server.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/sql/server.rs:1098`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
