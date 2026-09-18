# `arrow_cast::base64::b64_encode`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.base64.b64_encode.json).

<a id="op-563422813fc4e1fac6f210ca"></a>
## b64_encode

`function` · `arrow_cast::base64::b64_encode` · arrow-cast 59.3.0

```rust
fn b64_encode<E: Engine, O: OffsetSizeTrait>(engine: &E, array: &arrow_array::GenericBinaryArray<O>) -> arrow_array::GenericStringArray<O>
```

Source: `src/base64.rs:35`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Base64 encode each element of `array` with the provided [`Engine`]

Panics if the `Engine` emits output that is not valid UTF-8. A correct
`Engine` never does, but it is a safe trait so a misbehaving impl could;
validating keeps the returned [`GenericStringArray`](../operations/arrow_array.array.string_array.GenericStringArray.md#op-86b79a80b65c0571602ced76) sound (#10284).

Unresolved upstream links (retained, not inferred): ``Engine``.
