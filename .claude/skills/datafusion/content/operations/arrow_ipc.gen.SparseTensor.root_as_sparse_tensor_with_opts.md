# `arrow_ipc::gen::SparseTensor::root_as_sparse_tensor_with_opts`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.SparseTensor.root_as_sparse_tensor_with_opts.json).

<a id="op-2ea27dd65cae5ef49495230b"></a>
## root_as_sparse_tensor_with_opts

`function` · `arrow_ipc::gen::SparseTensor::root_as_sparse_tensor_with_opts` · arrow-ipc 59.3.0

```rust
fn root_as_sparse_tensor_with_opts<'b, 'o>(opts: &'o flatbuffers::VerifierOptions, buf: &'b [u8]) -> Result<SparseTensor<'b>, flatbuffers::InvalidFlatbuffer>
```

Source: `src/gen/SparseTensor.rs:2246`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Verifies, with the given options, that a buffer of bytes
contains a `SparseTensor` and returns it.
Note that verification is still experimental and may not
catch every error, or be maximally performant. For the
previous, unchecked, behavior use
`root_as_sparse_tensor_unchecked`.
