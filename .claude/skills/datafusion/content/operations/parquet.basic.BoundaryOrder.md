# `parquet::basic::BoundaryOrder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.basic.BoundaryOrder.json).

<a id="op-fbf37bb94d5d2d627ceb68a4"></a>
## BoundaryOrder

`enum` · `parquet::basic::BoundaryOrder` · parquet 59.3.0

```rust
enum BoundaryOrder
```

Source: `src/basic.rs:810`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Enum to annotate whether lists of min/max elements inside ColumnIndex
are ordered and if so, in which direction.

<a id="op-20de9fc0e41deff9c602545a"></a>
## ASCENDING

`variant` · `parquet::basic::BoundaryOrder::ASCENDING` · parquet 59.3.0

```rust
ASCENDING
```

Source: `src/basic.rs:810`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-938adc7a2e46bddddb4aeb7d"></a>
## DESCENDING

`variant` · `parquet::basic::BoundaryOrder::DESCENDING` · parquet 59.3.0

```rust
DESCENDING
```

Source: `src/basic.rs:810`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29838877a4db7cc6f1293e85"></a>
## MAX_DISCRIMINANT

`assoc_const` · `parquet::basic::BoundaryOrder::MAX_DISCRIMINANT` · parquet 59.3.0

```rust
MAX_DISCRIMINANT
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::BoundaryOrder", "path": "BoundaryOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [810, 1], "end": [818, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:810`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the largest discriminant value defined for this enum.

<a id="op-648d3713c8a7a6d4b4ea2eaa"></a>
## UNORDERED

`variant` · `parquet::basic::BoundaryOrder::UNORDERED` · parquet 59.3.0

```rust
UNORDERED
```

Source: `src/basic.rs:810`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-107e7f82dcee14e9d136a724"></a>
## VARIANTS

`assoc_const` · `parquet::basic::BoundaryOrder::VARIANTS` · parquet 59.3.0

```rust
VARIANTS
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::BoundaryOrder", "path": "BoundaryOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [810, 1], "end": [818, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:810`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a slice containing every variant of this enum.

<a id="op-9e7ce647bf46e02084c783fc"></a>
## clone

`function` · `parquet::basic::BoundaryOrder::clone` · parquet 59.3.0

```rust
fn clone(&self) -> BoundaryOrder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::BoundaryOrder", "path": "BoundaryOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [810, 1], "end": [818, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/basic.rs:810`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa0f9654109156a9fcd24e29"></a>
## cmp

`function` · `parquet::basic::BoundaryOrder::cmp` · parquet 59.3.0

```rust
fn cmp(&self, other: &BoundaryOrder) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::BoundaryOrder", "path": "BoundaryOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [810, 1], "end": [818, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/basic.rs:810`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-097dbe8dfea21f31c00e8d78"></a>
## eq

`function` · `parquet::basic::BoundaryOrder::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &BoundaryOrder) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::BoundaryOrder", "path": "BoundaryOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [810, 1], "end": [818, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/basic.rs:810`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4cb956eafd9645d469684e24"></a>
## fmt

`function` · `parquet::basic::BoundaryOrder::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::BoundaryOrder", "path": "BoundaryOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [810, 1], "end": [818, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/basic.rs:810`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc6193600fea9acdc6f68bb5"></a>
## fmt

`function` · `parquet::basic::BoundaryOrder::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::BoundaryOrder", "path": "BoundaryOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [810, 1], "end": [818, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/basic.rs:810`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae1fb002a7a20d61d7638c48"></a>
## hash

`function` · `parquet::basic::BoundaryOrder::hash` · parquet 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::BoundaryOrder", "path": "BoundaryOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [810, 1], "end": [818, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/basic.rs:810`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35a902aac41a981b468fb229"></a>
## partial_cmp

`function` · `parquet::basic::BoundaryOrder::partial_cmp` · parquet 59.3.0

```rust
fn partial_cmp(&self, other: &BoundaryOrder) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::BoundaryOrder", "path": "BoundaryOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [810, 1], "end": [818, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/basic.rs:810`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
