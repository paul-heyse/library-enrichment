# `arrow_ipc::gen::File::root_as_footer_with_opts`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.File.root_as_footer_with_opts.json).

<a id="op-c3e9edd6c34c3bc6d18a543a"></a>
## root_as_footer_with_opts

`function` · `arrow_ipc::gen::File::root_as_footer_with_opts` · arrow-ipc 59.3.0

```rust
fn root_as_footer_with_opts<'b, 'o>(opts: &'o flatbuffers::VerifierOptions, buf: &'b [u8]) -> Result<Footer<'b>, flatbuffers::InvalidFlatbuffer>
```

Source: `src/gen/File.rs:454`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Verifies, with the given options, that a buffer of bytes
contains a `Footer` and returns it.
Note that verification is still experimental and may not
catch every error, or be maximally performant. For the
previous, unchecked, behavior use
`root_as_footer_unchecked`.
