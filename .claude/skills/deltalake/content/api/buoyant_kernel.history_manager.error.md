# `buoyant_kernel::history_manager::error`

Crate `buoyant_kernel` · 2 public items · structured records in [`model/buoyant_kernel.history_manager.error.json`](../model/buoyant_kernel.history_manager.error.json)

## LogHistoryError

`enum` · `buoyant_kernel::history_manager::error::LogHistoryError`

Also reachable as `delta_kernel::history_manager::error::LogHistoryError`

```rust
enum LogHistoryError
```

**Variants**: `NoCommitsFound`, `InvalidTimestampRange`, `EmptyTimestampRange`, `TimestampOutOfRange`, `NoRecreatableCommit`, `Internal`

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

Represents errors that can occur when converting commit timestamps to versions.

---

## NearestTimestamp

`enum` · `buoyant_kernel::history_manager::error::NearestTimestamp`

Also reachable as `delta_kernel::history_manager::error::NearestTimestamp`

```rust
enum NearestTimestamp
```

**Variants**: `Earliest`, `Latest`, `Unknown`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

The nearest retained timestamp on the side of the search bound that an
out-of-range search failed against. Engines surface this to users so the
error message can point at a valid timestamp.

`Earliest` and `Latest` carry the boundary commit's timestamp. `Unknown`
means no boundary timestamp was available, e.g. empty log or a failure
reading the boundary commit's In-Commit Timestamp at the error site.

---
