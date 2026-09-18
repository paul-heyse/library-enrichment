# `arrow_flight::decode::DecodedPayload`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.decode.DecodedPayload.json).

<a id="op-73c8a48d99498f8d05355e7c"></a>
## DecodedPayload

`enum` · `arrow_flight::decode::DecodedPayload` · arrow-flight 59.3.0

```rust
enum DecodedPayload
```

Source: `src/decode.rs:456`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The result of decoding [`FlightData`](../operations/arrow_flight.gen.FlightData.md#op-d385f038797edc1454fc35a5)

<a id="op-b22dd1782548afeb99f1b736"></a>
## None

`variant` · `arrow_flight::decode::DecodedPayload::None` · arrow-flight 59.3.0

```rust
None
```

Source: `src/decode.rs:458`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

None (no data was sent in the corresponding FlightData)

<a id="op-21aa61434da895f931836c4f"></a>
## RecordBatch

`variant` · `arrow_flight::decode::DecodedPayload::RecordBatch` · arrow-flight 59.3.0

```rust
RecordBatch
```

Source: `src/decode.rs:464`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

A decoded Record batch.

<a id="op-a4982d07ce8ca38d5d8486ca"></a>
## Schema

`variant` · `arrow_flight::decode::DecodedPayload::Schema` · arrow-flight 59.3.0

```rust
Schema
```

Source: `src/decode.rs:461`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

A decoded Schema message

<a id="op-192044a307a208f1239638de"></a>
## fmt

`function` · `arrow_flight::decode::DecodedPayload::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::decode::DecodedPayload", "path": "DecodedPayload"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [455, 10], "end": [455, 15], "filename": "src/decode.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/decode.rs:455`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
