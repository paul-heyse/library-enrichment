# `parquet::basic::SortOrder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.basic.SortOrder.json).

<a id="op-4f8bd6b55f834082d2298e22"></a>
## SortOrder

`enum` · `parquet::basic::SortOrder` · parquet 59.3.0

```rust
enum SortOrder
```

Source: `src/basic.rs:981`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sort order for page and column statistics.

Types are associated with sort orders and column stats are aggregated using a sort
order, and a sort order should be considered when comparing values with statistics
min/max.

See reference in
<https://github.com/apache/arrow/blob/main/cpp/src/parquet/types.h>

<a id="op-f2cc3fca3f66facbea73f2e7"></a>
## SIGNED

`variant` · `parquet::basic::SortOrder::SIGNED` · parquet 59.3.0

```rust
SIGNED
```

Source: `src/basic.rs:983`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Signed (either value or legacy byte-wise) comparison.

<a id="op-bc1e8ce8383930db7b0a5764"></a>
## UNDEFINED

`variant` · `parquet::basic::SortOrder::UNDEFINED` · parquet 59.3.0

```rust
UNDEFINED
```

Source: `src/basic.rs:987`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Comparison is undefined.

<a id="op-739638e64b9a08c6818c9741"></a>
## UNSIGNED

`variant` · `parquet::basic::SortOrder::UNSIGNED` · parquet 59.3.0

```rust
UNSIGNED
```

Source: `src/basic.rs:985`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Unsigned (depending on physical type either value or byte-wise) comparison.

<a id="op-9fb6443ffa447da59b3e82f1"></a>
## clone

`function` · `parquet::basic::SortOrder::clone` · parquet 59.3.0

```rust
fn clone(&self) -> SortOrder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::SortOrder", "path": "SortOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [979, 17], "end": [979, 22], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/basic.rs:979`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-239ce10b743f42fb41cc31bd"></a>
## eq

`function` · `parquet::basic::SortOrder::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &SortOrder) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::SortOrder", "path": "SortOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [979, 30], "end": [979, 39], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/basic.rs:979`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-07da729a548f19cea16131c8"></a>
## fmt

`function` · `parquet::basic::SortOrder::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::SortOrder", "path": "SortOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [979, 10], "end": [979, 15], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/basic.rs:979`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ccf268420a9e5177e4754cf"></a>
## fmt

`function` · `parquet::basic::SortOrder::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::SortOrder", "path": "SortOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1184, 1], "end": [1188, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/basic.rs:1185`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dfee5a5b904f4c129af252fa"></a>
## is_signed

`function` · `parquet::basic::SortOrder::is_signed` · parquet 59.3.0

```rust
fn is_signed(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::SortOrder", "path": "SortOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [990, 1], "end": [995, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:992`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns true if this is [`Self::SIGNED`](../operations/parquet.basic.SortOrder.md#op-f2cc3fca3f66facbea73f2e7)
