# `deltalake_mount::file`

Crate `deltalake-mount` · 1 public items · structured records in [`model/deltalake_mount.file.json`](../model/deltalake_mount.file.json)

## LocalFileSystemError

`enum` · `deltalake_mount::file::LocalFileSystemError`
[Full member contracts, output types and access classification](../operations/deltalake_mount.file.LocalFileSystemError.md)

```rust
enum LocalFileSystemError
```

**Variants**: `AlreadyExists`, `NotFound`, `InvalidArgument`, `NullError`, `Generic`, `Tokio`

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::error::Error`**

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private20::Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Error raised by storage lock client

---
