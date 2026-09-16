# `deltalake_mount::error`

Crate `deltalake-mount` · 1 public items · structured records in [`model/deltalake_mount.error.json`](../model/deltalake_mount.error.json)

## Error

`enum` · `deltalake_mount::error::Error`

```rust
enum Error
```

**Variants**: `Parse`, `UnknownConfigKey`, `AllowUnsafeRenameNotSpecified`, `ObjectStore`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(source: object_store::Error) -> Self
```

**via `core::error::Error`**

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private20::Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

---
