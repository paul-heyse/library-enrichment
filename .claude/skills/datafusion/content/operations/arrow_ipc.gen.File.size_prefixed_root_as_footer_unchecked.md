# `arrow_ipc::gen::File::size_prefixed_root_as_footer_unchecked`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.File.size_prefixed_root_as_footer_unchecked.json).

<a id="op-63337deaa7b590a62ccacf18"></a>
## size_prefixed_root_as_footer_unchecked

`function` · `arrow_ipc::gen::File::size_prefixed_root_as_footer_unchecked` · arrow-ipc 59.3.0

```rust
unsafe fn size_prefixed_root_as_footer_unchecked(buf: &[u8]) -> Footer<'_>
```

Source: `src/gen/File.rs:484`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Assumes, without verification, that a buffer of bytes contains a size prefixed Footer and returns it.
# Safety
Callers must trust the given bytes do indeed contain a valid size prefixed `Footer`.
