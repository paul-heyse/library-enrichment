# `arrow_json::writer::encoder::NullableEncoder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_json.writer.encoder.NullableEncoder.json).

<a id="op-59e533bd96ea380a88b6ba68"></a>
## NullableEncoder

`struct` · `arrow_json::writer::encoder::NullableEncoder` · arrow-json 59.3.0

```rust
struct NullableEncoder<'a>
```

Source: `src/writer/encoder.rs:258`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

An encoder + a null buffer.
This is packaged together into a wrapper struct to minimize dynamic dispatch for null checks.

<a id="op-c6f6d4fe6ebf9bb2a300d452"></a>
## encode

`function` · `arrow_json::writer::encoder::NullableEncoder::encode` · arrow-json 59.3.0

```rust
fn encode(&mut self, idx: usize, out: &mut Vec<u8>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_json::writer::encoder::NullableEncoder", "path": "NullableEncoder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [263, 1], "end": [286, 2], "filename": "src/writer/encoder.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/encoder.rs:270`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Encode the value at index `idx` to `out`.

<a id="op-e6c4981621fac6247ae97edb"></a>
## encode

`function` · `arrow_json::writer::encoder::NullableEncoder::encode` · arrow-json 59.3.0

```rust
fn encode(&mut self, idx: usize, out: &mut Vec<u8>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_json::writer::encoder::NullableEncoder", "path": "NullableEncoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [288, 1], "end": [292, 2], "filename": "src/writer/encoder.rs"}, "trait": {"args": null, "id": "arrow_json::writer::encoder::Encoder", "path": "Encoder"}, "trait_path": "arrow_json::writer::encoder::Encoder"}`

Source: `src/writer/encoder.rs:289`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc04a4d14f9d759b9468e6d2"></a>
## has_nulls

`function` · `arrow_json::writer::encoder::NullableEncoder::has_nulls` · arrow-json 59.3.0

```rust
fn has_nulls(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_json::writer::encoder::NullableEncoder", "path": "NullableEncoder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [263, 1], "end": [286, 2], "filename": "src/writer/encoder.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/encoder.rs:280`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Returns whether the encoder has any nulls.

<a id="op-385f98a841afc1143482ef1e"></a>
## is_null

`function` · `arrow_json::writer::encoder::NullableEncoder::is_null` · arrow-json 59.3.0

```rust
fn is_null(&self, idx: usize) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_json::writer::encoder::NullableEncoder", "path": "NullableEncoder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [263, 1], "end": [286, 2], "filename": "src/writer/encoder.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/encoder.rs:275`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Returns whether the value at index `idx` is null.

<a id="op-4e46eb604c22d923a41e8c23"></a>
## new

`function` · `arrow_json::writer::encoder::NullableEncoder::new` · arrow-json 59.3.0

```rust
fn new(encoder: Box<dyn Encoder + 'a>, nulls: Option<NullBuffer>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_json::writer::encoder::NullableEncoder", "path": "NullableEncoder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [263, 1], "end": [286, 2], "filename": "src/writer/encoder.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/encoder.rs:265`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Create a new encoder with a null buffer.
