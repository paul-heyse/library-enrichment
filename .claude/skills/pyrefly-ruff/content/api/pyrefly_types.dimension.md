# `pyrefly_types::dimension`

Crate `pyrefly_types` · 14 public items · structured records in [`model/pyrefly_types.dimension.json`](../model/pyrefly_types.dimension.json)

## Int

`enum` · `pyrefly_types::dimension::Int`

```rust
enum Int
```

**Variants**: `Literal`, `Int`, `Symbolic`, `Add`, `Sub`, `Mul`, `FloorDiv`, `Pow`

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (10)

```rust
fn add(left: Type, right: Type) -> Self
fn as_literal(&self) -> Option<i64>
fn floor_div(left: Type, right: Type) -> Self
fn from_type(ty: &Type) -> Option<Int>
fn is_literal(&self) -> bool
fn literal(value: i64) -> Self
fn materialize(&mut self)
fn mul(left: Type, right: Type) -> Self
fn pow(left: Type, right: Type) -> Self
fn sub(left: Type, right: Type) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, f: &mut dyn FnMut(&'a Type))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut Type))
```

A dimension expression in a tensor shape.

Dimensions can be:
- Concrete literals: 2 and 3 in `Tensor[2, 3]`
- Symbolic expressions: N, N+1, N*M in `Tensor[N, N+1]`, `Tensor[N*M]`

---

## ShapeError

`enum` · `pyrefly_types::dimension::ShapeError`

```rust
enum ShapeError
```

**Variants**: `RankMismatch`, `InvalidDimension`, `ShapeComputation`, `StructuralMismatch`, `NestedTypeVarNotInferred`, `ScalarIndex`, `TooManyIndices`, `BadIndex`, `Unsupported`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn invalid_dimension(value: i64, reason: impl Into<String>) -> Self
fn nested_type_var_not_inferred() -> Self
fn rank_mismatch(got: usize, want: usize) -> Self
fn structural_mismatch(got: impl Into<String>, got_canonical: impl Into<String>, want: impl Into<String>, want_canonical: impl Into<String>) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Errors that can occur during shape/dimension checking

---

## canonicalize

`function` · `pyrefly_types::dimension::canonicalize`

```rust
fn canonicalize(ty: types::Type) -> types::Type
```

Canonicalize a dimension expression to a unique normal form.

This transforms dimension expressions into a canonical form where:
- Like terms are combined (e.g., 4*N + 2*N = 6*N)
- Divisions are flattened (e.g., (N // M) // K = N // (M*K))
- Factors are GCD-reduced (e.g., (4*N) // (6*M) = (2*N) // (3*M))
- Expressions are ordered consistently
- Gradual sizes propagate through the entire expression
- Arithmetic outside the representable dimension domain becomes gradual

This enables structural equality checking after canonicalization.

---

## contains_var_in_type

`function` · `pyrefly_types::dimension::contains_var_in_type`

```rust
fn contains_var_in_type(ty: &types::Type) -> bool
```

Check if a dimension type contains a solver Var anywhere in its structure.
This is used to detect when a type variable in a nested position cannot be inferred.

---

## gradual_size

`function` · `pyrefly_types::dimension::gradual_size`

```rust
fn gradual_size() -> types::Type
```

The gradual size type: the internal representation of bare `Int` and
`Int[int]`.

---

## int_is_provably_negative

`function` · `pyrefly_types::dimension::int_is_provably_negative`

```rust
fn int_is_provably_negative(dim: &Int) -> bool
```

---

## int_is_provably_nonnegative

`function` · `pyrefly_types::dimension::int_is_provably_nonnegative`

```rust
fn int_is_provably_nonnegative(dim: &Int) -> bool
```

---

## int_type_is_provably_negative

`function` · `pyrefly_types::dimension::int_type_is_provably_negative`

```rust
fn int_type_is_provably_negative(ty: &types::Type) -> bool
```

---

## int_type_is_provably_nonnegative

`function` · `pyrefly_types::dimension::int_type_is_provably_nonnegative`

```rust
fn int_type_is_provably_nonnegative(ty: &types::Type) -> bool
```

---

## is_gradual_size

`function` · `pyrefly_types::dimension::is_gradual_size`

```rust
fn is_gradual_size(ty: &types::Type) -> bool
```

Whether `ty` is the gradual size type.

---

## is_gradual_size_bound_type_var

`function` · `pyrefly_types::dimension::is_gradual_size_bound_type_var`

```rust
fn is_gradual_size_bound_type_var(ty: &types::Type) -> bool
```

Whether `ty` is an ordinary type variable whose bound is exactly the gradual size `Int`.

Shape features currently support only this broad bound. Narrower or wider bounds remain on
the ordinary type-variable path until their shape semantics are defined explicitly.

---

## is_optional_int

`function` · `pyrefly_types::dimension::is_optional_int`

```rust
fn is_optional_int(ty: &types::Type) -> bool
```

Whether `ty` is exactly the shape `Int | None` domain, in either order.

---

## is_optional_int_bound_type_var

`function` · `pyrefly_types::dimension::is_optional_int_bound_type_var`

```rust
fn is_optional_int_bound_type_var(ty: &types::Type) -> bool
```

Whether `ty` is an ordinary type variable whose resolved bound is exactly `Int | None`.

---

## type_is_gradual_fast

`function` · `pyrefly_types::dimension::type_is_gradual_fast`

```rust
fn type_is_gradual_fast(ty: &types::Type) -> bool
```

Whether a type is gradual for size purposes.

This is a sufficient pre-canonicalization check that avoids allocating on a
hot path. Canonicalization can additionally produce a gradual size when
concrete arithmetic overflows.

---
