# `parquet_variant::builder::object::ObjectBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant.builder.object.ObjectBuilder.json).

<a id="op-93ae35f464d302abd3045ea9"></a>
## ObjectBuilder

`struct` · `parquet_variant::builder::object::ObjectBuilder` · parquet-variant 59.3.0

```rust
struct ObjectBuilder<'a, S: BuilderSpecificState>
```

Source: `src/builder/object.rs:77`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

A builder for creating [`Variant::Object`](../operations/parquet_variant.variant.Variant.md#op-f448b5728cc2baae6632c68e) values.

See the examples on [`VariantBuilder`] for usage.

[`VariantBuilder`]: crate::VariantBuilder

<a id="op-10f838aba5b356c110ece1ad"></a>
## extend

`function` · `parquet_variant::builder::object::ObjectBuilder::extend` · parquet-variant 59.3.0

```rust
fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::object::ObjectBuilder", "path": "ObjectBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "S"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "generic_params": [], "type": {"generic": "S"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "str"}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "crate::Variant"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [327, 1], "end": [338, 2], "filename": "src/builder/object.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"generic": "K"}, {"generic": "V"}]}}], "constraints": []}}, "id": "core::iter::traits::collect::Extend", "path": "Extend"}, "trait_path": "core::iter::traits::collect::Extend"}`

Source: `src/builder/object.rs:333`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d30016208ba2fe1a4499fbbd"></a>
## finish

`function` · `parquet_variant::builder::object::ObjectBuilder::finish` · parquet-variant 59.3.0

```rust
fn finish(self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::object::ObjectBuilder", "path": "ObjectBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [325, 2], "filename": "src/builder/object.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/object.rs:257`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Finalizes this object and appends it to its parent, which otherwise remains unmodified.

<a id="op-ee45b8ab49f0cae0a427669d"></a>
## fmt

`function` · `parquet_variant::builder::object::ObjectBuilder::fmt` · parquet-variant 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::object::ObjectBuilder", "path": "ObjectBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 10], "end": [76, 15], "filename": "src/builder/object.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/builder/object.rs:76`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9302ed054ef5c7e885a3d8ea"></a>
## insert

`function` · `parquet_variant::builder::object::ObjectBuilder::insert` · parquet-variant 59.3.0

```rust
fn insert<'m, 'd, T: Into<Variant<'m, 'd>>>(&mut self, key: &str, value: T)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::object::ObjectBuilder", "path": "ObjectBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [325, 2], "filename": "src/builder/object.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/object.rs:103`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Add a field with key and value to the object

