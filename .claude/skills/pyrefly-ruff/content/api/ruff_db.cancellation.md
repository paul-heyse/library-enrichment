# `ruff_db::cancellation`

Crate `ruff_db` · 3 public items · structured records in [`model/ruff_db.cancellation.json`](../model/ruff_db.cancellation.json)

## Canceled

`struct` · `ruff_db::cancellation::Canceled`

```rust
struct Canceled
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

The operation was canceled by the provided [`CancellationToken`].

---

## CancellationToken

`struct` · `ruff_db::cancellation::CancellationToken`

```rust
struct CancellationToken
```

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn is_cancelled(&self) -> bool
```

Token signals whether an operation should be canceled.

---

## CancellationTokenSource

`struct` · `ruff_db::cancellation::CancellationTokenSource`

```rust
struct CancellationTokenSource
```

**Derives**: Clone, Debug, Default

**Methods** (3)

```rust
fn cancel(&self)
fn new() -> Self
fn token(&self) -> CancellationToken
```

Signals a [`CancellationToken`] that it should be canceled.

---
