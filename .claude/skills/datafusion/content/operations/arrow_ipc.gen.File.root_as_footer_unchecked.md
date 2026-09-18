# `arrow_ipc::gen::File::root_as_footer_unchecked`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.File.root_as_footer_unchecked.json).

<a id="op-b12c36bacdceb61d323bbe51"></a>
## root_as_footer_unchecked

`function` · `arrow_ipc::gen::File::root_as_footer_unchecked` · arrow-ipc 59.3.0

```rust
unsafe fn root_as_footer_unchecked(buf: &[u8]) -> Footer<'_>
```

Source: `src/gen/File.rs:477`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Assumes, without verification, that a buffer of bytes contains a Footer and returns it.
# Safety
Callers must trust the given bytes do indeed contain a valid `Footer`.