# See Also
- [`ObjectBuilder::try_insert`](../operations/parquet_variant.builder.object.ObjectBuilder.md#op-5fe2c5cb31e089974d3b32ba) for a fallible version.
- [`ObjectBuilder::with_field`](../operations/parquet_variant.builder.object.ObjectBuilder.md#op-7511f445dd2b13143e045c62) for a builder-style API.

# Panics

This method will panic if the variant contains duplicate field names in objects
when validation is enabled. For a fallible version, use [`ObjectBuilder::try_insert`](../operations/parquet_variant.builder.object.ObjectBuilder.md#op-5fe2c5cb31e089974d3b32ba)

<a id="op-ecd72bfdd554bdd17b384e17"></a>
## insert_bytes

`function` · `parquet_variant::builder::object::ObjectBuilder::insert_bytes` · parquet-variant 59.3.0

```rust
fn insert_bytes<'m, 'd>(&mut self, key: &str, value: impl Into<Variant<'m, 'd>>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::object::ObjectBuilder", "path": "ObjectBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [325, 2], "filename": "src/builder/object.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/object.rs:140`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Add a field with key and value to the object by copying raw bytes when possible.

For objects and lists, this directly copies their underlying byte representation instead of
performing a logical copy, and without touching the metadata builder. For other variant
types, this falls back to the standard append behavior.

The caller must ensure that the metadata dictionary is already built and correct for
any objects or lists being appended, but the value's new field name is handled normally.

# Panics

This method will panic if the variant contains duplicate field names in objects
when validation is enabled. For a fallible version, use [`ObjectBuilder::try_insert_bytes`](../operations/parquet_variant.builder.object.ObjectBuilder.md#op-73c41930eae475782b3f1def)

<a id="op-80db6ab0b56d836b55228a9f"></a>
## new

`function` · `parquet_variant::builder::object::ObjectBuilder::new` · parquet-variant 59.3.0

```rust
fn new(parent_state: ParentState<'a, S>, validate_unique_fields: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::object::ObjectBuilder", "path": "ObjectBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [325, 2], "filename": "src/builder/object.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/object.rs:85`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Creates a new object builder, nested on top of the given parent state.

<a id="op-67161420600d4b396dc52cca"></a>
## new_list

`function` · `parquet_variant::builder::object::ObjectBuilder::new_list` · parquet-variant 59.3.0

```rust
fn new_list<'b>(&'b mut self, key: &str) -> ListBuilder<'b, ObjectState<'b>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::object::ObjectBuilder", "path": "ObjectBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [325, 2], "filename": "src/builder/object.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/object.rs:239`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Returns a list builder that can be used to append a new (nested) list to this object.

Panics if the proposed key was a duplicate

WARNING: The builder will have no effect unless/until [`ListBuilder::finish`](../operations/parquet_variant.builder.list.ListBuilder.md#op-1ac58a0f81e82fa59ee4868e) is called.

<a id="op-be1a97e1fe7746db8bc84609"></a>
## new_object

`function` · `parquet_variant::builder::object::ObjectBuilder::new_object` · parquet-variant 59.3.0

```rust
fn new_object<'b>(&'b mut self, key: &'b str) -> ObjectBuilder<'b, ObjectState<'b>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::object::ObjectBuilder", "path": "ObjectBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [325, 2], "filename": "src/builder/object.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/object.rs:217`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Returns an object builder that can be used to append a new (nested) object to this object.

Panics if the proposed key was a duplicate

WARNING: The builder will have no effect unless/until [`ObjectBuilder::finish`](../operations/parquet_variant.builder.object.ObjectBuilder.md#op-d30016208ba2fe1a4499fbbd) is called.

<a id="op-5fe2c5cb31e089974d3b32ba"></a>
## try_insert

`function` · `parquet_variant::builder::object::ObjectBuilder::try_insert` · parquet-variant 59.3.0

```rust
fn try_insert<'m, 'd, T: Into<Variant<'m, 'd>>>(&mut self, key: &str, value: T) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::object::ObjectBuilder", "path": "ObjectBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [325, 2], "filename": "src/builder/object.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/object.rs:118`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Add a field with key and value to the object

# See Also
- [`ObjectBuilder::insert`](../operations/parquet_variant.builder.object.ObjectBuilder.md#op-9302ed054ef5c7e885a3d8ea) for an infallible version that panics
- [`ObjectBuilder::try_with_field`](../operations/parquet_variant.builder.object.ObjectBuilder.md#op-764bbfcf026a5fb5fe897087) for a builder-style API.

# Note
Attempting to insert a duplicate field name produces an error if unique field
validation is enabled. Otherwise, the new value overwrites the previous field mapping
without erasing the old value, resulting in a larger variant

<a id="op-73c41930eae475782b3f1def"></a>
## try_insert_bytes

`function` · `parquet_variant::builder::object::ObjectBuilder::try_insert_bytes` · parquet-variant 59.3.0

```rust
fn try_insert_bytes<'m, 'd>(&mut self, key: &str, value: impl Into<Variant<'m, 'd>>) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::object::ObjectBuilder", "path": "ObjectBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [325, 2], "filename": "src/builder/object.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/object.rs:156`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Add a field with key and value to the object by copying raw bytes when possible.

For objects and lists, this directly copies their underlying byte representation instead of
performing a logical copy, and without touching the metadata builder. For other variant
types, this falls back to the standard append behavior.

The caller must ensure that the metadata dictionary is already built and correct for
any objects or lists being appended, but the value's new field name is handled normally.

# Note
When inserting duplicate keys, the new value overwrites the previous mapping,
but the old value remains in the buffer, resulting in a larger variant

<a id="op-efe3d36ac3f66268f6fb6f44"></a>
## try_new_list

`function` · `parquet_variant::builder::object::ObjectBuilder::try_new_list` · parquet-variant 59.3.0

```rust
fn try_new_list<'b>(&'b mut self, key: &str) -> Result<ListBuilder<'b, ObjectState<'b>>, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::object::ObjectBuilder", "path": "ObjectBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [325, 2], "filename": "src/builder/object.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/object.rs:248`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Returns a list builder that can be used to append a new (nested) list to this object.

Fails if the proposed key was a duplicate

WARNING: The builder will have no effect unless/until [`ListBuilder::finish`](../operations/parquet_variant.builder.list.ListBuilder.md#op-1ac58a0f81e82fa59ee4868e) is called.

<a id="op-e483219d88bf879456e68621"></a>
## try_new_object

`function` · `parquet_variant::builder::object::ObjectBuilder::try_new_object` · parquet-variant 59.3.0

```rust
fn try_new_object<'b>(&'b mut self, key: &str) -> Result<ObjectBuilder<'b, ObjectState<'b>>, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::object::ObjectBuilder", "path": "ObjectBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [325, 2], "filename": "src/builder/object.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/object.rs:226`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Returns an object builder that can be used to append a new (nested) object to this object.

Fails if the proposed key was a duplicate

WARNING: The builder will have no effect unless/until [`ObjectBuilder::finish`](../operations/parquet_variant.builder.object.ObjectBuilder.md#op-d30016208ba2fe1a4499fbbd) is called.

<a id="op-764bbfcf026a5fb5fe897087"></a>
## try_with_field

`function` · `parquet_variant::builder::object::ObjectBuilder::try_with_field` · parquet-variant 59.3.0

```rust
fn try_with_field<'m, 'd, T: Into<Variant<'m, 'd>>>(self, key: &str, value: T) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::object::ObjectBuilder", "path": "ObjectBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [325, 2], "filename": "src/builder/object.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/object.rs:177`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Builder style API for adding a field with key and value to the object

Same as [`ObjectBuilder::try_insert`](../operations/parquet_variant.builder.object.ObjectBuilder.md#op-5fe2c5cb31e089974d3b32ba), but returns `self` for chaining.

<a id="op-7511f445dd2b13143e045c62"></a>
## with_field

`function` · `parquet_variant::builder::object::ObjectBuilder::with_field` · parquet-variant 59.3.0

```rust
fn with_field<'m, 'd, T: Into<Variant<'m, 'd>>>(self, key: &str, value: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::object::ObjectBuilder", "path": "ObjectBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [325, 2], "filename": "src/builder/object.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/object.rs:169`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Builder style API for adding a field with key and value to the object

Same as [`ObjectBuilder::insert`](../operations/parquet_variant.builder.object.ObjectBuilder.md#op-9302ed054ef5c7e885a3d8ea), but returns `self` for chaining.

<a id="op-cbca9da4ce1a03ef6cdeddb4"></a>
## with_validate_unique_fields

`function` · `parquet_variant::builder::object::ObjectBuilder::with_validate_unique_fields` · parquet-variant 59.3.0

```rust
fn with_validate_unique_fields(self, validate_unique_fields: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::object::ObjectBuilder", "path": "ObjectBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [325, 2], "filename": "src/builder/object.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/object.rs:190`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Enables validation for unique field keys when inserting into this object.

When this is enabled, calling [`ObjectBuilder::finish`](../operations/parquet_variant.builder.object.ObjectBuilder.md#op-d30016208ba2fe1a4499fbbd) will return an error
if any duplicate field keys were added using [`ObjectBuilder::insert`](../operations/parquet_variant.builder.object.ObjectBuilder.md#op-9302ed054ef5c7e885a3d8ea).
