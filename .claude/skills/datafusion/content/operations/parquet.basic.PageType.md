# `parquet::basic::PageType`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.basic.PageType.json).

<a id="op-9932999a245d509642ceeb04"></a>
## PageType

`enum` · `parquet::basic::PageType` · parquet 59.3.0

```rust
enum PageType
```

Source: `src/basic.rs:796`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Available data pages for Parquet file format.
Note that some of the page types may not be supported.

<a id="op-6a2db92079e2d47d1d79034f"></a>
## DATA_PAGE

`variant` · `parquet::basic::PageType::DATA_PAGE` · parquet 59.3.0

```rust
DATA_PAGE
```

Source: `src/basic.rs:796`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c9d34067366fef12a80a8835"></a>
## DATA_PAGE_V2

`variant` · `parquet::basic::PageType::DATA_PAGE_V2` · parquet 59.3.0

```rust
DATA_PAGE_V2
```

Source: `src/basic.rs:796`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-039d81bf43c2369c78b2b0a1"></a>
## DICTIONARY_PAGE

`variant` · `parquet::basic::PageType::DICTIONARY_PAGE` · parquet 59.3.0

```rust
DICTIONARY_PAGE
```

Source: `src/basic.rs:796`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21d690d6f471f8f4558c81c0"></a>
## INDEX_PAGE

`variant` · `parquet::basic::PageType::INDEX_PAGE` · parquet 59.3.0

```rust
INDEX_PAGE
```

Source: `src/basic.rs:796`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33ecd338d19ac6d1980d5ce6"></a>
## MAX_DISCRIMINANT

`assoc_const` · `parquet::basic::PageType::MAX_DISCRIMINANT` · parquet 59.3.0

```rust
MAX_DISCRIMINANT
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::PageType", "path": "PageType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [796, 1], "end": [805, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:796`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the largest discriminant value defined for this enum.

<a id="op-18402bca4ad1f92f73fa97f2"></a>
## VARIANTS

`assoc_const` · `parquet::basic::PageType::VARIANTS` · parquet 59.3.0

```rust
VARIANTS
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::PageType", "path": "PageType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [796, 1], "end": [805, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:796`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a slice containing every variant of this enum.

<a id="op-e53aa44e04e0cb03b97d2366"></a>
## clone

`function` · `parquet::basic::PageType::clone` · parquet 59.3.0

```rust
fn clone(&self) -> PageType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::PageType", "path": "PageType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [796, 1], "end": [805, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/basic.rs:796`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f27f7cb65701bb660ca74b55"></a>
## cmp

`function` · `parquet::basic::PageType::cmp` · parquet 59.3.0

```rust
fn cmp(&self, other: &PageType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::PageType", "path": "PageType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [796, 1], "end": [805, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/basic.rs:796`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-544dc15fc794dfb5175905aa"></a>
## eq

`function` · `parquet::basic::PageType::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &PageType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::PageType", "path": "PageType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [796, 1], "end": [805, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/basic.rs:796`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72b779b9d380307eb2f9d8c3"></a>
## fmt

`function` · `parquet::basic::PageType::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::PageType", "path": "PageType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [796, 1], "end": [805, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/basic.rs:796`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f83490b133d49905d7d527c7"></a>
## fmt

`function` · `parquet::basic::PageType::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::PageType", "path": "PageType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [796, 1], "end": [805, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/basic.rs:796`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb925489f204da3f6a2d271b"></a>
## hash

`function` · `parquet::basic::PageType::hash` · parquet 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::PageType", "path": "PageType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [796, 1], "end": [805, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/basic.rs:796`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2ea50f96a51cd792c1b2bbd"></a>
## partial_cmp

`function` · `parquet::basic::PageType::partial_cmp` · parquet 59.3.0

```rust
fn partial_cmp(&self, other: &PageType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::PageType", "path": "PageType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [796, 1], "end": [805, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/basic.rs:796`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
