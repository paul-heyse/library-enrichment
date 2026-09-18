# `arrow_array::scalar::Scalar`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.scalar.Scalar.json).

<a id="op-0ca10f05b262b3afc7077257"></a>
## Scalar

`struct` · `arrow_array::scalar::Scalar` · arrow-array 59.3.0

```rust
struct Scalar<T: Array>
```

Source: `src/scalar.rs:128`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A wrapper around a single value [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21) that implements
[`Datum`](../operations/arrow_array.scalar.Datum.md#op-8fd206fb21d47e60f8e9716f) and indicates [compute] kernels should treat this array
as a scalar value (a single value).

Using a [`Scalar`](../operations/arrow_array.scalar.Scalar.md#op-0ca10f05b262b3afc7077257) is often much more efficient than creating an
[`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21) with the same (repeated) value.

See [`Datum`](../operations/arrow_array.scalar.Datum.md#op-8fd206fb21d47e60f8e9716f) for more information.

# Example

```rust
# use arrow_array::{Scalar, Int32Array, ArrayRef};
# fn get_array() -> ArrayRef { std::sync::Arc::new(Int32Array::from(vec![42])) }
// Create a (typed) scalar for Int32Array for the value 42
let scalar = Scalar::new(Int32Array::from(vec![42]));

// Create a scalar using PrimtiveArray::scalar
let scalar = Int32Array::new_scalar(42);

// create a scalar from an ArrayRef (for dynamic typed Arrays)
let array: ArrayRef = get_array();
let scalar = Scalar::new(array);
```

[compute]: https://docs.rs/arrow/latest/arrow/compute/index.html

<a id="op-0477a06db8f641f929348b8c"></a>
## clone

`function` · `arrow_array::scalar::Scalar::clone` · arrow-array 59.3.0

```rust
fn clone(&self) -> Scalar<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::scalar::Scalar", "path": "Scalar"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 23], "end": [127, 28], "filename": "src/scalar.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/scalar.rs:127`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-231cf620733f806d95a0974e"></a>
## fmt

`function` · `arrow_array::scalar::Scalar::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::scalar::Scalar", "path": "Scalar"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 10], "end": [127, 15], "filename": "src/scalar.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/scalar.rs:127`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3fbb3b73990690beb7ec26f5"></a>
## get

`function` · `arrow_array::scalar::Scalar::get` · arrow-array 59.3.0

```rust
fn get(&self) -> (&dyn Array, bool)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::scalar::Scalar", "path": "Scalar"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [148, 1], "end": [152, 2], "filename": "src/scalar.rs"}, "trait": {"args": null, "id": "arrow_array::scalar::Datum", "path": "Datum"}, "trait_path": "arrow_array::scalar::Datum"}`

Source: `src/scalar.rs:149`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf37cee8bef4cadec51532e3"></a>
## into_inner

`function` · `arrow_array::scalar::Scalar::into_inner` · arrow-array 59.3.0

```rust
fn into_inner(self) -> T
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::scalar::Scalar", "path": "Scalar"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [146, 2], "filename": "src/scalar.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar.rs:143`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the inner array

<a id="op-d643d6ff86d621ce6cbdb9a0"></a>
## new

`function` · `arrow_array::scalar::Scalar::new` · arrow-array 59.3.0

```rust
fn new(array: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::scalar::Scalar", "path": "Scalar"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [146, 2], "filename": "src/scalar.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar.rs:136`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`Scalar`](../operations/arrow_array.scalar.Scalar.md#op-0ca10f05b262b3afc7077257) from an [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21)

# Panics

Panics if `array.len() != 1`
