# `deltalake_mount::file`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_mount.file.json).

<a id="op-ca1dcc03ceb81ae16d439876"></a>
## file

`module` · `deltalake_mount::file` · deltalake-mount 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod file
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/mount/src/file.rs#L1).

Source: `crates/mount/src/file.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Mount file storage backend. This backend read and write objects from mounted filesystem.

The mount file storage backend is not multi-writer safe.
