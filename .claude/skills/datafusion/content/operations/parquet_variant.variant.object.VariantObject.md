# `parquet_variant::variant::object::VariantObject`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant.variant.object.VariantObject.json).

<a id="op-4002021fea3ee4472f856a19"></a>
## VariantObject

`struct` · `parquet_variant::variant::object::VariantObject` · parquet-variant 59.3.0

```rust
struct VariantObject<'m, 'v>
```

Source: `src/variant/object.rs:119`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

A [`Variant`](../operations/parquet_variant.variant.Variant.md#op-56d8e2cab45de055ef39f43d) Object (struct with named fields).

See the [Variant spec] file for more information.

# Validation

Every instance of variant object is either _valid_ or _invalid_. depending on whether the
underlying bytes are a valid encoding of a variant object subtype (see below).

Instances produced by [`Self::try_new`](../operations/parquet_variant.variant.object.VariantObject.md#op-85afaf67a9193aade95b3643) or [`Self::with_full_validation`](../operations/parquet_variant.variant.object.VariantObject.md#op-b878c25eb2e2b5ada75fc134) are fully (and recursively)
_validated_. They always contain _valid_ data, and infallible accesses such as iteration and
indexing are panic-free. The validation cost is linear in the number of underlying bytes.

Instances produced by [`Self::new`](../operations/parquet_variant.variant.object.VariantObject.md#op-95bdb5455c0bdc1212ecab54) are _unvalidated_ and so they may contain either _valid_ or
_invalid_ data. Infallible accesses such as iteration and indexing will panic if the underlying
bytes are _invalid_, and fallible alternatives such as [`Self::iter_try`](../operations/parquet_variant.variant.object.VariantObject.md#op-89f0dd39ccbf75a79e6f66f0) and [`Self::get`](../operations/parquet_variant.variant.object.VariantObject.md#op-750e05c8a1b9ba9029bcb186) are
provided as panic-free alternatives. [`Self::with_full_validation`](../operations/parquet_variant.variant.object.VariantObject.md#op-b878c25eb2e2b5ada75fc134) can also be used to _validate_ an
_unvalidated_ instance, if desired.

_Unvalidated_ instances can be constructed in constant time. They can be useful if the caller
knows the underlying bytes were already validated previously, or if the caller intends to
perform a small number of (fallible) field accesses against a large object.

A _validated_ instance guarantees that:

- header byte is valid
- num_elements is in bounds
- field id array is in bounds
- field offset array is in bounds
- field value array is in bounds
- all field ids are valid metadata dictionary entries (*)
- field ids are lexically ordered according by their corresponding string values (*)
- all field offsets are in bounds (*)
- all field values are (recursively) _valid_ variant values (*)
- the associated variant metadata is [valid] (*)

NOTE: [`Self::new`](../operations/parquet_variant.variant.object.VariantObject.md#op-95bdb5455c0bdc1212ecab54) only skips expensive (non-constant cost) validation checks (marked by `(*)`
in the list above); it panics any of the other checks fails.

# Safety

Even an _invalid_ variant object instance is still _safe_ to use in the Rust sense. Accessing it
with infallible methods may cause panics but will never lead to undefined behavior.

[valid]: VariantMetadata#Validation
[Variant spec]: https://github.com/apache/parquet-format/blob/master/VariantEncoding.md#value-data-for-object-basic_type2

<a id="op-18a785d5236e5a1479296dc8"></a>
## clone

`function` · `parquet_variant::variant::object::VariantObject::clone` · parquet-variant 59.3.0

```rust
fn clone(&self) -> VariantObject<'m, 'v>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::object::VariantObject", "path": "VariantObject"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 17], "end": [118, 22], "filename": "src/variant/object.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/variant/object.rs:118`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a72d082feaeb02aafbb07497"></a>
## eq

`function` · `parquet_variant::variant::object::VariantObject::eq` · parquet-variant 59.3.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::object::VariantObject", "path": "VariantObject"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [417, 1], "end": [430, 2], "filename": "src/variant/object.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/variant/object.rs:418`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a274460f75a17a42a041146"></a>
## field

`function` · `parquet_variant::variant::object::VariantObject::field` · parquet-variant 59.3.0

```rust
fn field(&self, i: usize) -> Option<Variant<'m, 'v>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::object::VariantObject", "path": "VariantObject"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [408, 2], "filename": "src/variant/object.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/object.rs:320`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Get a field's value by index in `0..self.len()`

# Panics

If the index is out of bounds. Also if variant object is corrupted (e.g., invalid offsets or
field IDs). The latter can only happen when working with an unvalidated object produced by
[`Self::new`](../operations/parquet_variant.variant.object.VariantObject.md#op-95bdb5455c0bdc1212ecab54).

<a id="op-2860896990f7b825d7f69856"></a>
## field_name

`function` · `parquet_variant::variant::object::VariantObject::field_name` · parquet-variant 59.3.0

```rust
fn field_name(&self, i: usize) -> Option<&'m str>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::object::VariantObject", "path": "VariantObject"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [408, 2], "filename": "src/variant/object.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/object.rs:353`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Get a field's name by index in `0..self.len()`

# Panics
If the variant object is corrupted (e.g., invalid offsets or field IDs).
This should never happen since the constructor validates all data upfront.

<a id="op-1544bb23ebcf74f588933283"></a>
## fmt

`function` · `parquet_variant::variant::object::VariantObject::fmt` · parquet-variant 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::object::VariantObject", "path": "VariantObject"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 10], "end": [118, 15], "filename": "src/variant/object.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/variant/object.rs:118`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-750e05c8a1b9ba9029bcb186"></a>
## get

`function` · `parquet_variant::variant::object::VariantObject::get` · parquet-variant 59.3.0

```rust
fn get(&self, name: &str) -> Option<Variant<'m, 'v>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::object::VariantObject", "path": "VariantObject"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [408, 2], "filename": "src/variant/object.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/object.rs:398`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Returns the value of the field with the specified name, if any.

Returns `Some(Variant)` if the field exists, or `None` if the field does not exist.

<a id="op-003d31e38fae969b90bcacbe"></a>
## is_empty

`function` · `parquet_variant::variant::object::VariantObject::is_empty` · parquet-variant 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::object::VariantObject", "path": "VariantObject"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [408, 2], "filename": "src/variant/object.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/object.rs:309`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Returns true if the object contains no key-value pairs

<a id="op-c42771b989a5e82057dffbd7"></a>
## is_fully_validated

`function` · `parquet_variant::variant::object::VariantObject::is_fully_validated` · parquet-variant 59.3.0

```rust
fn is_fully_validated(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::object::VariantObject", "path": "VariantObject"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [408, 2], "filename": "src/variant/object.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/object.rs:207`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

True if this instance is fully [validated] for panic-free infallible accesses.

[validated]: Self#Validation

<a id="op-fc31f2b4282e11189b0ff60b"></a>
## iter

`function` · `parquet_variant::variant::object::VariantObject::iter` · parquet-variant 59.3.0

```rust
fn iter(&self) -> impl Iterator<Item = (&'m str, Variant<'m, 'v>)> + '_
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::object::VariantObject", "path": "VariantObject"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [408, 2], "filename": "src/variant/object.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/object.rs:369`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Returns an iterator of (name, value) pairs over the fields of this object.

<a id="op-89f0dd39ccbf75a79e6f66f0"></a>
## iter_try

`function` · `parquet_variant::variant::object::VariantObject::iter_try` · parquet-variant 59.3.0

```rust
fn iter_try(&self) -> impl Iterator<Item = Result<(&'m str, Variant<'m, 'v>), ArrowError>> + '_
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::object::VariantObject", "path": "VariantObject"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [408, 2], "filename": "src/variant/object.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/object.rs:375`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Fallible iteration over the fields of this object.

<a id="op-4eaf7b74733472cb0ae8b134"></a>
## len

`function` · `parquet_variant::variant::object::VariantObject::len` · parquet-variant 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::object::VariantObject", "path": "VariantObject"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [408, 2], "filename": "src/variant/object.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/object.rs:304`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Returns the number of key-value pairs in this object

<a id="op-a77be7d49a2006e70bb4f2e3"></a>
## metadata

`struct_field` · `parquet_variant::variant::object::VariantObject::metadata` · parquet-variant 59.3.0

```rust
metadata: variant::VariantMetadata<'m>
```

Source: `src/variant/object.rs:120`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95bdb5455c0bdc1212ecab54"></a>
## new

`function` · `parquet_variant::variant::object::VariantObject::new` · parquet-variant 59.3.0

```rust
fn new(metadata: VariantMetadata<'m>, value: &'v [u8]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::object::VariantObject", "path": "VariantObject"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [408, 2], "filename": "src/variant/object.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/object.rs:137`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b210a10978e278a701265881"></a>
## try_field

`function` · `parquet_variant::variant::object::VariantObject::try_field` · parquet-variant 59.3.0

```rust
fn try_field(&self, i: usize) -> Result<Variant<'m, 'v>, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::object::VariantObject", "path": "VariantObject"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [408, 2], "filename": "src/variant/object.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/object.rs:328`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Fallible version of `field`. Returns field value by index, capturing validation errors

<a id="op-85afaf67a9193aade95b3643"></a>
## try_new

`function` · `parquet_variant::variant::object::VariantObject::try_new` · parquet-variant 59.3.0

```rust
fn try_new(metadata: VariantMetadata<'m>, value: &'v [u8]) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::object::VariantObject", "path": "VariantObject"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [408, 2], "filename": "src/variant/object.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/object.rs:148`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Attempts to interpet `metadata` and `value` as a variant object.

# Validation

This constructor verifies that `value` points to a valid variant object value. In
particular, that all field ids exist in `metadata`, and all offsets are in-bounds and point
to valid objects.

<a id="op-970a3afa6bdc5a5cfb99ffcd"></a>
## value

`struct_field` · `parquet_variant::variant::object::VariantObject::value` · parquet-variant 59.3.0

```rust
value: &'v [u8]
```

Source: `src/variant/object.rs:121`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b878c25eb2e2b5ada75fc134"></a>
## with_full_validation

`function` · `parquet_variant::variant::object::VariantObject::with_full_validation` · parquet-variant 59.3.0

```rust
fn with_full_validation(self) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::object::VariantObject", "path": "VariantObject"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [408, 2], "filename": "src/variant/object.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/object.rs:214`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Performs a full [validation] of this variant object.

[validation]: Self#Validation
