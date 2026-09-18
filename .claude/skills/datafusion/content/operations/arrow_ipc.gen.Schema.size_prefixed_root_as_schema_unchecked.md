# `arrow_ipc::gen::Schema::size_prefixed_root_as_schema_unchecked`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Schema.size_prefixed_root_as_schema_unchecked.json).

<a id="op-ad5e780d0b4181ff238ea6ca"></a>
## size_prefixed_root_as_schema_unchecked

`function` · `arrow_ipc::gen::Schema::size_prefixed_root_as_schema_unchecked` · arrow-ipc 59.3.0

```rust
unsafe fn size_prefixed_root_as_schema_unchecked(buf: &[u8]) -> Schema<'_>
```

Source: `src/gen/Schema.rs:5612`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Assumes, without verification, that a buffer of bytes contains a size prefixed Schema and returns it.
# Safety
Callers must trust the given bytes do indeed contain a valid size prefixed `Schema`.
