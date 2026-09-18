# `arrow_ipc::writer::EncodedData`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.writer.EncodedData.json).

<a id="op-7935547a30bcba70bbdf01dd"></a>
## EncodedData

`struct` · `arrow_ipc::writer::EncodedData` · arrow-ipc 59.3.0

```rust
struct EncodedData
```

Source: `src/writer.rs:2197`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Stores the encoded data, which is an crate::Message, and optional Arrow data

<a id="op-8003e38977f736f9aad1c238"></a>
## arrow_data

`struct_field` · `arrow_ipc::writer::EncodedData::arrow_data` · arrow-ipc 59.3.0

```rust
arrow_data: Vec<u8>
```

Source: `src/writer.rs:2201`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Arrow buffers to be written, should be an empty vec for schema messages

<a id="op-88ee97b547b9688baeaf4002"></a>
## ipc_message

`struct_field` · `arrow_ipc::writer::EncodedData::ipc_message` · arrow-ipc 59.3.0

```rust
ipc_message: Vec<u8>
```

Source: `src/writer.rs:2199`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

An encoded crate::Message
