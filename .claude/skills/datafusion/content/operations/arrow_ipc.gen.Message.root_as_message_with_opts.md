# `arrow_ipc::gen::Message::root_as_message_with_opts`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Message.root_as_message_with_opts.json).

<a id="op-6dbcd241fa7b6150454fc21b"></a>
## root_as_message_with_opts

`function` · `arrow_ipc::gen::Message::root_as_message_with_opts` · arrow-ipc 59.3.0

```rust
fn root_as_message_with_opts<'b, 'o>(opts: &'o flatbuffers::VerifierOptions, buf: &'b [u8]) -> Result<Message<'b>, flatbuffers::InvalidFlatbuffer>
```

Source: `src/gen/Message.rs:1463`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Verifies, with the given options, that a buffer of bytes
contains a `Message` and returns it.
Note that verification is still experimental and may not
catch every error, or be maximally performant. For the
previous, unchecked, behavior use
`root_as_message_unchecked`.
