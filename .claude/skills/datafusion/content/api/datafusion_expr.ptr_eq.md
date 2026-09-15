# `datafusion_expr::ptr_eq`

Crate `datafusion-expr` · 3 public items · structured records in [`model/datafusion_expr.ptr_eq.json`](../model/datafusion_expr.ptr_eq.json)

## arc_ptr_eq

`function` · `datafusion_expr::ptr_eq::arc_ptr_eq`

```rust
fn arc_ptr_eq<T: ?Sized>(a: &std::sync::Arc<T>, b: &std::sync::Arc<T>) -> bool
```

Compares two `Arc` pointers for equality based on their underlying pointers values.
This is not equivalent to [`Arc::ptr_eq`] for fat pointers, see that method
for more information.

---

## arc_ptr_hash

`function` · `datafusion_expr::ptr_eq::arc_ptr_hash`

```rust
fn arc_ptr_hash<T: ?Sized>(a: &std::sync::Arc<T>, hasher: &mut impl Hasher)
```

Hashes an `Arc` pointer based on its underlying pointer value.
The general contract for this function is that if [`arc_ptr_eq`] returns `true`
for two `Arc`s, then this function should return the same hash value for both.

---

## PtrEq

`struct` · `datafusion_expr::ptr_eq::PtrEq`

```rust
struct PtrEq<Ptr: PointerType>
```

**Implements**: `core::convert::From`, `core::ops::deref::Deref`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**via `core::convert::From`**

```rust
fn from(ptr: Ptr) -> Self
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

A wrapper around a pointer that implements `Eq` and `Hash` comparing
the underlying pointer address.

If you have pointers to a `dyn UDF impl` consider using [`super::udf_eq::UdfEq`].

---
