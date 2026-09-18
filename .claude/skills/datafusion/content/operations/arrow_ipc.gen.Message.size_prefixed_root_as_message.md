# `arrow_ipc::gen::Message::size_prefixed_root_as_message`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Message.size_prefixed_root_as_message.json).

<a id="op-1d9d721fba3da6a1d914ef5e"></a>
## size_prefixed_root_as_message

`function` · `arrow_ipc::gen::Message::size_prefixed_root_as_message` · arrow-ipc 59.3.0

```rust
fn size_prefixed_root_as_message(buf: &[u8]) -> Result<Message<'_>, flatbuffers::InvalidFlatbuffer>
```

Source: `src/gen/Message.rs:1451`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Verifies that a buffer of bytes contains a size prefixed
`Message` and returns it.
Note that verification is still experimental and may not
catch every error, or be maximally performant. For the
previous, unchecked, behavior use
`size_prefixed_root_as_message_unchecked`.
