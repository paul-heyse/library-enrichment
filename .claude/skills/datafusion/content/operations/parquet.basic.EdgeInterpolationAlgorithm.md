# `parquet::basic::EdgeInterpolationAlgorithm`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.basic.EdgeInterpolationAlgorithm.json).

<a id="op-a758133551a80028a0617ffd"></a>
## EdgeInterpolationAlgorithm

`enum` · `parquet::basic::EdgeInterpolationAlgorithm` · parquet 59.3.0

```rust
enum EdgeInterpolationAlgorithm
```

Source: `src/basic.rs:829`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Edge interpolation algorithm for [`LogicalType::Geography`](../operations/parquet.basic.LogicalType.md#op-d8f7db3ce93041ddf6ddf923)

<a id="op-33361b4a3a2d4df99699e6b6"></a>
## ANDOYER

`variant` · `parquet::basic::EdgeInterpolationAlgorithm::ANDOYER` · parquet 59.3.0

```rust
ANDOYER
```

Source: `src/basic.rs:838`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Thomas, Paul D. Mathematical models for navigation systems. US Naval Oceanographic Office, 1965.

<a id="op-3550daf85bd4b5c5e54a9347"></a>
## Err

`assoc_type` · `parquet::basic::EdgeInterpolationAlgorithm::Err` · parquet 59.3.0

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::EdgeInterpolationAlgorithm", "path": "EdgeInterpolationAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [886, 1], "end": [902, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/basic.rs:887`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c47149eeec0b9ca3f40e0166"></a>
## KARNEY

`variant` · `parquet::basic::EdgeInterpolationAlgorithm::KARNEY` · parquet 59.3.0

```rust
KARNEY
```

Source: `src/basic.rs:840`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Karney, Charles FF. "Algorithms for geodesics." Journal of Geodesy 87 (2013): 43-55

<a id="op-5a1e9397bed962a0a9504cbc"></a>
## SPHERICAL

`variant` · `parquet::basic::EdgeInterpolationAlgorithm::SPHERICAL` · parquet 59.3.0

```rust
SPHERICAL
```

Source: `src/basic.rs:832`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Edges are interpolated as geodesics on a sphere.

<a id="op-dd0b2603da45600c64edb453"></a>
## THOMAS

`variant` · `parquet::basic::EdgeInterpolationAlgorithm::THOMAS` · parquet 59.3.0

```rust
THOMAS
```

Source: `src/basic.rs:836`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Thomas, Paul D. Spheroidal geodesics, reference systems, & local geometry. US Naval Oceanographic Office, 1970

<a id="op-58fc80b74fcfe09f3a84b0a2"></a>
## VINCENTY

`variant` · `parquet::basic::EdgeInterpolationAlgorithm::VINCENTY` · parquet 59.3.0

```rust
VINCENTY
```

Source: `src/basic.rs:834`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

<https://en.wikipedia.org/wiki/Vincenty%27s_formulae>

<a id="op-267564e0ff1fca9daa559fe1"></a>
## _Unknown

`variant` · `parquet::basic::EdgeInterpolationAlgorithm::_Unknown` · parquet 59.3.0

```rust
_Unknown
```

Source: `src/basic.rs:842`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Unknown algorithm

<a id="op-939d2c8458fea03ab4caa797"></a>
## clone

`function` · `parquet::basic::EdgeInterpolationAlgorithm::clone` · parquet 59.3.0

```rust
fn clone(&self) -> EdgeInterpolationAlgorithm
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::EdgeInterpolationAlgorithm", "path": "EdgeInterpolationAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [826, 10], "end": [826, 15], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/basic.rs:826`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c07beaaf0c5c7a8403220b6"></a>
## cmp

`function` · `parquet::basic::EdgeInterpolationAlgorithm::cmp` · parquet 59.3.0

```rust
fn cmp(&self, other: &EdgeInterpolationAlgorithm) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::EdgeInterpolationAlgorithm", "path": "EdgeInterpolationAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [826, 40], "end": [826, 43], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/basic.rs:826`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6f2c18060f7993d99a058a0f"></a>
## default

`function` · `parquet::basic::EdgeInterpolationAlgorithm::default` · parquet 59.3.0

```rust
fn default() -> EdgeInterpolationAlgorithm
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::EdgeInterpolationAlgorithm", "path": "EdgeInterpolationAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [828, 10], "end": [828, 17], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/basic.rs:828`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c32c94b93dbbc3dc85ee1436"></a>
## eq

`function` · `parquet::basic::EdgeInterpolationAlgorithm::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &EdgeInterpolationAlgorithm) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::EdgeInterpolationAlgorithm", "path": "EdgeInterpolationAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [826, 45], "end": [826, 54], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/basic.rs:826`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39f0189770a9cdd93df24741"></a>
## fmt

`function` · `parquet::basic::EdgeInterpolationAlgorithm::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::EdgeInterpolationAlgorithm", "path": "EdgeInterpolationAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [826, 23], "end": [826, 28], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/basic.rs:826`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1fa0c84425b0e5e1e7a4f81"></a>
## fmt

`function` · `parquet::basic::EdgeInterpolationAlgorithm::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::EdgeInterpolationAlgorithm", "path": "EdgeInterpolationAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [867, 1], "end": [871, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/basic.rs:868`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9762b9255b0537d1126b2ade"></a>
## from

`function` · `parquet::basic::EdgeInterpolationAlgorithm::from` · parquet 59.3.0

```rust
fn from(value: parquet_geospatial::WkbEdges) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::EdgeInterpolationAlgorithm", "path": "EdgeInterpolationAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [874, 1], "end": [884, 2], "filename": "src/basic.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet_geospatial::types::Edges", "path": "Edges"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/basic.rs:875`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f0f512f40591a1e192944cc"></a>
## from_str

`function` · `parquet::basic::EdgeInterpolationAlgorithm::from_str` · parquet 59.3.0

```rust
fn from_str(s: &str) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::EdgeInterpolationAlgorithm", "path": "EdgeInterpolationAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [886, 1], "end": [902, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/basic.rs:889`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e0cc6e9a440928251bf50cf"></a>
## hash

`function` · `parquet::basic::EdgeInterpolationAlgorithm::hash` · parquet 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::EdgeInterpolationAlgorithm", "path": "EdgeInterpolationAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [826, 34], "end": [826, 38], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/basic.rs:826`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9299d60a33bf6176c1ea040b"></a>
## partial_cmp

`function` · `parquet::basic::EdgeInterpolationAlgorithm::partial_cmp` · parquet 59.3.0

```rust
fn partial_cmp(&self, other: &EdgeInterpolationAlgorithm) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::EdgeInterpolationAlgorithm", "path": "EdgeInterpolationAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [826, 56], "end": [826, 66], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/basic.rs:826`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0c37f4c2991156afd6b72ed"></a>
## try_as_edges

`function` · `parquet::basic::EdgeInterpolationAlgorithm::try_as_edges` · parquet 59.3.0

```rust
fn try_as_edges(&self) -> Result<parquet_geospatial::WkbEdges>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::EdgeInterpolationAlgorithm", "path": "EdgeInterpolationAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [846, 1], "end": [865, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:852`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Converts an [`EdgeInterpolationAlgorithm`](../operations/parquet.basic.EdgeInterpolationAlgorithm.md#op-a758133551a80028a0617ffd) into its corresponding algorithm defined by
[`parquet_geospatial::WkbEdges`].

This method will only return an Err if the [`EdgeInterpolationAlgorithm`](../operations/parquet.basic.EdgeInterpolationAlgorithm.md#op-a758133551a80028a0617ffd) is the `_Unknown`
variant.

Unresolved upstream links (retained, not inferred): ``parquet_geospatial::WkbEdges``.
