# `arrow_ipc::gen::Schema::size_prefixed_root_as_schema`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Schema.size_prefixed_root_as_schema.json).

<a id="op-4936eb32210e649adf6328e0"></a>
## size_prefixed_root_as_schema

`function` · `arrow_ipc::gen::Schema::size_prefixed_root_as_schema` · arrow-ipc 59.3.0

```rust
fn size_prefixed_root_as_schema(buf: &[u8]) -> Result<Schema<'_>, flatbuffers::InvalidFlatbuffer>
```

Source: `src/gen/Schema.rs:5572`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Verifies that a buffer of bytes contains a size prefixed
`Schema` and returns it.
Note that verification is still experimental and may not
catch every error, or be maximally performant. For the
previous, unchecked, behavior use
`size_prefixed_root_as_schema_unchecked`.
