# `arrow_flight::decode::DecodedFlightData`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.decode.DecodedFlightData.json).

<a id="op-f9dfa46385cb440a6c03cc52"></a>
## DecodedFlightData

`struct` · `arrow_flight::decode::DecodedFlightData` · arrow-flight 59.3.0

```rust
struct DecodedFlightData
```

Source: `src/decode.rs:416`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

FlightData and the decoded payload (Schema, RecordBatch), if any

<a id="op-194d1d1f1071154d21001b80"></a>
## app_metadata

`function` · `arrow_flight::decode::DecodedFlightData::app_metadata` · arrow-flight 59.3.0

```rust
fn app_metadata(&self) -> Bytes
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::decode::DecodedFlightData", "path": "DecodedFlightData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [423, 1], "end": [452, 2], "filename": "src/decode.rs"}, "trait": null, "trait_path": null}`

Source: `src/decode.rs:449`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Return the metadata field of the inner flight data

<a id="op-53408d842e369d6d14d91559"></a>
## fmt

`function` · `arrow_flight::decode::DecodedFlightData::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::decode::DecodedFlightData", "path": "DecodedFlightData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [415, 10], "end": [415, 15], "filename": "src/decode.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/decode.rs:415`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-902992b7f86909201f000d29"></a>
## inner

`struct_field` · `arrow_flight::decode::DecodedFlightData::inner` · arrow-flight 59.3.0

```rust
inner: FlightData
```

Source: `src/decode.rs:418`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The original FlightData message

<a id="op-e9127a3a45cd133327666b18"></a>
## new_none

`function` · `arrow_flight::decode::DecodedFlightData::new_none` · arrow-flight 59.3.0

```rust
fn new_none(inner: FlightData) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::decode::DecodedFlightData", "path": "DecodedFlightData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [423, 1], "end": [452, 2], "filename": "src/decode.rs"}, "trait": null, "trait_path": null}`

Source: `src/decode.rs:425`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Create a new DecodedFlightData with no payload

<a id="op-3908878d2ad656603f24e886"></a>
## new_record_batch

`function` · `arrow_flight::decode::DecodedFlightData::new_record_batch` · arrow-flight 59.3.0

```rust
fn new_record_batch(inner: FlightData, batch: RecordBatch) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::decode::DecodedFlightData", "path": "DecodedFlightData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [423, 1], "end": [452, 2], "filename": "src/decode.rs"}, "trait": null, "trait_path": null}`

Source: `src/decode.rs:441`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Create a new [`DecodedFlightData`](../operations/arrow_flight.decode.DecodedFlightData.md#op-f9dfa46385cb440a6c03cc52) with a [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) payload

<a id="op-4a2634ca0bf28e23021a3f94"></a>
## new_schema

`function` · `arrow_flight::decode::DecodedFlightData::new_schema` · arrow-flight 59.3.0

```rust
fn new_schema(inner: FlightData, schema: SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::decode::DecodedFlightData", "path": "DecodedFlightData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [423, 1], "end": [452, 2], "filename": "src/decode.rs"}, "trait": null, "trait_path": null}`

Source: `src/decode.rs:433`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Create a new DecodedFlightData with a [`Schema`](../operations/arrow_schema.schema.Schema.md#op-144709aa539d6163483c2050) payload

<a id="op-dc5d50ffdcd89d5c70aade1c"></a>
## payload

`struct_field` · `arrow_flight::decode::DecodedFlightData::payload` · arrow-flight 59.3.0

```rust
payload: DecodedPayload
```

Source: `src/decode.rs:420`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The decoded payload
