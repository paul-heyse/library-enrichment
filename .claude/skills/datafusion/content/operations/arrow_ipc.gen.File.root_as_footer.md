# `arrow_ipc::gen::File::root_as_footer`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.File.root_as_footer.json).

<a id="op-84d2f3ca91b59b3901a0dcb8"></a>
## root_as_footer

`function` · `arrow_ipc::gen::File::root_as_footer` · arrow-ipc 59.3.0

```rust
fn root_as_footer(buf: &[u8]) -> Result<Footer<'_>, flatbuffers::InvalidFlatbuffer>
```

Source: `src/gen/File.rs:434`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Verifies that a buffer of bytes contains a `Footer`
and returns it.
Note that verification is still experimental and may not
catch every error, or be maximally performant. For the
previous, unchecked, behavior use
`root_as_footer_unchecked`.
