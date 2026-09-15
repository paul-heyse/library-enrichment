# `arrow_cast::base64`

Crate `arrow-cast` · 2 public items · structured records in [`model/arrow_cast.base64.json`](../model/arrow_cast.base64.json)

## b64_decode

`function` · `arrow_cast::base64::b64_decode`

```rust
fn b64_decode<E: Engine, O: OffsetSizeTrait>(engine: &E, array: &arrow_array::GenericBinaryArray<O>) -> Result<arrow_array::GenericBinaryArray<O>, arrow_schema::ArrowError>
```

Base64 decode each element of `array` with the provided [`Engine`]

---

## b64_encode

`function` · `arrow_cast::base64::b64_encode`

```rust
fn b64_encode<E: Engine, O: OffsetSizeTrait>(engine: &E, array: &arrow_array::GenericBinaryArray<O>) -> arrow_array::GenericStringArray<O>
```

Base64 encode each element of `array` with the provided [`Engine`]

Panics if the `Engine` emits output that is not valid UTF-8. A correct
`Engine` never does, but it is a safe trait so a misbehaving impl could;
validating keeps the returned [`GenericStringArray`] sound (#10284).

---
