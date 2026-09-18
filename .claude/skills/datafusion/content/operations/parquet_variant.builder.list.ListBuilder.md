# `parquet_variant::builder::list::ListBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant.builder.list.ListBuilder.json).

<a id="op-943a723c42ac5d70b99dbfe9"></a>
## ListBuilder

`struct` · `parquet_variant::builder::list::ListBuilder` · parquet-variant 59.3.0

```rust
struct ListBuilder<'a, S: BuilderSpecificState>
```

Source: `src/builder/list.rs:46`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

A builder for creating [`Variant::List`](../operations/parquet_variant.variant.Variant.md#op-9ee3bf99ad7b6967fe1dd651) values.

See the examples on [`VariantBuilder`] for usage.

[`VariantBuilder`]: crate::VariantBuilder

<a id="op-97ed2cfac877a6754562453e"></a>
## State

`assoc_type` · `parquet_variant::builder::list::ListBuilder::State` · parquet-variant 59.3.0

```rust
State
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::list::ListBuilder", "path": "ListBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [217, 2], "filename": "src/builder/list.rs"}, "trait": {"args": null, "id": "parquet_variant::builder::VariantBuilderExt", "path": "VariantBuilderExt"}, "trait_path": "parquet_variant::builder::VariantBuilderExt"}`

Source: `src/builder/list.rs:197`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f93c166d7ecd040c4ca459a"></a>
## append_null

`function` · `parquet_variant::builder::list::ListBuilder::append_null` · parquet-variant 59.3.0

```rust
fn append_null(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::list::ListBuilder", "path": "ListBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [217, 2], "filename": "src/builder/list.rs"}, "trait": {"args": null, "id": "parquet_variant::builder::VariantBuilderExt", "path": "VariantBuilderExt"}, "trait_path": "parquet_variant::builder::VariantBuilderExt"}`

Source: `src/builder/list.rs:203`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Variant arrays cannot encode NULL values, only `Variant::Null`.

<a id="op-07c328d51b0770c71ba2134b"></a>
## append_value

`function` · `parquet_variant::builder::list::ListBuilder::append_value` · parquet-variant 59.3.0

```rust
fn append_value<'m, 'v>(&mut self, value: impl Into<Variant<'m, 'v>>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::list::ListBuilder", "path": "ListBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [217, 2], "filename": "src/builder/list.rs"}, "trait": {"args": null, "id": "parquet_variant::builder::VariantBuilderExt", "path": "VariantBuilderExt"}, "trait_path": "parquet_variant::builder::VariantBuilderExt"}`

Source: `src/builder/list.rs:206`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ea921a83902deb9a4785198"></a>
## append_value

`function` · `parquet_variant::builder::list::ListBuilder::append_value` · parquet-variant 59.3.0

```rust
fn append_value<'m, 'd, T: Into<Variant<'m, 'd>>>(&mut self, value: T)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::list::ListBuilder", "path": "ListBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [194, 2], "filename": "src/builder/list.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/list.rs:104`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Appends a variant to the list.

# Panics

This method will panic if the variant contains duplicate field names in objects
when validation is enabled. For a fallible version, use [`ListBuilder::try_append_value`](../operations/parquet_variant.builder.list.ListBuilder.md#op-25fa4e328b55c1cfce1d9e43).

<a id="op-51afedd5d8a0cdac96176bd6"></a>
## append_value_bytes

`function` · `parquet_variant::builder::list::ListBuilder::append_value_bytes` · parquet-variant 59.3.0

```rust
fn append_value_bytes<'m, 'd>(&mut self, value: impl Into<Variant<'m, 'd>>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::list::ListBuilder", "path": "ListBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [194, 2], "filename": "src/builder/list.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/list.rs:126`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Appends a variant value to this list by copying raw bytes when possible.

For objects and lists, this directly copies their underlying byte representation instead of
performing a logical copy. For other variant types, this falls back to the standard append
behavior.

The caller must ensure that the metadata dictionary is already built and correct for
any objects or lists being appended.

<a id="op-58247461ff979499864ef21c"></a>
## extend

`function` · `parquet_variant::builder::list::ListBuilder::extend` · parquet-variant 59.3.0

```rust
fn extend<T: IntoIterator<Item = V>>(&mut self, iter: T)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::list::ListBuilder", "path": "ListBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "S"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "generic_params": [], "type": {"generic": "S"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "crate::Variant"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [219, 1], "end": [229, 2], "filename": "src/builder/list.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "V"}}], "constraints": []}}, "id": "core::iter::traits::collect::Extend", "path": "Extend"}, "trait_path": "core::iter::traits::collect::Extend"}`

Source: `src/builder/list.rs:224`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ac58a0f81e82fa59ee4868e"></a>
## finish

`function` · `parquet_variant::builder::list::ListBuilder::finish` · parquet-variant 59.3.0

```rust
fn finish(self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::list::ListBuilder", "path": "ListBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [194, 2], "filename": "src/builder/list.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/list.rs:154`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Finalizes this list and appends it to its parent, which otherwise remains unmodified.

<a id="op-1c9668e1be92154ce7067704"></a>
## fmt

`function` · `parquet_variant::builder::list::ListBuilder::fmt` · parquet-variant 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::list::ListBuilder", "path": "ListBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 10], "end": [45, 15], "filename": "src/builder/list.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/builder/list.rs:45`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a08cd969ec69669e338bd86"></a>
## new

`function` · `parquet_variant::builder::list::ListBuilder::new` · parquet-variant 59.3.0

```rust
fn new(parent_state: ParentState<'a, S>, validate_unique_fields: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::list::ListBuilder", "path": "ListBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [194, 2], "filename": "src/builder/list.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/list.rs:54`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Creates a new list builder, nested on top of the given parent state.

<a id="op-21b1d459408705c275d92d90"></a>
## new_list

`function` · `parquet_variant::builder::list::ListBuilder::new_list` · parquet-variant 59.3.0

```rust
fn new_list(&mut self) -> ListBuilder<'_, ListState<'_>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::list::ListBuilder", "path": "ListBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [194, 2], "filename": "src/builder/list.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/list.rs:93`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Returns a list builder that can be used to append a new (nested) list to this list.

WARNING: The builder will have no effect unless/until [`ListBuilder::finish`](../operations/parquet_variant.builder.list.ListBuilder.md#op-1ac58a0f81e82fa59ee4868e) is called.

<a id="op-5059f1247abd40ea502eda47"></a>
## new_object

`function` · `parquet_variant::builder::list::ListBuilder::new_object` · parquet-variant 59.3.0

```rust
fn new_object(&mut self) -> ObjectBuilder<'_, ListState<'_>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::list::ListBuilder", "path": "ListBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [194, 2], "filename": "src/builder/list.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/list.rs:85`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Returns an object builder that can be used to append a new (nested) object to this list.

WARNING: The builder will have no effect unless/until [`ObjectBuilder::finish`](../operations/parquet_variant.builder.object.ObjectBuilder.md#op-d30016208ba2fe1a4499fbbd) is called.

<a id="op-25fa4e328b55c1cfce1d9e43"></a>
## try_append_value

`function` · `parquet_variant::builder::list::ListBuilder::try_append_value` · parquet-variant 59.3.0

```rust
fn try_append_value<'m, 'd, T: Into<Variant<'m, 'd>>>(&mut self, value: T) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::list::ListBuilder", "path": "ListBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [194, 2], "filename": "src/builder/list.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/list.rs:110`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Appends a new primitive value to this list

<a id="op-af65a71dea71288595a50403"></a>
## try_new_list

`function` · `parquet_variant::builder::list::ListBuilder::try_new_list` · parquet-variant 59.3.0

```rust
fn try_new_list(&mut self) -> Result<ListBuilder<'_, Self::State<'_>>, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::list::ListBuilder", "path": "ListBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [217, 2], "filename": "src/builder/list.rs"}, "trait": {"args": null, "id": "parquet_variant::builder::VariantBuilderExt", "path": "VariantBuilderExt"}, "trait_path": "parquet_variant::builder::VariantBuilderExt"}`

Source: `src/builder/list.rs:210`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9f1ebdec99bc1dfd59bc039"></a>
## try_new_object

`function` · `parquet_variant::builder::list::ListBuilder::try_new_object` · parquet-variant 59.3.0

```rust
fn try_new_object(&mut self) -> Result<ObjectBuilder<'_, Self::State<'_>>, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::list::ListBuilder", "path": "ListBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [217, 2], "filename": "src/builder/list.rs"}, "trait": {"args": null, "id": "parquet_variant::builder::VariantBuilderExt", "path": "VariantBuilderExt"}, "trait_path": "parquet_variant::builder::VariantBuilderExt"}`

Source: `src/builder/list.rs:214`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de55037219b03aa413a9ab45"></a>
## try_with_value

`function` · `parquet_variant::builder::list::ListBuilder::try_with_value` · parquet-variant 59.3.0

```rust
fn try_with_value<'m, 'd, T: Into<Variant<'m, 'd>>>(self, value: T) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::list::ListBuilder", "path": "ListBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [194, 2], "filename": "src/builder/list.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/list.rs:145`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Builder-style API for appending a value to the list and returns self for method chaining.

This is the fallible version of [`ListBuilder::with_value`](../operations/parquet_variant.builder.list.ListBuilder.md#op-61c616714723e8dfa453f708).

<a id="op-853a839260212ed05cd643c5"></a>
## with_validate_unique_fields

`function` · `parquet_variant::builder::list::ListBuilder::with_validate_unique_fields` · parquet-variant 59.3.0

```rust
fn with_validate_unique_fields(self, validate_unique_fields: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::list::ListBuilder", "path": "ListBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [194, 2], "filename": "src/builder/list.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/list.rs:66`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Enables unique field key validation for objects created within this list.

Propagates the validation flag to any [`ObjectBuilder`](../operations/parquet_variant.builder.object.ObjectBuilder.md#op-93ae35f464d302abd3045ea9)s created using
[`ListBuilder::new_object`](../operations/parquet_variant.builder.list.ListBuilder.md#op-5059f1247abd40ea502eda47).

<a id="op-61c616714723e8dfa453f708"></a>
## with_value

`function` · `parquet_variant::builder::list::ListBuilder::with_value` · parquet-variant 59.3.0

```rust
fn with_value<'m, 'd, T: Into<Variant<'m, 'd>>>(self, value: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::list::ListBuilder", "path": "ListBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [194, 2], "filename": "src/builder/list.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/list.rs:137`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Builder-style API for appending a value to the list and returning self to enable method chaining.

# Panics

This method will panic if the variant contains duplicate field names in objects
when validation is enabled. For a fallible version, use [`ListBuilder::try_with_value`](../operations/parquet_variant.builder.list.ListBuilder.md#op-de55037219b03aa413a9ab45).
