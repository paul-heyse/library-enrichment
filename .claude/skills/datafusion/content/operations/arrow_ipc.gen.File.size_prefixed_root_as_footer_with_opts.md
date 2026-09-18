# `arrow_ipc::gen::File::size_prefixed_root_as_footer_with_opts`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.File.size_prefixed_root_as_footer_with_opts.json).

<a id="op-c89bae67ddef5b46b7265027"></a>
## size_prefixed_root_as_footer_with_opts

`function` · `arrow_ipc::gen::File::size_prefixed_root_as_footer_with_opts` · arrow-ipc 59.3.0

```rust
fn size_prefixed_root_as_footer_with_opts<'b, 'o>(opts: &'o flatbuffers::VerifierOptions, buf: &'b [u8]) -> Result<Footer<'b>, flatbuffers::InvalidFlatbuffer>
```

Source: `src/gen/File.rs:467`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Verifies, with the given verifier options, that a buffer of
bytes contains a size prefixed `Footer` and returns
it. Note that verification is still experimental and may not
catch every error, or be maximally performant. For the
previous, unchecked, behavior use
`root_as_footer_unchecked`.
