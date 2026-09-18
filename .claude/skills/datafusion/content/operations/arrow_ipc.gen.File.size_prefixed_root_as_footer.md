# `arrow_ipc::gen::File::size_prefixed_root_as_footer`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.File.size_prefixed_root_as_footer.json).

<a id="op-de7827e7f6a109b8236573f8"></a>
## size_prefixed_root_as_footer

`function` · `arrow_ipc::gen::File::size_prefixed_root_as_footer` · arrow-ipc 59.3.0

```rust
fn size_prefixed_root_as_footer(buf: &[u8]) -> Result<Footer<'_>, flatbuffers::InvalidFlatbuffer>
```

Source: `src/gen/File.rs:444`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Verifies that a buffer of bytes contains a size prefixed
`Footer` and returns it.
Note that verification is still experimental and may not
catch every error, or be maximally performant. For the
previous, unchecked, behavior use
`size_prefixed_root_as_footer_unchecked`.
