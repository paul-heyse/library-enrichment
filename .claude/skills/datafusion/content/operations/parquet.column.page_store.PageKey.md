# `parquet::column::page_store::PageKey`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.page_store.PageKey.json).

<a id="op-ed2b8f23a50733178579bb70"></a>
## PageKey

`struct` · `parquet::column::page_store::PageKey` · parquet 59.3.0

```rust
struct PageKey
```

Source: `src/column/page_store.rs:43`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

An opaque, store-allocated handle to a blob held by a [`PageStore`](../operations/parquet.column.page_store.PageStore.md#op-049980600cf52224c7af9631).

Handles are allocated by the store — densely and sequentially — and are only
meaningful to the store that produced them. The caller treats them as opaque
tokens.

<a id="op-962131dc85f4b1a4315a6f3d"></a>
## clone

`function` · `parquet::column::page_store::PageKey::clone` · parquet 59.3.0

```rust
fn clone(&self) -> PageKey
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page_store::PageKey", "path": "PageKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 17], "end": [42, 22], "filename": "src/column/page_store.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/column/page_store.rs:42`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad6299130f0881f21ceaf8b9"></a>
## cmp

`function` · `parquet::column::page_store::PageKey::cmp` · parquet 59.3.0

```rust
fn cmp(&self, other: &PageKey) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page_store::PageKey", "path": "PageKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 57], "end": [42, 60], "filename": "src/column/page_store.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/column/page_store.rs:42`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c67b348b7eecd6c21427796"></a>
## eq

`function` · `parquet::column::page_store::PageKey::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &PageKey) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page_store::PageKey", "path": "PageKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 30], "end": [42, 39], "filename": "src/column/page_store.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/column/page_store.rs:42`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e024f33898c3116882c72bf1"></a>
## fmt

`function` · `parquet::column::page_store::PageKey::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page_store::PageKey", "path": "PageKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 10], "end": [42, 15], "filename": "src/column/page_store.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/column/page_store.rs:42`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fae16faaff961590b461b7c4"></a>
## get

`function` · `parquet::column::page_store::PageKey::get` · parquet 59.3.0

```rust
const fn get(self) -> u64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page_store::PageKey", "path": "PageKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [60, 2], "filename": "src/column/page_store.rs"}, "trait": null, "trait_path": null}`

Source: `src/column/page_store.rs:57`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The raw value passed to [`new`](Self::new).

<a id="op-79465fed6aefb31be611b70f"></a>
## hash

`function` · `parquet::column::page_store::PageKey::hash` · parquet 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page_store::PageKey", "path": "PageKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 62], "end": [42, 66], "filename": "src/column/page_store.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/column/page_store.rs:42`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4110dc4269862349c6bbd10c"></a>
## new

`function` · `parquet::column::page_store::PageKey::new` · parquet 59.3.0

```rust
const fn new(raw: u64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page_store::PageKey", "path": "PageKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [60, 2], "filename": "src/column/page_store.rs"}, "trait": null, "trait_path": null}`

Source: `src/column/page_store.rs:52`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a handle wrapping `raw`.

A [`PageStore`](../operations/parquet.column.page_store.PageStore.md#op-049980600cf52224c7af9631) implementation calls this to mint the handle it returns
from [`put`](PageStore::put). The value is opaque to the caller, so a
store is free to use a dense counter, a packed locator, or anything else
it can later resolve in [`take`](PageStore::take).

<a id="op-7dafd4704b3eb23eabc07af7"></a>
## partial_cmp

`function` · `parquet::column::page_store::PageKey::partial_cmp` · parquet 59.3.0

```rust
fn partial_cmp(&self, other: &PageKey) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page_store::PageKey", "path": "PageKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 45], "end": [42, 55], "filename": "src/column/page_store.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/column/page_store.rs:42`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
