# `pyrefly_types::facet`

Crate `pyrefly_types` · 4 public items · structured records in [`model/pyrefly_types.facet.json`](../model/pyrefly_types.facet.json)

## FacetKind

`enum` · `pyrefly_types::facet::FacetKind`

```rust
enum FacetKind
```

**Variants**: `Attribute`, `Index`, `Key`

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn invalidate_on_unknown_assignment(&self) -> bool
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

The idea of "facet narrowing" is that for attribute narrowing, index narrowing,
and some other cases we maintain a tree of "facets" (things like attributes, etc)
for which we have narrowed types and we'll use these both for narrowing and for
reading along "facet chains".

For example if I write
`if x.y is not None and x.z is not None and x.y[0]["w"] is not None: ...`
then we'll wind up with two facet chains narrowed in our tree, one at
  [Attribute(y), Index(0), Key("w")]
and another at
  [Attribute(z)]

---

## UnresolvedFacetKind

`enum` · `pyrefly_types::facet::UnresolvedFacetKind`

```rust
enum UnresolvedFacetKind
```

**Variants**: `Attribute`, `Index`, `Key`, `VariableSubscript`, `MatchArg`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

---

## FacetChain

`struct` · `pyrefly_types::facet::FacetChain`

```rust
struct FacetChain
```

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn facets(&self) -> &Vec1<FacetKind>
fn new(chain: Vec1<FacetKind>) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

---

## UnresolvedFacetChain

`struct` · `pyrefly_types::facet::UnresolvedFacetChain`

```rust
struct UnresolvedFacetChain
```

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn facets(&self) -> &Vec1<UnresolvedFacetKind>
fn new(chain: Vec1<UnresolvedFacetKind>) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

---
