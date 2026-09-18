# `arrow_ipc::gen::Schema::root_as_schema_unchecked`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Schema.root_as_schema_unchecked.json).

<a id="op-734e0d09d438b33c852ece47"></a>
## root_as_schema_unchecked

`function` · `arrow_ipc::gen::Schema::root_as_schema_unchecked` · arrow-ipc 59.3.0

```rust
unsafe fn root_as_schema_unchecked(buf: &[u8]) -> Schema<'_>
```

Source: `src/gen/Schema.rs:5605`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Assumes, without verification, that a buffer of bytes contains a Schema and returns it.
# Safety
Callers must trust the given bytes do indeed contain a valid `Schema`.
