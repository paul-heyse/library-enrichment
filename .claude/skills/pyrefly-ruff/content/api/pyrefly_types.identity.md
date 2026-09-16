# `pyrefly_types::identity`

Crate `pyrefly_types` · 1 public items · structured records in [`model/pyrefly_types.identity.json`](../model/pyrefly_types.identity.json)

## IdentityIgnored

`struct` · `pyrefly_types::identity::IdentityIgnored`

```rust
struct IdentityIgnored<T>
```

**Implements**: `core::ops::deref::Deref`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &T
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, _other: &Self, _ctx: &mut TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, _: &mut dyn FnMut(&'a Type))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, _: &mut dyn FnMut(&mut Type))
```

A wrapper for auxiliary data whose identity should be completely ignored
in equality, hashing, ordering, and type-equality comparisons.
`IdentityIgnored<T>` always compares as equal, hashes as a no-op, and
orders as `Equal` — making it transparent to all identity checks.

This is useful for attaching auxiliary data (e.g. closure caches) to
types that derive `PartialEq`, `Hash`, `Ord`, etc. without affecting
their logical identity.

---
