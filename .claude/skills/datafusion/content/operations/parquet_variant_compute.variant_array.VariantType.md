# `parquet_variant_compute::variant_array::VariantType`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant_compute.variant_array.VariantType.json).

<a id="op-88d7342856958a338cda9720"></a>
## VariantType

`struct` · `parquet_variant_compute::variant_array::VariantType` · parquet-variant-compute 59.3.0

```rust
struct VariantType
```

Source: `src/variant_array.rs:90`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Arrow Variant [`ExtensionType`](../operations/arrow_schema.extension.ExtensionType.md#op-efe5f6895e277f3bfd68ba9f).

Represents the canonical Arrow Extension Type for storing variants.
See [`VariantArray`](../operations/parquet_variant_compute.variant_array.VariantArray.md#op-0730efce8ca33e866d425d18) for more examples of using this extension type.

<a id="op-8d66e917297e423ab937a88b"></a>
## Metadata

`assoc_type` · `parquet_variant_compute::variant_array::VariantType::Metadata` · parquet-variant-compute 59.3.0

```rust
Metadata
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::VariantType", "path": "VariantType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [129, 2], "filename": "src/variant_array.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/variant_array.rs:97`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-42bd20230d3055576d1633ea"></a>
## NAME

`assoc_const` · `parquet_variant_compute::variant_array::VariantType::NAME` · parquet-variant-compute 59.3.0

```rust
NAME
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::VariantType", "path": "VariantType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [129, 2], "filename": "src/variant_array.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/variant_array.rs:93`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef2b2200256f417eafa6038e"></a>
## deserialize_metadata

`function` · `parquet_variant_compute::variant_array::VariantType::deserialize_metadata` · parquet-variant-compute 59.3.0

```rust
fn deserialize_metadata(_metadata: Option<&str>) -> Result<Self::Metadata>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::VariantType", "path": "VariantType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [129, 2], "filename": "src/variant_array.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/variant_array.rs:107`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3027d636fdc272d499ae063"></a>
## metadata

`function` · `parquet_variant_compute::variant_array::VariantType::metadata` · parquet-variant-compute 59.3.0

```rust
fn metadata(&self) -> &Self::Metadata
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::VariantType", "path": "VariantType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [129, 2], "filename": "src/variant_array.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/variant_array.rs:99`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-54add63bd635b9a76ac9d00f"></a>
## serialize_metadata

`function` · `parquet_variant_compute::variant_array::VariantType::serialize_metadata` · parquet-variant-compute 59.3.0

```rust
fn serialize_metadata(&self) -> Option<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::VariantType", "path": "VariantType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [129, 2], "filename": "src/variant_array.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/variant_array.rs:103`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c08a5815e8c8a5249059a2b0"></a>
## supports_data_type

`function` · `parquet_variant_compute::variant_array::VariantType::supports_data_type` · parquet-variant-compute 59.3.0

```rust
fn supports_data_type(&self, data_type: &DataType) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::VariantType", "path": "VariantType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [129, 2], "filename": "src/variant_array.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/variant_array.rs:111`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da0a97896397df8305a13703"></a>
## try_new

`function` · `parquet_variant_compute::variant_array::VariantType::try_new` · parquet-variant-compute 59.3.0

```rust
fn try_new(data_type: &DataType, _metadata: Self::Metadata) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::VariantType", "path": "VariantType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [129, 2], "filename": "src/variant_array.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/variant_array.rs:121`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0eca7c49a7c50741d981a30b"></a>
## validate

`function` · `parquet_variant_compute::variant_array::VariantType::validate` · parquet-variant-compute 59.3.0

```rust
fn validate(data_type: &DataType, _metadata: Self::Metadata) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::VariantType", "path": "VariantType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [129, 2], "filename": "src/variant_array.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/variant_array.rs:126`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
