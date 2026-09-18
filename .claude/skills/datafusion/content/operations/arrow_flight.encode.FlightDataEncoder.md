# `arrow_flight::encode::FlightDataEncoder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.encode.FlightDataEncoder.json).

<a id="op-e4d1d137c901fe86782ae657"></a>
## FlightDataEncoder

`struct` · `arrow_flight::encode::FlightDataEncoder` · arrow-flight 59.3.0

```rust
struct FlightDataEncoder
```

Source: `src/encode.rs:269`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Stream that encodes a stream of record batches to flight data.

See [`FlightDataEncoderBuilder`](../operations/arrow_flight.encode.FlightDataEncoderBuilder.md#op-417e00a5164d321214a222fc) for details and example.

<a id="op-79951497ee3a4b650f069205"></a>
## Item

`assoc_type` · `arrow_flight::encode::FlightDataEncoder::Item` · arrow-flight 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::encode::FlightDataEncoder", "path": "FlightDataEncoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [393, 1], "end": [438, 2], "filename": "src/encode.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/encode.rs:394`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9072e717861c149fdfc554ad"></a>
## known_schema

`function` · `arrow_flight::encode::FlightDataEncoder::known_schema` · arrow-flight 59.3.0

```rust
fn known_schema(&self) -> Option<SchemaRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::encode::FlightDataEncoder", "path": "FlightDataEncoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [292, 1], "end": [391, 2], "filename": "src/encode.rs"}, "trait": null, "trait_path": null}`

Source: `src/encode.rs:327`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Report the schema of the encoded data when known.
A schema is known when provided via the [`FlightDataEncoderBuilder::with_schema`](../operations/arrow_flight.encode.FlightDataEncoderBuilder.md#op-54884f346f4f7fe90f915141) method.

<a id="op-197a2db17d66353b179ef4af"></a>
## poll_next

`function` · `arrow_flight::encode::FlightDataEncoder::poll_next` · arrow-flight 59.3.0

```rust
fn poll_next(Pin<&mut self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::encode::FlightDataEncoder", "path": "FlightDataEncoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [393, 1], "end": [438, 2], "filename": "src/encode.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/encode.rs:396`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
