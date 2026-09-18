# `arrow_ipc::gen::Schema::size_prefixed_root_as_schema_with_opts`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Schema.size_prefixed_root_as_schema_with_opts.json).

<a id="op-087ffc3e21b71be40fe313e4"></a>
## size_prefixed_root_as_schema_with_opts

`function` · `arrow_ipc::gen::Schema::size_prefixed_root_as_schema_with_opts` · arrow-ipc 59.3.0

```rust
fn size_prefixed_root_as_schema_with_opts<'b, 'o>(opts: &'o flatbuffers::VerifierOptions, buf: &'b [u8]) -> Result<Schema<'b>, flatbuffers::InvalidFlatbuffer>
```

Source: `src/gen/Schema.rs:5595`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Verifies, with the given verifier options, that a buffer of
bytes contains a size prefixed `Schema` and returns
it. Note that verification is still experimental and may not
catch every error, or be maximally performant. For the
previous, unchecked, behavior use
`root_as_schema_unchecked`.
