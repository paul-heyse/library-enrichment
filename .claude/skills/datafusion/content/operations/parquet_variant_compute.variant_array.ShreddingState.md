# `parquet_variant_compute::variant_array::ShreddingState`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant_compute.variant_array.ShreddingState.json).

<a id="op-2697d4c6c010f5bb2b7f7109"></a>
## ShreddingState

`struct` · `parquet_variant_compute::variant_array::ShreddingState` · parquet-variant-compute 59.3.0

```rust
struct ShreddingState
```

Source: `src/variant_array.rs:846`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Represents the shredding state of a [`VariantArray`](../operations/parquet_variant_compute.variant_array.VariantArray.md#op-0730efce8ca33e866d425d18)

[`VariantArray`](../operations/parquet_variant_compute.variant_array.VariantArray.md#op-0730efce8ca33e866d425d18)s can be shredded according to the [Parquet Variant
Shredding Spec]. Shredding means that the actual value is stored in a typed
`typed_field` instead of the generic `value` field.

The `value` column is always present (the spec requires writers to emit
it); `typed_value` is optional. Values in the two columns must be
interpreted according to the following table (see [Parquet Variant
Shredding Spec] for more details):

| value    | typed_value  | Meaning |
|----------|--------------|---------|
| NULL     | NULL         | The value is missing; only valid for shredded object fields |
| non-NULL | NULL         | The value is present and may be any type, including [`Variant::Null`](../operations/parquet_variant.variant.Variant.md#op-a77e1395c0a1609bb5be4d06) |
| NULL     | non-NULL     | The value is present and is the shredded type |
| non-NULL | non-NULL     | The value is present and is a partially shredded object |


Applying the above rules to entire columns, we obtain the following:

| value  | typed_value  | Meaning |
|--------|-------------|---------|
| exists | --          | **Unshredded**: If present, the value may be any type, including [`Variant::Null`](../operations/parquet_variant.variant.Variant.md#op-a77e1395c0a1609bb5be4d06)
| exists | exists      | **Shredded**: perfectly if `value` is all-null, otherwise imperfectly |

Note the spec requires the `value` column to always be present in the
schema; structs without one are rejected
(see <https://github.com/apache/arrow-rs/issues/10306>).

NOTE: Partial shredding is a row-wise situation that can arise under imperfect shredding (a
column-wise situation): When both columns exist (imperfect shredding) and the typed_value column
is a struct, then both columns can be non-NULL for the same row if value is a variant object
(partial shredding).

[Parquet Variant Shredding Spec]: https://github.com/apache/parquet-format/blob/master/VariantShredding.md#value-shredding

<a id="op-7a232ef3219ee3a81daebc5e"></a>
## Error

`assoc_type` · `parquet_variant_compute::variant_array::ShreddingState::Error` · parquet-variant-compute 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::ShreddingState", "path": "ShreddingState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [889, 1], "end": [910, 2], "filename": "src/variant_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/variant_array.rs:890`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-58693116fe78e2b125d82896"></a>
## clone

`function` · `parquet_variant_compute::variant_array::ShreddingState::clone` · parquet-variant-compute 59.3.0

```rust
fn clone(&self) -> ShreddingState
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::ShreddingState", "path": "ShreddingState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [845, 17], "end": [845, 22], "filename": "src/variant_array.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/variant_array.rs:845`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c282caaf4a6061223d937b1b"></a>
## fmt

`function` · `parquet_variant_compute::variant_array::ShreddingState::fmt` · parquet-variant-compute 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::ShreddingState", "path": "ShreddingState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [845, 10], "end": [845, 15], "filename": "src/variant_array.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/variant_array.rs:845`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09158567a0604f972e1bcfe0"></a>
## new

`function` · `parquet_variant_compute::variant_array::ShreddingState::new` · parquet-variant-compute 59.3.0

```rust
fn new(value: ArrayRef, typed_value: Option<ArrayRef>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::ShreddingState", "path": "ShreddingState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [851, 1], "end": [887, 2], "filename": "src/variant_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_array.rs:866`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Create a new `ShreddingState` from the given `value` and `typed_value` fields

Note you can create a `ShreddingState` from a &[`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207) using
`ShreddingState::try_from(&struct_array)`, for example:

```no_run
# use arrow::array::StructArray;
# use parquet_variant_compute::ShreddingState;
# fn get_struct_array() -> StructArray {
#   unimplemented!()
# }
let struct_array: StructArray = get_struct_array();
let shredding_state = ShreddingState::try_from(&struct_array).unwrap();
```

<a id="op-ac7869b0a6c233f10bfa434e"></a>
## slice

`function` · `parquet_variant_compute::variant_array::ShreddingState::slice` · parquet-variant-compute 59.3.0

```rust
fn slice(&self, offset: usize, length: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::ShreddingState", "path": "ShreddingState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [851, 1], "end": [887, 2], "filename": "src/variant_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_array.rs:881`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Slice all the underlying arrays

<a id="op-7eb88594d8d37b0faf4c36cf"></a>
## try_from

`function` · `parquet_variant_compute::variant_array::ShreddingState::try_from` · parquet-variant-compute 59.3.0

```rust
fn try_from(inner_struct: &StructArray) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::ShreddingState", "path": "ShreddingState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [889, 1], "end": [910, 2], "filename": "src/variant_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/variant_array.rs:892`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3916ea15abf7bc717ae7df97"></a>
## typed_value_column

`function` · `parquet_variant_compute::variant_array::ShreddingState::typed_value_column` · parquet-variant-compute 59.3.0

```rust
fn typed_value_column(&self) -> Option<&ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::ShreddingState", "path": "ShreddingState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [851, 1], "end": [887, 2], "filename": "src/variant_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_array.rs:876`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Return a reference to the `typed_value` column, if present

<a id="op-5407f5031b0e644009b62159"></a>
## value_column

`function` · `parquet_variant_compute::variant_array::ShreddingState::value_column` · parquet-variant-compute 59.3.0

```rust
fn value_column(&self) -> &ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::ShreddingState", "path": "ShreddingState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [851, 1], "end": [887, 2], "filename": "src/variant_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_array.rs:871`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Return a reference to the `value` column
