# `parquet::basic::GeographyType`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.basic.GeographyType.json).

<a id="op-3acd124053e6757b279016fc"></a>
## GeographyType

`struct` · `parquet::basic::GeographyType` · parquet 59.3.0

```rust
struct GeographyType
```

Source: `src/basic.rs:235`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0936b860ee634d5494ded8cb"></a>
## algorithm

`function` · `parquet::basic::GeographyType::algorithm` · parquet 59.3.0

```rust
fn algorithm(&self) -> Option<EdgeInterpolationAlgorithm>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::GeographyType", "path": "GeographyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [245, 1], "end": [254, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:251`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Accessor for the `GeographyType::algorithm` field. If this field is not set, this
function returns the default value (currently [`EdgeInterpolationAlgorithm::SPHERICAL`](../operations/parquet.basic.EdgeInterpolationAlgorithm.md#op-5a1e9397bed962a0a9504cbc)
per the Parquet [specification]).

[specification]: https://github.com/apache/parquet-format/blob/master/LogicalTypes.md#geography

<a id="op-d2872c8ff0326e92eef04db7"></a>
## algorithm

`struct_field` · `parquet::basic::GeographyType::algorithm` · parquet 59.3.0

```rust
algorithm: Option<EdgeInterpolationAlgorithm>
```

Source: `src/basic.rs:235`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

An optional algorithm can be set to correctly interpret edges interpolation
of the geometries. If unset, the `SPHERICAL` algorithm should be used.

<a id="op-b1115043ed3fe061e3538777"></a>
## clone

`function` · `parquet::basic::GeographyType::clone` · parquet 59.3.0

```rust
fn clone(&self) -> GeographyType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::GeographyType", "path": "GeographyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 1], "end": [243, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/basic.rs:235`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2f0a9118ad05750afa0b5d3"></a>
## crs

`struct_field` · `parquet::basic::GeographyType::crs` · parquet 59.3.0

```rust
crs: Option<String>
```

Source: `src/basic.rs:235`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A custom CRS. If unset the CRS `OGC:CRS84` should be used.

<a id="op-3dc77f9164568a0a18570590"></a>
## eq

`function` · `parquet::basic::GeographyType::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &GeographyType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::GeographyType", "path": "GeographyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 1], "end": [243, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/basic.rs:235`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-971b9d206a0ed28a89016373"></a>
## fmt

`function` · `parquet::basic::GeographyType::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::GeographyType", "path": "GeographyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 1], "end": [243, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/basic.rs:235`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
