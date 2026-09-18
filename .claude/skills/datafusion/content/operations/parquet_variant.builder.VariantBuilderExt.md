# `parquet_variant::builder::VariantBuilderExt`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant.builder.VariantBuilderExt.json).

<a id="op-30a3aae52156192decb8e0bb"></a>
## VariantBuilderExt

`trait` · `parquet_variant::builder::VariantBuilderExt` · parquet-variant 59.3.0

```rust
trait VariantBuilderExt
```

Source: `src/builder.rs:992`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Extends [`VariantBuilder`](../operations/parquet_variant.builder.VariantBuilder.md#op-a9ba86b818cec580af8379d6) to help building nested [`Variant`](../operations/parquet_variant.variant.Variant.md#op-56d8e2cab45de055ef39f43d)s

Allows users to append values to a [`VariantBuilder`](../operations/parquet_variant.builder.VariantBuilder.md#op-a9ba86b818cec580af8379d6), [`ListBuilder`](../operations/parquet_variant.builder.list.ListBuilder.md#op-943a723c42ac5d70b99dbfe9) or
[`ObjectBuilder`](../operations/parquet_variant.builder.object.ObjectBuilder.md#op-93ae35f464d302abd3045ea9). using the same interface.

<a id="op-405fb5037dc1ee950c00b14d"></a>
## State

`assoc_type` · `parquet_variant::builder::VariantBuilderExt::State` · parquet-variant 59.3.0

```rust
State
```

Source: `src/builder.rs:994`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

The builder specific state used by nested builders

<a id="op-1fe4bdbfeae19cda12bc01c5"></a>
## append_null

`function` · `parquet_variant::builder::VariantBuilderExt::append_null` · parquet-variant 59.3.0

```rust
fn append_null(&mut self)
```

Source: `src/builder.rs:1000`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Appends a NULL value to this builder. The semantics depend on the implementation, but will
often translate to appending a [`Variant::Null`](../operations/parquet_variant.variant.Variant.md#op-a77e1395c0a1609bb5be4d06) value.

<a id="op-4673d376f5da96a7216a04fc"></a>
## append_value

`function` · `parquet_variant::builder::VariantBuilderExt::append_value` · parquet-variant 59.3.0

```rust
fn append_value<'m, 'v>(&mut self, value: impl Into<Variant<'m, 'v>>)
```

Source: `src/builder.rs:1003`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Appends a new variant value to this builder. See e.g. [`VariantBuilder::append_value`](../operations/parquet_variant.builder.VariantBuilder.md#op-0f5301ff0a475505e8d3a898).

<a id="op-d1ec7376aa81325af0537ce8"></a>
## new_list

`function` · `parquet_variant::builder::VariantBuilderExt::new_list` · parquet-variant 59.3.0

```rust
fn new_list(&mut self) -> ListBuilder<'_, Self::State<'_>>
```

Source: `src/builder.rs:1007`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Creates a nested list builder. See e.g. [`VariantBuilder::new_list`](../operations/parquet_variant.builder.VariantBuilder.md#op-5a36a7e52afed53c4a09ccfa). Panics if the nested
builder cannot be created, see e.g. [`ObjectBuilder::new_list`](../operations/parquet_variant.builder.object.ObjectBuilder.md#op-67161420600d4b396dc52cca).

<a id="op-7f22835d3937c1c1d00157ce"></a>
## new_object

`function` · `parquet_variant::builder::VariantBuilderExt::new_object` · parquet-variant 59.3.0

```rust
fn new_object(&mut self) -> ObjectBuilder<'_, Self::State<'_>>
```

Source: `src/builder.rs:1013`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Creates a nested object builder. See e.g. [`VariantBuilder::new_object`](../operations/parquet_variant.builder.VariantBuilder.md#op-ad6d10e9b917a72a12f25db5). Panics if the
nested builder cannot be created, see e.g. [`ObjectBuilder::new_object`](../operations/parquet_variant.builder.object.ObjectBuilder.md#op-be1a97e1fe7746db8bc84609).

<a id="op-7e537eb6861e1d1db660b1b1"></a>
## try_new_list

`function` · `parquet_variant::builder::VariantBuilderExt::try_new_list` · parquet-variant 59.3.0

```rust
fn try_new_list(&mut self) -> Result<ListBuilder<'_, Self::State<'_>>, ArrowError>
```

Source: `src/builder.rs:1019`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Creates a nested list builder. See e.g. [`VariantBuilder::new_list`](../operations/parquet_variant.builder.VariantBuilder.md#op-5a36a7e52afed53c4a09ccfa). Returns an error if
the nested builder cannot be created, see e.g. [`ObjectBuilder::try_new_list`](../operations/parquet_variant.builder.object.ObjectBuilder.md#op-efe3d36ac3f66268f6fb6f44).

<a id="op-5ccd7ef88f47c2ffd65a0f7a"></a>
## try_new_object

`function` · `parquet_variant::builder::VariantBuilderExt::try_new_object` · parquet-variant 59.3.0

```rust
fn try_new_object(&mut self) -> Result<ObjectBuilder<'_, Self::State<'_>>, ArrowError>
```

Source: `src/builder.rs:1023`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Creates a nested object builder. See e.g. [`VariantBuilder::new_object`](../operations/parquet_variant.builder.VariantBuilder.md#op-ad6d10e9b917a72a12f25db5). Returns an error
if the nested builder cannot be created, see e.g. [`ObjectBuilder::try_new_object`](../operations/parquet_variant.builder.object.ObjectBuilder.md#op-e483219d88bf879456e68621).
