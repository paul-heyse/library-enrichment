# `deltalake_mount::file::LocalFileSystemError::AlreadyExists`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_mount.file.LocalFileSystemError.AlreadyExists.json).

<a id="op-9a8993926605f56c52185020"></a>
## path

`struct_field` · `deltalake_mount::file::LocalFileSystemError::AlreadyExists::path` · deltalake-mount 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
path: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/mount/src/file.rs#L26).

Source: `crates/mount/src/file.rs:26`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Path of the already existing file

<a id="op-e0a52bbb98dda6271664308a"></a>
## source

`struct_field` · `deltalake_mount::file::LocalFileSystemError::AlreadyExists::source` · deltalake-mount 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
source: Box<dyn std::error::Error + Send + Sync + 'static>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/mount/src/file.rs#L28).

Source: `crates/mount/src/file.rs:28`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Originating error
