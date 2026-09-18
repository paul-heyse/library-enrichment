# `parquet::basic::Type`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.basic.Type.json).

<a id="op-796c3f25eb2f2865773fe11c"></a>
## Type

`enum` · `parquet::basic::Type` · parquet 59.3.0

```rust
enum Type
```

Source: `src/basic.rs:46`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Types supported by Parquet.

These physical types are intended to be used in combination with the encodings to
control the on disk storage format.
For example INT16 is not included as a type since a good encoding of INT32
would handle this.

<a id="op-00a09c180a7b3b92c6202e0d"></a>
## BOOLEAN

`variant` · `parquet::basic::Type::BOOLEAN` · parquet 59.3.0

```rust
BOOLEAN
```

Source: `src/basic.rs:46`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb6265c321e8e45191325ad7"></a>
## BYTE_ARRAY

`variant` · `parquet::basic::Type::BYTE_ARRAY` · parquet 59.3.0

```rust
BYTE_ARRAY
```

Source: `src/basic.rs:46`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81d5e824c53d46e0be3b5d04"></a>
## DOUBLE

`variant` · `parquet::basic::Type::DOUBLE` · parquet 59.3.0

```rust
DOUBLE
```

Source: `src/basic.rs:46`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d0b670fb7334a49b06428c3"></a>
## Err

`assoc_type` · `parquet::basic::Type::Err` · parquet 59.3.0

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1269, 1], "end": [1285, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/basic.rs:1270`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0c1cde975781ec6977e79e6"></a>
## FIXED_LEN_BYTE_ARRAY

`variant` · `parquet::basic::Type::FIXED_LEN_BYTE_ARRAY` · parquet 59.3.0

```rust
FIXED_LEN_BYTE_ARRAY
```

Source: `src/basic.rs:46`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a066c84d1e22c8c89ae2637f"></a>
## FLOAT

`variant` · `parquet::basic::Type::FLOAT` · parquet 59.3.0

```rust
FLOAT
```

Source: `src/basic.rs:46`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00002a9f20e79b7779cc8e19"></a>
## INT32

`variant` · `parquet::basic::Type::INT32` · parquet 59.3.0

```rust
INT32
```

Source: `src/basic.rs:46`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7efb01d885237ea0c911803"></a>
## INT64

`variant` · `parquet::basic::Type::INT64` · parquet 59.3.0

```rust
INT64
```

Source: `src/basic.rs:46`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c3d05ccf275cb0b34aa5a8d"></a>
## INT96

`variant` · `parquet::basic::Type::INT96` · parquet 59.3.0

```rust
INT96
```

Source: `src/basic.rs:46`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68e23e11b8a4bc8189e084a5"></a>
## MAX_DISCRIMINANT

`assoc_const` · `parquet::basic::Type::MAX_DISCRIMINANT` · parquet 59.3.0

```rust
MAX_DISCRIMINANT
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [63, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:46`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the largest discriminant value defined for this enum.

<a id="op-6c27d7107a6d3cd1d690ddd4"></a>
## VARIANTS

`assoc_const` · `parquet::basic::Type::VARIANTS` · parquet 59.3.0

```rust
VARIANTS
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [63, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:46`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a slice containing every variant of this enum.

<a id="op-5b254c6a4af846aa00bb1f6c"></a>
## clone

`function` · `parquet::basic::Type::clone` · parquet 59.3.0

```rust
fn clone(&self) -> Type
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [63, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/basic.rs:46`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a318a78a3ba1ff4ee0daec7e"></a>
## cmp

`function` · `parquet::basic::Type::cmp` · parquet 59.3.0

```rust
fn cmp(&self, other: &Type) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [63, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/basic.rs:46`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8c9a95b36774502e3f437f8"></a>
## eq

`function` · `parquet::basic::Type::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &Type) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [63, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/basic.rs:46`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0565386f6d889ab90a23afd4"></a>
## fmt

`function` · `parquet::basic::Type::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [63, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/basic.rs:46`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3eaf32b66d8201e2ec1a45f"></a>
## fmt

`function` · `parquet::basic::Type::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [63, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/basic.rs:46`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b56fd21fb32dd2f5ea364dd"></a>
## from_str

`function` · `parquet::basic::Type::from_str` · parquet 59.3.0

```rust
fn from_str(s: &str) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1269, 1], "end": [1285, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/basic.rs:1272`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f6145432948d3f0d68875505"></a>
## hash

`function` · `parquet::basic::Type::hash` · parquet 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [63, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/basic.rs:46`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-275e8e001fbef3226984d212"></a>
## partial_cmp

`function` · `parquet::basic::Type::partial_cmp` · parquet 59.3.0

```rust
fn partial_cmp(&self, other: &Type) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [63, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/basic.rs:46`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
