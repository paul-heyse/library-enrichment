# `ruff_python_ast::name`

Crate `ruff_python_ast` · 4 public items · structured records in [`model/ruff_python_ast.name.json`](../model/ruff_python_ast.name.json)

## Name

`struct` · `ruff_python_ast::name::Name`

```rust
struct Name
```

**Implements**: `core::borrow::Borrow`, `core::convert::AsRef`, `core::convert::From`, `core::fmt::Display`, `core::iter::traits::collect::FromIterator`, `core::ops::deref::Deref`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (8)

```rust
fn as_str(&self) -> &str
fn concat<T: AsRef<str>>(slices: &[T]) -> Self
fn empty() -> Self
fn join<T: AsRef<str>>(slices: &[T], separator: &str) -> Self
fn new(name: impl AsRef<str>) -> Self
fn new_heap(name: impl AsRef<str>) -> Self
fn new_inline(name: impl AsRef<str>) -> Option<Self>
const fn new_static(name: &'static str) -> Self
```

**via `core::borrow::Borrow`**

```rust
fn borrow(&self) -> &str
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::convert::From`**

```rust
fn from(s: &'a String) -> Self
fn from(identifier: Identifier) -> Name
fn from(s: String) -> Self
fn from(b: Box<str>) -> Self
fn from(name: CharString) -> Self
fn from(s: &'a str) -> Self
fn from(cow: Cow<'a, str>) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

An immutable name.

# Choosing a string representation

On 64-bit targets, [`CharStr`] occupies 16 bytes and stores up to 16 UTF-8 bytes inline. Longer
values use an exactly-sized, reference-counted allocation, so cloning a heap-backed value
reuses its allocation. [`compact_str::CompactString`] occupies 24 bytes, stores up to 24 bytes
inline, and remains mutable; cloning a heap-backed value copies its contents into a new
allocation.

Prefer `CharStr` for immutable text that is retained densely or passed between owners, when
either the smaller handle or structural sharing offsets the extra heap allocations for values
between 17 and 24 bytes. Prefer `CompactString` for uniquely owned text, especially when it is
built incrementally, mutated, or commonly falls in that 17-to-24-byte range.

`Name` uses `CharStr` because names appear throughout the AST and repeated heap-backed parser
names share an allocation. By contrast, [`crate::DebugText`] uses `CompactString` because it
builds a uniquely owned buffer incrementally, and `ty_module_resolver::ModuleName` uses
`CompactString` because module names can be extended in place.

Converting a borrowed `&str` into `CharStr` creates a new value and does not preserve structural
sharing. When an API retains text already held in a `CharStr` (including a `Name`), pass or clone
the owned value rather than converting it through `&str`. This is especially relevant at Salsa
interning boundaries.

---

## QualifiedName

`struct` · `ruff_python_ast::name::QualifiedName`

```rust
struct QualifiedName<'a>
```

**Implements**: `core::fmt::Display`, `core::iter::traits::collect::FromIterator`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (8)

```rust
fn append_member(self, member: &'a str) -> Self
fn builtin(name: &'a str) -> Self
fn extend_members<T: IntoIterator<Item = &'a str>>(self, members: T) -> Self
fn from_dotted_name(name: &'a str) -> Self
fn is_unresolved_import(&self) -> bool
fn segments(&self) -> &[&'a str]
fn starts_with(&self, other: &QualifiedName<'_>) -> bool
fn user_defined(name: &'a str) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self
```

A representation of a qualified name, like `typing.List`.

---

## QualifiedNameBuilder

`struct` · `ruff_python_ast::name::QualifiedNameBuilder`

```rust
struct QualifiedNameBuilder<'a>
```

**Derives**: Clone, Debug, Default

**Methods** (4)

```rust
fn build(self) -> QualifiedName<'a>
fn extend(&mut self, segments: impl IntoIterator<Item = &'a str>)
fn push(&mut self, segment: &'a str)
fn with_capacity(capacity: usize) -> Self
```

---

## UnqualifiedName

`struct` · `ruff_python_ast::name::UnqualifiedName`

```rust
struct UnqualifiedName<'a>
```

**Implements**: `core::fmt::Display`, `core::iter::traits::collect::FromIterator`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn from_expr(expr: &'a Expr) -> Option<Self>
fn segments(&self) -> &[&'a str]
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self
```

---
