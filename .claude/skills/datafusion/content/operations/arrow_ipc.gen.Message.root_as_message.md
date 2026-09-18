# `arrow_ipc::gen::Message::root_as_message`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Message.root_as_message.json).

<a id="op-e9c11d0f9e47e1f1dd2350d7"></a>
## root_as_message

`function` · `arrow_ipc::gen::Message::root_as_message` · arrow-ipc 59.3.0

```rust
fn root_as_message(buf: &[u8]) -> Result<Message<'_>, flatbuffers::InvalidFlatbuffer>
```

Source: `src/gen/Message.rs:1441`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Verifies that a buffer of bytes contains a `Message`
and returns it.
Note that verification is still experimental and may not
catch every error, or be maximally performant. For the
previous, unchecked, behavior use
`root_as_message_unchecked`.
