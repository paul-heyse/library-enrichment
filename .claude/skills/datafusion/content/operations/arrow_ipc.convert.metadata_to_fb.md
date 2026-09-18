# `arrow_ipc::convert::metadata_to_fb`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.convert.metadata_to_fb.json).

<a id="op-51c2ee85966595e9c4554639"></a>
## metadata_to_fb

`function` · `arrow_ipc::convert::metadata_to_fb` · arrow-ipc 59.3.0

```rust
fn metadata_to_fb<'a>(fbb: &mut flatbuffers::FlatBufferBuilder<'a>, metadata: &std::collections::HashMap<String, String>) -> flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<KeyValue<'a>>>>
```

Source: `src/convert.rs:132`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Push a key-value metadata into a FlatBufferBuilder and return [WIPOffset]

Unresolved upstream links (retained, not inferred): `WIPOffset`.
