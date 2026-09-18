# `parquet_variant_compute::variant_array_builder::VariantValueArrayBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant_compute.variant_array_builder.VariantValueArrayBuilder.json).

<a id="op-55740ef8e74ee4362750ad72"></a>
## VariantValueArrayBuilder

`struct` · `parquet_variant_compute::variant_array_builder::VariantValueArrayBuilder` · parquet-variant-compute 59.3.0

```rust
struct VariantValueArrayBuilder
```

Source: `src/variant_array_builder.rs:281`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

A builder for creating only the value column of a [`VariantArray`](../operations/parquet_variant_compute.variant_array.VariantArray.md#op-0730efce8ca33e866d425d18)

This builder is used when you have existing metadata and only need to build
the value column. It's useful for scenarios like variant unshredding, data
transformation, or filtering where you want to reuse existing metadata.

The builder produces a [`BinaryViewArray`](../operations/arrow_array.array.byte_view_array.BinaryViewArray.md#op-3ee392743cc781de5833990c) that can be combined with existing
metadata to create a complete [`VariantArray`](../operations/parquet_variant_compute.variant_array.VariantArray.md#op-0730efce8ca33e866d425d18).

# Example:
```
# use arrow::array::Array;
# use parquet_variant::{Variant};
# use parquet_variant_compute::VariantValueArrayBuilder;
// Create a variant value builder for 10 rows
let mut builder = VariantValueArrayBuilder::new(10);

// Append some values with their corresponding metadata, which the
// builder takes advantage of to avoid creating new metadata.
builder.append_value(Variant::from(42));
builder.append_null();
builder.append_value(Variant::from("hello"));

// Build the final value array
let value_array = builder.build().unwrap();
assert_eq!(value_array.len(), 3);
```

<a id="op-1788f5019a1ebe4f6bf9abac"></a>
## append_null

`function` · `parquet_variant_compute::variant_array_builder::VariantValueArrayBuilder::append_null` · parquet-variant-compute 59.3.0

```rust
fn append_null(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array_builder::VariantValueArrayBuilder", "path": "VariantValueArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [287, 1], "end": [392, 2], "filename": "src/variant_array_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_array_builder.rs:317`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Append a null row to the builder

WARNING: It is only valid to call this method when building the `value` field of a shredded
variant column (which is nullable). The `value` field of a binary (unshredded) variant
column is non-nullable, and callers should instead invoke [`Self::append_value`](../operations/parquet_variant_compute.variant_array_builder.VariantValueArrayBuilder.md#op-8c0df87f3da9af76937e02a6) with
`Variant::Null`, passing the appropriate metadata value.

<a id="op-8c0df87f3da9af76937e02a6"></a>
## append_value

`function` · `parquet_variant_compute::variant_array_builder::VariantValueArrayBuilder::append_value` · parquet-variant-compute 59.3.0

```rust
fn append_value(&mut self, value: Variant<'_, '_>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array_builder::VariantValueArrayBuilder", "path": "VariantValueArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [287, 1], "end": [392, 2], "filename": "src/variant_array_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_array_builder.rs:339`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Append a variant value with its corresponding metadata

# Arguments
* `value` - The variant value to append
* `metadata` - The metadata dictionary for this variant (used for field name resolution)

# Returns
* `Ok(())` if the value was successfully appended
* `Err(ArrowError)` if the variant contains field names not found in the metadata

# Example
```
# use parquet_variant::Variant;
# use parquet_variant_compute::VariantValueArrayBuilder;
let mut builder = VariantValueArrayBuilder::new(10);
builder.append_value(Variant::from(42));
```

<a id="op-262908d2292e20ee0ee41f18"></a>
## build

`function` · `parquet_variant_compute::variant_array_builder::VariantValueArrayBuilder::build` · parquet-variant-compute 59.3.0

```rust
fn build(self) -> Result<BinaryViewArray, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array_builder::VariantValueArrayBuilder", "path": "VariantValueArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [287, 1], "end": [392, 2], "filename": "src/variant_array_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_array_builder.rs:301`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Build the final value array

Returns a [`BinaryViewArray`](../operations/arrow_array.array.byte_view_array.BinaryViewArray.md#op-3ee392743cc781de5833990c) containing the serialized variant values.
This can be combined with existing metadata to create a complete [`VariantArray`](../operations/parquet_variant_compute.variant_array.VariantArray.md#op-0730efce8ca33e866d425d18).

<a id="op-6e94bd8da0c420a416716b71"></a>
## builder_ext

`function` · `parquet_variant_compute::variant_array_builder::VariantValueArrayBuilder::builder_ext` · parquet-variant-compute 59.3.0

```rust
fn builder_ext<'a>(&'a mut self, metadata: &'a VariantMetadata<'a>) -> VariantValueArrayBuilderExt<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array_builder::VariantValueArrayBuilder", "path": "VariantValueArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [287, 1], "end": [392, 2], "filename": "src/variant_array_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_array_builder.rs:383`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Creates a thin [`VariantBuilderExt`](../operations/parquet_variant.builder.VariantBuilderExt.md#op-30a3aae52156192decb8e0bb) wrapper for this builder, which hides the `metadata`
parameter (similar to the way [`parquet_variant::ObjectFieldBuilder`](../operations/parquet_variant.builder.object.ObjectFieldBuilder.md#op-f375bbc520d6d58c886b77e1) hides field names).

<a id="op-56c0220445bd1f469f3b0722"></a>
## fmt

`function` · `parquet_variant_compute::variant_array_builder::VariantValueArrayBuilder::fmt` · parquet-variant-compute 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array_builder::VariantValueArrayBuilder", "path": "VariantValueArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [280, 10], "end": [280, 15], "filename": "src/variant_array_builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/variant_array_builder.rs:280`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2855ce31b0b26dbf04bd8277"></a>
## new

`function` · `parquet_variant_compute::variant_array_builder::VariantValueArrayBuilder::new` · parquet-variant-compute 59.3.0

```rust
fn new(row_capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array_builder::VariantValueArrayBuilder", "path": "VariantValueArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [287, 1], "end": [392, 2], "filename": "src/variant_array_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_array_builder.rs:289`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Create a new `VariantValueArrayBuilder` with the specified row capacity

<a id="op-8fda1d6e1040e0fe2d721fb9"></a>
## parent_state

`function` · `parquet_variant_compute::variant_array_builder::VariantValueArrayBuilder::parent_state` · parquet-variant-compute 59.3.0

```rust
fn parent_state<'a>(&'a mut self, metadata_builder: &'a mut dyn MetadataBuilder) -> ParentState<'a, ValueArrayBuilderState<'a>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array_builder::VariantValueArrayBuilder", "path": "VariantValueArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [287, 1], "end": [392, 2], "filename": "src/variant_array_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_array_builder.rs:369`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Creates a builder-specific parent state.

For example, this can be useful for code that wants to copy a subset of fields from an
object `value` as a new row of `value_array_builder`:

```no_run
# use parquet_variant::{ObjectBuilder, ReadOnlyMetadataBuilder, Variant};
# use parquet_variant_compute::VariantValueArrayBuilder;
# let value = Variant::Null;
# let mut value_array_builder = VariantValueArrayBuilder::new(0);
# fn should_keep(field_name: &str) -> bool { todo!() };
let Variant::Object(obj) = value else {
    panic!("Not a variant object");
};
let mut metadata_builder = ReadOnlyMetadataBuilder::new(&obj.metadata);
let state = value_array_builder.parent_state(&mut metadata_builder);
let mut object_builder = ObjectBuilder::new(state, false);
for (field_name, field_value) in obj.iter() {
    if should_keep(field_name) {
        object_builder.insert_bytes(field_name, field_value);
    }
}
 object_builder.finish(); // appends the filtered object
```
