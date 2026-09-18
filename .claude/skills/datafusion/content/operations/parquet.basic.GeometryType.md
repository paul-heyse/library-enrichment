# `parquet::basic::GeometryType`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.basic.GeometryType.json).

<a id="op-d7716fee150ad1856b7d3e1c"></a>
## GeometryType

`struct` · `parquet::basic::GeometryType` · parquet 59.3.0

```rust
struct GeometryType
```

Source: `src/basic.rs:227`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-922cb1322a5c7a09cca5d0a5"></a>
## clone

`function` · `parquet::basic::GeometryType::clone` · parquet 59.3.0

```rust
fn clone(&self) -> GeometryType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::GeometryType", "path": "GeometryType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 1], "end": [233, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/basic.rs:227`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-96777b3d583c6d3515de948f"></a>
## crs

`struct_field` · `parquet::basic::GeometryType::crs` · parquet 59.3.0

```rust
crs: Option<String>
```

Source: `src/basic.rs:227`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A custom CRS. If unset the CRS `OGC:CRS84` should be used, which means that the geometries
must be stored in longitude, latitude based on the WGS84 datum.

<a id="op-bd6e64c890a441fae0cc0315"></a>
## eq

`function` · `parquet::basic::GeometryType::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &GeometryType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::GeometryType", "path": "GeometryType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 1], "end": [233, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/basic.rs:227`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd759fa33576228782bdbd04"></a>
## fmt

`function` · `parquet::basic::GeometryType::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::GeometryType", "path": "GeometryType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 1], "end": [233, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/basic.rs:227`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
