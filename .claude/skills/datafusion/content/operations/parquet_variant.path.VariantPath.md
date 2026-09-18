# `parquet_variant::path::VariantPath`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant.path.VariantPath.json).

<a id="op-80e1a932cd6b1f8c63058dd5"></a>
## VariantPath

`struct` · `parquet_variant::path::VariantPath` · parquet-variant 59.3.0

```rust
struct VariantPath<'a>
```

Source: `src/path.rs:89`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Represents a qualified path to a potential subfield or index of a variant
value.

Can be used with [`Variant::get_path`] to retrieve a specific subfield of
a variant value.

[`Variant::get_path`]: crate::Variant::get_path

Create a [`VariantPath`](../operations/parquet_variant.path.VariantPath.md#op-80e1a932cd6b1f8c63058dd5) from a vector of [`VariantPathElement`](../operations/parquet_variant.path.VariantPathElement.md#op-740ae15d16b7e2533737aec4), or
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

<a id="op-36da3603f7374756f66b8285"></a>
## Error

`assoc_type` · `parquet_variant::path::VariantPath::Error` · parquet-variant 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::path::VariantPath", "path": "VariantPath"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [126, 1], "end": [132, 2], "filename": "src/path.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/path.rs:127`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33baa0c0c1a44efa47d8b1b1"></a>
## Target

`assoc_type` · `parquet_variant::path::VariantPath::Target` · parquet-variant 59.3.0

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::path::VariantPath", "path": "VariantPath"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 1], "end": [160, 2], "filename": "src/path.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/path.rs:155`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d781faf2aea56bcefe5a84b"></a>
## clone

`function` · `parquet_variant::path::VariantPath::clone` · parquet-variant 59.3.0

```rust
fn clone(&self) -> VariantPath<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::path::VariantPath", "path": "VariantPath"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 17], "end": [88, 22], "filename": "src/path.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/path.rs:88`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a6690451d0664b270eef37be"></a>
## default

`function` · `parquet_variant::path::VariantPath::default` · parquet-variant 59.3.0

```rust
fn default() -> VariantPath<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::path::VariantPath", "path": "VariantPath"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 35], "end": [88, 42], "filename": "src/path.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/path.rs:88`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-089c2f007174521cc77effbe"></a>
## deref

`function` · `parquet_variant::path::VariantPath::deref` · parquet-variant 59.3.0

```rust
fn deref(&self) -> &Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::path::VariantPath", "path": "VariantPath"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 1], "end": [160, 2], "filename": "src/path.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/path.rs:157`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3df88c15bd095b91a02cc2c"></a>
## eq

`function` · `parquet_variant::path::VariantPath::eq` · parquet-variant 59.3.0

```rust
fn eq(&self, other: &VariantPath<'a>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::path::VariantPath", "path": "VariantPath"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 24], "end": [88, 33], "filename": "src/path.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/path.rs:88`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef7c1cfae8ee9c7ae617a1be"></a>
## fmt

`function` · `parquet_variant::path::VariantPath::fmt` · parquet-variant 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::path::VariantPath", "path": "VariantPath"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 10], "end": [88, 15], "filename": "src/path.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/path.rs:88`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1592b2ac22f3dec2163888ea"></a>
## from

`function` · `parquet_variant::path::VariantPath::from` · parquet-variant 59.3.0

```rust
fn from(elements: &[VariantPathElement<'a>]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::path::VariantPath", "path": "VariantPath"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [145, 2], "filename": "src/path.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"slice": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::path::VariantPathElement", "path": "VariantPathElement"}}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/path.rs:142`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48f5dbc85c33bb61caf781d2"></a>
## from

`function` · `parquet_variant::path::VariantPath::from` · parquet-variant 59.3.0

```rust
fn from(value: Vec<VariantPathElement<'a>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::path::VariantPath", "path": "VariantPath"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 1], "end": [123, 2], "filename": "src/path.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::path::VariantPathElement", "path": "VariantPathElement"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/path.rs:120`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d7d19c830d544d8bd33e23d"></a>
## from

`function` · `parquet_variant::path::VariantPath::from` · parquet-variant 59.3.0

```rust
fn from(index: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::path::VariantPath", "path": "VariantPath"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 1], "end": [139, 2], "filename": "src/path.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "usize"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/path.rs:136`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c78c9c2719ec065c180b3430"></a>
## from_iter

`function` · `parquet_variant::path::VariantPath::from_iter` · parquet-variant 59.3.0

```rust
fn from_iter<T: IntoIterator<Item = VariantPathElement<'a>>>(iter: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::path::VariantPath", "path": "VariantPath"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [148, 1], "end": [152, 2], "filename": "src/path.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::path::VariantPathElement", "path": "VariantPathElement"}}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/path.rs:149`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0124b7cea1fd2a9730b9df67"></a>
## is_empty

`function` · `parquet_variant::path::VariantPath::is_empty` · parquet-variant 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::path::VariantPath", "path": "VariantPath"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [117, 2], "filename": "src/path.rs"}, "trait": null, "trait_path": null}`

Source: `src/path.rs:114`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Returns whether [`VariantPath`](../operations/parquet_variant.path.VariantPath.md#op-80e1a932cd6b1f8c63058dd5) has no path elements

<a id="op-4481381571a8626559df01b5"></a>
## join

`function` · `parquet_variant::path::VariantPath::join` · parquet-variant 59.3.0

```rust
fn join(self, element: impl Into<VariantPathElement<'a>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::path::VariantPath", "path": "VariantPath"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [117, 2], "filename": "src/path.rs"}, "trait": null, "trait_path": null}`

Source: `src/path.rs:103`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Return a new `VariantPath` with element appended

<a id="op-7fda27cf83328b4dde8591a6"></a>
## new

`function` · `parquet_variant::path::VariantPath::new` · parquet-variant 59.3.0

```rust
fn new(path: Vec<VariantPathElement<'a>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::path::VariantPath", "path": "VariantPath"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [117, 2], "filename": "src/path.rs"}, "trait": null, "trait_path": null}`

Source: `src/path.rs:93`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Create a new `VariantPath` from a vector of `VariantPathElement`.

<a id="op-a0d25a89c8566b4b17209ba8"></a>
## path

`function` · `parquet_variant::path::VariantPath::path` · parquet-variant 59.3.0

```rust
fn path(&self) -> &Vec<VariantPathElement<'_>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::path::VariantPath", "path": "VariantPath"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [117, 2], "filename": "src/path.rs"}, "trait": null, "trait_path": null}`

Source: `src/path.rs:98`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Return the inner path elements.

<a id="op-3b2601a66cc3640f76765a8d"></a>
## push

`function` · `parquet_variant::path::VariantPath::push` · parquet-variant 59.3.0

```rust
fn push(&mut self, element: impl Into<VariantPathElement<'a>>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::path::VariantPath", "path": "VariantPath"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [117, 2], "filename": "src/path.rs"}, "trait": null, "trait_path": null}`

Source: `src/path.rs:109`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Append a new element to the path

<a id="op-224512c7f68ed22122645933"></a>
## try_from

`function` · `parquet_variant::path::VariantPath::try_from` · parquet-variant 59.3.0

```rust
fn try_from(path: &'a str) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::path::VariantPath", "path": "VariantPath"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [126, 1], "end": [132, 2], "filename": "src/path.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/path.rs:129`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
