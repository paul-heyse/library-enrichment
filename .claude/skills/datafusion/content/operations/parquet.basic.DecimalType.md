# `parquet::basic::DecimalType`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.basic.DecimalType.json).

<a id="op-9834afaa76b594fc9cd1d595"></a>
## DecimalType

`struct` · `parquet::basic::DecimalType` · parquet 59.3.0

```rust
struct DecimalType
```

Source: `src/basic.rs:189`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-40a6f6bf0c1372c1a4f2dba0"></a>
## clone

`function` · `parquet::basic::DecimalType::clone` · parquet 59.3.0

```rust
fn clone(&self) -> DecimalType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::DecimalType", "path": "DecimalType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [189, 1], "end": [196, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/basic.rs:189`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49fff4b4cb066a609ff26417"></a>
## eq

`function` · `parquet::basic::DecimalType::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &DecimalType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::DecimalType", "path": "DecimalType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [189, 1], "end": [196, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/basic.rs:189`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d6a8d21c341ea679d897a49"></a>
## fmt

`function` · `parquet::basic::DecimalType::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::DecimalType", "path": "DecimalType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [189, 1], "end": [196, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/basic.rs:189`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4db5a55f4902176f3f624150"></a>
## precision

`struct_field` · `parquet::basic::DecimalType::precision` · parquet 59.3.0

```rust
precision: i32
```

Source: `src/basic.rs:189`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The location of the decimal point.

<a id="op-63556b7429fca22b77658771"></a>
## scale

`struct_field` · `parquet::basic::DecimalType::scale` · parquet 59.3.0

```rust
scale: i32
```

Source: `src/basic.rs:189`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The number of digits in the decimal.
