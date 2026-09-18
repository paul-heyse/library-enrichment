# `arrow_ipc::gen::Message::size_prefixed_root_as_message_unchecked`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Message.size_prefixed_root_as_message_unchecked.json).

<a id="op-7683a2590fecfb1bcf2f573f"></a>
## size_prefixed_root_as_message_unchecked

`function` · `arrow_ipc::gen::Message::size_prefixed_root_as_message_unchecked` · arrow-ipc 59.3.0

```rust
unsafe fn size_prefixed_root_as_message_unchecked(buf: &[u8]) -> Message<'_>
```

Source: `src/gen/Message.rs:1493`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Assumes, without verification, that a buffer of bytes contains a size prefixed Message and returns it.
# Safety
Callers must trust the given bytes do indeed contain a valid size prefixed `Message`.
