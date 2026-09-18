# `parquet::basic::FieldRepetitionType`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.basic.FieldRepetitionType.json).

<a id="op-6ce509f4589343d2ba6b293e"></a>
## FieldRepetitionType

`enum` · `parquet::basic::FieldRepetitionType` · parquet 59.3.0

```rust
enum FieldRepetitionType
```

Source: `src/basic.rs:353`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Representation of field types in schema.

<a id="op-cb0c074bd08667fea33ef98d"></a>
## MAX_DISCRIMINANT

`assoc_const` · `parquet::basic::FieldRepetitionType::MAX_DISCRIMINANT` · parquet 59.3.0

```rust
MAX_DISCRIMINANT
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::FieldRepetitionType", "path": "FieldRepetitionType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [353, 1], "end": [363, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:353`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the largest discriminant value defined for this enum.

<a id="op-e4e81ac8cbfb1fac9919d37c"></a>
## OPTIONAL

`variant` · `parquet::basic::FieldRepetitionType::OPTIONAL` · parquet 59.3.0

```rust
OPTIONAL
```

Source: `src/basic.rs:353`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The field is optional (can be null) and each row has 0 or 1 values.

<a id="op-c1fd3cf687126999f90e4215"></a>
## REPEATED

`variant` · `parquet::basic::FieldRepetitionType::REPEATED` · parquet 59.3.0

```rust
REPEATED
```

Source: `src/basic.rs:353`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The field is repeated and can contain 0 or more values.

<a id="op-ae3f20476021c3f822f9f3e4"></a>
## REQUIRED

`variant` · `parquet::basic::FieldRepetitionType::REQUIRED` · parquet 59.3.0

```rust
REQUIRED
```

Source: `src/basic.rs:353`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

This field is required (can not be null) and each row has exactly 1 value.

<a id="op-191f78c52b795ecc2f7b1704"></a>
## VARIANTS

`assoc_const` · `parquet::basic::FieldRepetitionType::VARIANTS` · parquet 59.3.0

```rust
VARIANTS
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::FieldRepetitionType", "path": "FieldRepetitionType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [353, 1], "end": [363, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:353`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a slice containing every variant of this enum.

<a id="op-60292c7e5106955665f9206c"></a>
## clone

`function` · `parquet::basic::FieldRepetitionType::clone` · parquet 59.3.0

```rust
fn clone(&self) -> FieldRepetitionType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::FieldRepetitionType", "path": "FieldRepetitionType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [353, 1], "end": [363, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/basic.rs:353`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa0914968724a5432fe0f8d0"></a>
## cmp

`function` · `parquet::basic::FieldRepetitionType::cmp` · parquet 59.3.0

```rust
fn cmp(&self, other: &FieldRepetitionType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::FieldRepetitionType", "path": "FieldRepetitionType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [353, 1], "end": [363, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/basic.rs:353`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37d94562dd77cca9f6ca305e"></a>
## eq

`function` · `parquet::basic::FieldRepetitionType::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &FieldRepetitionType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::FieldRepetitionType", "path": "FieldRepetitionType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [353, 1], "end": [363, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/basic.rs:353`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b15ba2d3a1f5683337121ba"></a>
## fmt

`function` · `parquet::basic::FieldRepetitionType::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::FieldRepetitionType", "path": "FieldRepetitionType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [353, 1], "end": [363, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/basic.rs:353`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c0ea3e34c02f8434cd9f0da"></a>
## fmt

`function` · `parquet::basic::FieldRepetitionType::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::FieldRepetitionType", "path": "FieldRepetitionType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [353, 1], "end": [363, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/basic.rs:353`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f6061424a5295342e3cd7f85"></a>
## hash

`function` · `parquet::basic::FieldRepetitionType::hash` · parquet 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::FieldRepetitionType", "path": "FieldRepetitionType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [353, 1], "end": [363, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/basic.rs:353`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b050633547274a923db47dab"></a>
## partial_cmp

`function` · `parquet::basic::FieldRepetitionType::partial_cmp` · parquet 59.3.0

```rust
fn partial_cmp(&self, other: &FieldRepetitionType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::FieldRepetitionType", "path": "FieldRepetitionType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [353, 1], "end": [363, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/basic.rs:353`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
