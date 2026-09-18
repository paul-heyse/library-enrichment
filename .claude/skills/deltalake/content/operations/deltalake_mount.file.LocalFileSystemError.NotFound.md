# `deltalake_mount::file::LocalFileSystemError::NotFound`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_mount.file.LocalFileSystemError.NotFound.json).

<a id="op-1fa0936ba3685363f98ba9df"></a>
## path

`struct_field` · `deltalake_mount::file::LocalFileSystemError::NotFound::path` · deltalake-mount 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
path: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/mount/src/file.rs#L35).

Source: `crates/mount/src/file.rs:35`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Provided path which does not exist

<a id="op-1dec166231c5191270edb76b"></a>
## source

`struct_field` · `deltalake_mount::file::LocalFileSystemError::NotFound::source` · deltalake-mount 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
source: Box<dyn std::error::Error + Send + Sync + 'static>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/mount/src/file.rs#L37).

Source: `crates/mount/src/file.rs:37`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Originating error
