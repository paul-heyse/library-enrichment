# `arrow_cast::base64::b64_decode`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.base64.b64_decode.json).

<a id="op-800553f6622cf32aaa87203c"></a>
## b64_decode

`function` · `arrow_cast::base64::b64_decode` · arrow-cast 59.3.0

```rust
fn b64_decode<E: Engine, O: OffsetSizeTrait>(engine: &E, array: &arrow_array::GenericBinaryArray<O>) -> Result<arrow_array::GenericBinaryArray<O>, arrow_schema::ArrowError>
```

Source: `src/base64.rs:62`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Base64 decode each element of `array` with the provided [`Engine`]

Unresolved upstream links (retained, not inferred): ``Engine``.
