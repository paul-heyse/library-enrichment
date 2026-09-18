# `arrow_ipc::gen::Schema::root_as_schema`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Schema.root_as_schema.json).

<a id="op-6220358d5a0e35c5988a2d36"></a>
## root_as_schema

`function` · `arrow_ipc::gen::Schema::root_as_schema` · arrow-ipc 59.3.0

```rust
fn root_as_schema(buf: &[u8]) -> Result<Schema<'_>, flatbuffers::InvalidFlatbuffer>
```

Source: `src/gen/Schema.rs:5562`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Verifies that a buffer of bytes contains a `Schema`
and returns it.
Note that verification is still experimental and may not
catch every error, or be maximally performant. For the
previous, unchecked, behavior use
`root_as_schema_unchecked`.
