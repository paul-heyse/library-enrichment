# `parquet::basic::VariantType`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.basic.VariantType.json).

<a id="op-88fef9018b5a216b5b51b0b2"></a>
## VariantType

`struct` · `parquet::basic::VariantType` · parquet 59.3.0

```rust
struct VariantType
```

Source: `src/basic.rs:219`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2fcf30a338bbbb2266d966d"></a>
## clone

`function` · `parquet::basic::VariantType::clone` · parquet 59.3.0

```rust
fn clone(&self) -> VariantType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::VariantType", "path": "VariantType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [225, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/basic.rs:219`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c9eb091a85fc0c86cf477b0"></a>
## eq

`function` · `parquet::basic::VariantType::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &VariantType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::VariantType", "path": "VariantType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [225, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/basic.rs:219`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28a212032864b8388957cc57"></a>
## fmt

`function` · `parquet::basic::VariantType::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::VariantType", "path": "VariantType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [225, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/basic.rs:219`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1150cd36ea517b9ad484a938"></a>
## specification_version

`struct_field` · `parquet::basic::VariantType::specification_version` · parquet 59.3.0

```rust
specification_version: Option<i8>
```

Source: `src/basic.rs:219`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The version of the variant specification that the variant was
written with.
