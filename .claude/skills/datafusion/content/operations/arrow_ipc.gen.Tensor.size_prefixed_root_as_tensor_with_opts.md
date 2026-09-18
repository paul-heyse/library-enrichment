# `arrow_ipc::gen::Tensor::size_prefixed_root_as_tensor_with_opts`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Tensor.size_prefixed_root_as_tensor_with_opts.json).

<a id="op-3a420c166dc5ecc2f9f0b503"></a>
## size_prefixed_root_as_tensor_with_opts

`function` · `arrow_ipc::gen::Tensor::size_prefixed_root_as_tensor_with_opts` · arrow-ipc 59.3.0

```rust
fn size_prefixed_root_as_tensor_with_opts<'b, 'o>(opts: &'o flatbuffers::VerifierOptions, buf: &'b [u8]) -> Result<Tensor<'b>, flatbuffers::InvalidFlatbuffer>
```

Source: `src/gen/Tensor.rs:1174`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Verifies, with the given verifier options, that a buffer of
bytes contains a size prefixed `Tensor` and returns
it. Note that verification is still experimental and may not
catch every error, or be maximally performant. For the
previous, unchecked, behavior use
`root_as_tensor_unchecked`.
