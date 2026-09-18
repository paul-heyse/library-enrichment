# `parquet_variant::variant::list::VariantList`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant.variant.list.VariantList.json).

<a id="op-50198737fce518dd29afb54a"></a>
## VariantList

`struct` · `parquet_variant::variant::list::VariantList` · parquet-variant 59.3.0

```rust
struct VariantList<'m, 'v>
```

Source: `src/variant/list.rs:121`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

[`Variant`](../operations/parquet_variant.variant.Variant.md#op-56d8e2cab45de055ef39f43d) Array.

See the [Variant spec] for details.

NOTE: The "list" naming differs from the variant spec -- which calls it "array" -- in order to be
consistent with Parquet and Arrow type naming. Otherwise, the name would conflict with the
`VariantArray : Array` we must eventually define for variant-typed arrow arrays.

# Validation

Every instance of variant list is either _valid_ or _invalid_. depending on whether the
underlying bytes are a valid encoding of a variant array (see below).

Instances produced by [`Self::try_new`](../operations/parquet_variant.variant.list.VariantList.md#op-9e8a8c6f0072a34092aa9544) or [`Self::with_full_validation`](../operations/parquet_variant.variant.list.VariantList.md#op-082dc702afdf4a37d099dc8d) are fully _validated_. They always
contain _valid_ data, and infallible accesses such as iteration and indexing are panic-free. The
validation cost is linear in the number of underlying bytes.

Instances produced by [`Self::new`](../operations/parquet_variant.variant.list.VariantList.md#op-bd462478ee50fb63f6eabc4e) are _unvalidated_ and so they may contain either _valid_ or
_invalid_ data. Infallible accesses such as iteration and indexing will panic if the underlying
bytes are _invalid_, and fallible alternatives such as [`Self::iter_try`](../operations/parquet_variant.variant.list.VariantList.md#op-57fc540e3f85a59691bd85d3) and [`Self::get`](../operations/parquet_variant.variant.list.VariantList.md#op-25d78e3beb6960aa905df29d) are
provided as panic-free alternatives. [`Self::with_full_validation`](../operations/parquet_variant.variant.list.VariantList.md#op-082dc702afdf4a37d099dc8d) can also be used to _validate_ an
_unvalidated_ instance, if desired.

_Unvalidated_ instances can be constructed in constant time. This can be useful if the caller
knows the underlying bytes were already validated previously, or if the caller intends to
perform a small number of (fallible) accesses to a large list.

A _validated_ variant list instance guarantees that:

- header byte is valid
- num_elements is in bounds
- offset array content is in-bounds
- first offset is zero
- last offset is in-bounds
- all other offsets are in-bounds (*)
- all offsets are monotonically increasing (*)
- all values are (recursively) valid variant objects (*)
- the associated variant metadata is [valid] (*)

NOTE: [`Self::new`](../operations/parquet_variant.variant.list.VariantList.md#op-bd462478ee50fb63f6eabc4e) only skips expensive (non-constant cost) validation checks (marked by `(*)`
in the list above); it panics any of the other checks fails.

# Safety

Even an _invalid_ variant list instance is still _safe_ to use in the Rust sense. Accessing
it with infallible methods may cause panics but will never lead to undefined behavior.

[valid]: VariantMetadata#Validation
[Variant spec]: https://github.com/apache/parquet-format/blob/master/VariantEncoding.md#value-data-for-array-basic_type3

<a id="op-8a6b69fe0cbbf9809094a1eb"></a>
## clone

`function` · `parquet_variant::variant::list::VariantList::clone` · parquet-variant 59.3.0

```rust
fn clone(&self) -> VariantList<'m, 'v>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::list::VariantList", "path": "VariantList"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 17], "end": [120, 22], "filename": "src/variant/list.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/variant/list.rs:120`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21038267540dc6ca50ad3c41"></a>
## eq

`function` · `parquet_variant::variant::list::VariantList::eq` · parquet-variant 59.3.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::list::VariantList", "path": "VariantList"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [321, 2], "filename": "src/variant/list.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/variant/list.rs:314`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-810738a0012332bade5ae87c"></a>
## fmt

`function` · `parquet_variant::variant::list::VariantList::fmt` · parquet-variant 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::list::VariantList", "path": "VariantList"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 10], "end": [120, 15], "filename": "src/variant/list.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/variant/list.rs:120`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25d78e3beb6960aa905df29d"></a>
## get

`function` · `parquet_variant::variant::list::VariantList::get` · parquet-variant 59.3.0

```rust
fn get(&self, index: usize) -> Option<Variant<'m, 'v>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::list::VariantList", "path": "VariantList"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [307, 2], "filename": "src/variant/list.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/list.rs:256`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Returns element by index in `0..self.len()`, if any. May panic if this list is [invalid].

[invalid]: Self#Validation

<a id="op-4ea3e9d1b457553f9a868370"></a>
## is_empty

`function` · `parquet_variant::variant::list::VariantList::is_empty` · parquet-variant 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::list::VariantList", "path": "VariantList"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [307, 2], "filename": "src/variant/list.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/list.rs:249`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Is the array of zero length

<a id="op-14b64c74e9343e3b69358c6e"></a>
## is_fully_validated

`function` · `parquet_variant::variant::list::VariantList::is_fully_validated` · parquet-variant 59.3.0

```rust
fn is_fully_validated(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::list::VariantList", "path": "VariantList"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [307, 2], "filename": "src/variant/list.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/list.rs:205`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

True if this instance is fully [validated] for panic-free infallible accesses.

[validated]: Self#Validation

<a id="op-41f377fd0aff474dda28b4fd"></a>
## iter

`function` · `parquet_variant::variant::list::VariantList::iter` · parquet-variant 59.3.0

```rust
fn iter(&self) -> impl Iterator<Item = Variant<'m, 'v>> + '_
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::list::VariantList", "path": "VariantList"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [307, 2], "filename": "src/variant/list.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/list.rs:283`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Iterates over the values of this list. When working with [unvalidated] input, consider
[`Self::iter_try`](../operations/parquet_variant.variant.list.VariantList.md#op-57fc540e3f85a59691bd85d3) to avoid panics due to invalid data.

[unvalidated]: Self#Validation

<a id="op-57fc540e3f85a59691bd85d3"></a>
## iter_try

`function` · `parquet_variant::variant::list::VariantList::iter_try` · parquet-variant 59.3.0

```rust
fn iter_try(&self) -> impl Iterator<Item = Result<Variant<'m, 'v>, ArrowError>> + '_
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::list::VariantList", "path": "VariantList"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [307, 2], "filename": "src/variant/list.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/list.rs:289`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Fallible iteration over the elements of this list.

<a id="op-d6c219b14c6cf6275ce9ac87"></a>
## len

`function` · `parquet_variant::variant::list::VariantList::len` · parquet-variant 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::list::VariantList", "path": "VariantList"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [307, 2], "filename": "src/variant/list.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/list.rs:244`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Return the length of this array

<a id="op-598a73e6bac33c7380ac2bb3"></a>
## metadata

`struct_field` · `parquet_variant::variant::list::VariantList::metadata` · parquet-variant 59.3.0

```rust
metadata: variant::VariantMetadata<'m>
```

Source: `src/variant/list.rs:122`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd462478ee50fb63f6eabc4e"></a>
## new

`function` · `parquet_variant::variant::list::VariantList::new` · parquet-variant 59.3.0

```rust
fn new(metadata: VariantMetadata<'m>, value: &'v [u8]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::list::VariantList", "path": "VariantList"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [307, 2], "filename": "src/variant/list.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/list.rs:148`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b94419ec19a2d35da9a8db46"></a>
## try_get

`function` · `parquet_variant::variant::list::VariantList::try_get` · parquet-variant 59.3.0

```rust
fn try_get(&self, index: usize) -> Result<Variant<'m, 'v>, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::list::VariantList", "path": "VariantList"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [307, 2], "filename": "src/variant/list.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/list.rs:264`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Fallible version of `get`. Returns element by index, capturing validation errors

<a id="op-9e8a8c6f0072a34092aa9544"></a>
## try_new

`function` · `parquet_variant::variant::list::VariantList::try_new` · parquet-variant 59.3.0

```rust
fn try_new(metadata: VariantMetadata<'m>, value: &'v [u8]) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::list::VariantList", "path": "VariantList"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [307, 2], "filename": "src/variant/list.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/list.rs:144`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Attempts to interpret `value` as a variant array value.

# Validation

This constructor verifies that `value` points to a valid variant array value. In particular,
that all offsets are in-bounds and point to valid (recursively validated) objects.

<a id="op-99684202e9b8f5340540ea56"></a>
## value

`struct_field` · `parquet_variant::variant::list::VariantList::value` · parquet-variant 59.3.0

```rust
value: &'v [u8]
```

Source: `src/variant/list.rs:123`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-082dc702afdf4a37d099dc8d"></a>
## with_full_validation

`function` · `parquet_variant::variant::list::VariantList::with_full_validation` · parquet-variant 59.3.0

```rust
fn with_full_validation(self) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::list::VariantList", "path": "VariantList"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [307, 2], "filename": "src/variant/list.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/list.rs:212`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Performs a full [validation] of this variant array and returns the result.

[validation]: Self#Validation
