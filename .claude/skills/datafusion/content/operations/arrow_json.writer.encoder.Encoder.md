# `arrow_json::writer::encoder::Encoder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_json.writer.encoder.Encoder.json).

<a id="op-21c065171dc1eb2068b0a606"></a>
## Encoder

`trait` · `arrow_json::writer::encoder::Encoder` · arrow-json 59.3.0

```rust
trait Encoder
```

Source: `src/writer/encoder.rs:297`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

A trait to format array values as JSON values

Nullability is handled by the caller to allow encoding nulls implicitly, i.e. `{}` instead of `{"a": null}`

<a id="op-ff37e3ea78a10b3c48881032"></a>
## encode

`function` · `arrow_json::writer::encoder::Encoder::encode` · arrow-json 59.3.0

```rust
fn encode(&mut self, idx: usize, out: &mut Vec<u8>)
```

Source: `src/writer/encoder.rs:301`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Encode the non-null value at index `idx` to `out`.

The behaviour is unspecified if `idx` corresponds to a null index.
