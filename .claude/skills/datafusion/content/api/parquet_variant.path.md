# `parquet_variant::path`

Crate `parquet-variant` · 2 public items · structured records in [`model/parquet_variant.path.json`](../model/parquet_variant.path.json)

## VariantPathElement

`enum` · `parquet_variant::path::VariantPathElement`

Also reachable as `parquet::variant::VariantPathElement`, `parquet_variant::VariantPathElement`

```rust
enum VariantPathElement<'a>
```

**Variants**: `Field`, `Index`

**Implements**: `core::convert::From`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn field(name: impl Into<Cow<'a, str>>) -> VariantPathElement<'a>
fn index(index: usize) -> VariantPathElement<'a>
```

**via `core::convert::From`**

```rust
fn from(index: usize) -> Self
fn from(name: String) -> Self
fn from(name: Cow<'a, str>) -> Self
fn from(name: &'a String) -> Self
fn from(name: &'a str) -> Self
```

[Full member, field, variant and typed contracts](../operations/parquet_variant.path.VariantPathElement.md).


Element of a [`VariantPath`] that can be a field name or an index.

See [`VariantPath`] for more details and examples.

---

## VariantPath

`struct` · `parquet_variant::path::VariantPath`

Also reachable as `parquet::variant::VariantPath`, `parquet_variant::VariantPath`

```rust
struct VariantPath<'a>
```

**Implements**: `core::convert::From`, `core::convert::TryFrom`, `core::iter::traits::collect::FromIterator`, `core::ops::deref::Deref`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn is_empty(&self) -> bool
fn join(self, element: impl Into<VariantPathElement<'a>>) -> Self
fn new(path: Vec<VariantPathElement<'a>>) -> Self
fn path(&self) -> &Vec<VariantPathElement<'_>>
fn push(&mut self, element: impl Into<VariantPathElement<'a>>)
```

**via `core::convert::From`**

```rust
fn from(index: usize) -> Self
fn from(value: Vec<VariantPathElement<'a>>) -> Self
fn from(elements: &[VariantPathElement<'a>]) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(path: &'a str) -> Result<Self, Self::Error>
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: IntoIterator<Item = VariantPathElement<'a>>>(iter: T) -> Self
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

[Full member, field, variant and typed contracts](../operations/parquet_variant.path.VariantPath.md).


Represents a qualified path to a potential subfield or index of a variant
value.

Can be used with [`Variant::get_path`] to retrieve a specific subfield of
a variant value.

[`Variant::get_path`]: crate::Variant::get_path

Create a [`VariantPath`] from a vector of [`VariantPathElement`], or
from a single field name or index.

# Example: Simple paths
```rust
# use parquet_variant::{VariantPath, VariantPathElement};
// access the field "foo" in a variant object value
let path = VariantPath::try_from("foo").unwrap();
// access the first element in a variant list vale
let path = VariantPath::from(0);
```

# Example: Compound paths
```
# use parquet_variant::{VariantPath, VariantPathElement};
/// You can also create a path by joining elements together:
// access the field "foo" and then the first element in a variant list value
let path = VariantPath::try_from("foo").unwrap().join(0);
// this is the same as the previous one
let path2 = VariantPath::from_iter(["foo".into(), 0.into()]);
assert_eq!(path, path2);
// you can also create a path from a vector of `VariantPathElement` directly
let path3 = [
  VariantPathElement::field("foo"),
  VariantPathElement::index(0)
].into_iter().collect::<VariantPath>();
assert_eq!(path, path3);
```

# Example: From Dot notation strings
```
# use parquet_variant::{VariantPath, VariantPathElement};
/// You can also convert strings directly into paths using dot notation
let path = VariantPath::try_from("foo.bar.baz").unwrap();
let expected = VariantPath::try_from("foo").unwrap().join("bar").join("baz");
assert_eq!(path, expected);
```

# Example: Accessing Compound paths
```
# use parquet_variant::{VariantPath, VariantPathElement};
/// You can access the paths using slices
// access the field "foo" and then the first element in a variant list value
let path = VariantPath::try_from("foo").unwrap()
  .join("bar")
  .join("baz");
assert_eq!(path[1], VariantPathElement::field("bar"));
```

# Example: Accessing field with bracket
```
# use parquet_variant::{VariantPath, VariantPathElement};
let path = VariantPath::try_from("a['b.c'].d[2]['3']").unwrap();
let expected = VariantPath::from_iter([VariantPathElement::field("a"),
    VariantPathElement::field("b.c"),
    VariantPathElement::field("d"),
    VariantPathElement::index(2),
    VariantPathElement::field("3")]);
assert_eq!(path, expected)

---
