# `deltalake_core::kernel::transaction::conflict_checker`

Crate `deltalake-core` · 1 public items · structured records in [`model/deltalake_core.kernel.transaction.conflict_checker.json`](../model/deltalake_core.kernel.transaction.conflict_checker.json)

## CommitConflictError

`enum` · `deltalake_core::kernel::transaction::conflict_checker::CommitConflictError`

Also reachable as `deltalake::kernel::transaction::CommitConflictError`, `deltalake_core::kernel::transaction::CommitConflictError`

```rust
enum CommitConflictError
```

**Variants**: `ConcurrentAppend`, `ConcurrentDeleteRead`, `ConcurrentDeleteDelete`, `MetadataChanged`, `ConcurrentTransaction`, `ProtocolChanged`, `UnsupportedWriterVersion`, `UnsupportedReaderVersion`, `CorruptedState`, `Predicate`, `NoMetadata`

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

Exceptions raised during commit conflict resolution

---
