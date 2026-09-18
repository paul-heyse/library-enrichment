# `arrow_select::zip::ScalarZipper`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_select.zip.ScalarZipper.json).

<a id="op-31bab752c7d1d7fa69754363"></a>
## ScalarZipper

`struct` · `arrow_select::zip::ScalarZipper` · arrow-select 59.3.0

```rust
struct ScalarZipper
```

Source: `src/zip.rs:225`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Zipper for 2 scalars

Useful for using in `IF <expr> THEN <scalar> ELSE <scalar> END` expressions

# Example
```
# use std::sync::Arc;
# use arrow_array::{ArrayRef, BooleanArray, Int32Array, Scalar, cast::AsArray, types::Int32Type};

# use arrow_select::zip::ScalarZipper;
let scalar_truthy = Scalar::new(Int32Array::from_value(42, 1));
let scalar_falsy = Scalar::new(Int32Array::from_value(123, 1));
let zipper = ScalarZipper::try_new(&scalar_truthy, &scalar_falsy).unwrap();

// Later when we have a boolean mask
let mask = BooleanArray::from(vec![true, false, true, false, true]);
let result = zipper.zip(&mask).unwrap();
let actual = result.as_primitive::<Int32Type>();
let expected = Int32Array::from(vec![Some(42), Some(123), Some(42), Some(123), Some(42)]);
```


<a id="op-77695cfe5085e400d7fe0718"></a>
## clone

`function` · `arrow_select::zip::ScalarZipper::clone` · arrow-select 59.3.0

```rust
fn clone(&self) -> ScalarZipper
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_select::zip::ScalarZipper", "path": "ScalarZipper"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [224, 17], "end": [224, 22], "filename": "src/zip.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/zip.rs:224`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb52917fbe67758f9db4f991"></a>
## fmt

`function` · `arrow_select::zip::ScalarZipper::fmt` · arrow-select 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_select::zip::ScalarZipper", "path": "ScalarZipper"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [224, 10], "end": [224, 15], "filename": "src/zip.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/zip.rs:224`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4fbabc14082ed42ad36f5e3"></a>
## try_new

`function` · `arrow_select::zip::ScalarZipper::try_new` · arrow-select 59.3.0

```rust
fn try_new(truthy: &dyn Datum, falsy: &dyn Datum) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_select::zip::ScalarZipper", "path": "ScalarZipper"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 1], "end": [309, 2], "filename": "src/zip.rs"}, "trait": null, "trait_path": null}`

Source: `src/zip.rs:237`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Try to create a new ScalarZipper from two scalar Datum

# Errors
returns error if:
- the two Datum have different data types
- either Datum is not a scalar (or has more than 1 element)


<a id="op-fc821da886be98af3121eb4e"></a>
## zip

`function` · `arrow_select::zip::ScalarZipper::zip` · arrow-select 59.3.0

```rust
fn zip(&self, mask: &BooleanArray) -> Result<ArrayRef, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_select::zip::ScalarZipper", "path": "ScalarZipper"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 1], "end": [309, 2], "filename": "src/zip.rs"}, "trait": null, "trait_path": null}`

Source: `src/zip.rs:306`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Creating output array based on input boolean array and the two scalar values the zipper was created with
See struct level documentation for examples.
