# `deltalake_lakefs::errors`

Crate `deltalake-lakefs` · 2 public items · structured records in [`model/deltalake_lakefs.errors.json`](../model/deltalake_lakefs.errors.json)

## LakeFSConfigError

`enum` · `deltalake_lakefs::errors::LakeFSConfigError`

```rust
enum LakeFSConfigError
```

**Variants**: `EndpointMissing`, `UsernameCredentialMissing`, `PasswordCredentialMissing`

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

---

## LakeFSOperationError

`enum` · `deltalake_lakefs::errors::LakeFSOperationError`

```rust
enum LakeFSOperationError
```

**Variants**: `HttpRequestFailed`, `UnauthorizedAction`, `CommitFailed`, `MergeFailed`, `CreateBranchFailed`, `DeleteBranchFailed`, `TransactionIdNotFound`

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

---
