# `ruff_db::panic`

Crate `ruff_db` · 3 public items · structured records in [`model/ruff_db.panic.json`](../model/ruff_db.panic.json)

## catch_unwind

`function` · `ruff_db::panic::catch_unwind`

```rust
fn catch_unwind<F, R>(f: F) -> Result<R, PanicError> where F: FnOnce() -> R + std::panic::UnwindSafe
```

Invokes a closure, capturing and returning the cause of an unwinding panic if one occurs.

### Thread safety

This is implemented by installing a custom [panic hook](std::panic::set_hook).  This panic hook
is a global resource.  The hook that we install captures panic info in a thread-safe manner,
and also ensures that any threads that are _not_ currently using this `catch_unwind` wrapper
still use the previous hook (typically the default hook, which prints out panic information to
stderr).

We assume that there is nothing else running in this process that needs to install a competing
panic hook. We are careful to install our custom hook only once, and we do not ever restore
the previous hook (since you can always retain the previous hook's behavior by not calling this
wrapper).

---

## PanicError

`struct` · `ruff_db::panic::PanicError`

```rust
struct PanicError
```

**Fields**: `location`, `payload`, `backtrace`, `salsa_backtrace`

**Implements**: `core::fmt::Display`

**Derives**: Debug

**Methods** (2)

```rust
fn resume_unwind(self) -> never
fn to_diagnostic_message(&self, path: Option<impl std::fmt::Display>) -> String
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

---

## Payload

`struct` · `ruff_db::panic::Payload`

```rust
struct Payload
```

**Implements**: `core::fmt::Display`

**Derives**: Debug

**Methods** (1)

```rust
fn downcast_ref<R: Any>(&self) -> Option<&R>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

---
