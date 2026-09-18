# `deltalake_mount::file::LocalFileSystemError::Generic`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_mount.file.LocalFileSystemError.Generic.json).

<a id="op-6a4e5dd1d68e30d99ca648eb"></a>
## source

`struct_field` · `deltalake_mount::file::LocalFileSystemError::Generic::source` · deltalake-mount 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
source: Box<dyn std::error::Error + Send + Sync + 'static>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/mount/src/file.rs#L64).

Source: `crates/mount/src/file.rs:64`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Originating error

<a id="op-c7bea175cd0a9f27d74e7fd9"></a>
## store

`struct_field` · `deltalake_mount::file::LocalFileSystemError::Generic::store` · deltalake-mount 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
store: &'static str
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/mount/src/file.rs#L62).

Source: `crates/mount/src/file.rs:62`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

String name of the object store
