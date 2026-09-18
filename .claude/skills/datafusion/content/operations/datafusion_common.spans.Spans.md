# `datafusion_common::spans::Spans`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.spans.Spans.json).

<a id="op-fbbf3241bfdb08ad34f51334"></a>
## Spans

`struct` · `datafusion_common::spans::Spans` · datafusion-common 55.1.0

```rust
struct Spans
```

Source: `src/spans.rs:147`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A collection of [`Span`](../operations/datafusion_common.spans.Span.md#op-f6759e6258e5342ee1352ef2), meant to be used as a field of entities whose
location in the original SQL query is desired to be tracked. Sometimes an
entity can have multiple spans. e.g. if you want to track the position of
the column a that comes from SELECT 1 AS a UNION ALL SELECT 2 AS a you'll
need two spans.

<a id="op-bf8ee21ad581713640758eb5"></a>
## 0

`struct_field` · `datafusion_common::spans::Spans::0` · datafusion-common 55.1.0

```rust
0: Vec<Span>
```

Source: `src/spans.rs:147`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e8cd6ec6ef1c43466654499"></a>
## add_span

`function` · `datafusion_common::spans::Spans::add_span` · datafusion-common 55.1.0

```rust
fn add_span(&mut self, span: Span)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::spans::Spans", "path": "Spans"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [175, 2], "filename": "src/spans.rs"}, "trait": null, "trait_path": null}`

Source: `src/spans.rs:167`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Adds a [`Span`](../operations/datafusion_common.spans.Span.md#op-f6759e6258e5342ee1352ef2) to the collection.

<a id="op-7b88747fd75c466edd258fa6"></a>
## clone

`function` · `datafusion_common::spans::Spans::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> Spans
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::spans::Spans", "path": "Spans"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 17], "end": [144, 22], "filename": "src/spans.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/spans.rs:144`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-380dfb42d21fc27bb3824f95"></a>
## cmp

`function` · `datafusion_common::spans::Spans::cmp` · datafusion-common 55.1.0

```rust
fn cmp(&self, _other: &Self) -> Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::spans::Spans", "path": "Spans"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 1], "end": [213, 2], "filename": "src/spans.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/spans.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6910be7392cfc68e9fb4d81"></a>
## default

`function` · `datafusion_common::spans::Spans::default` · datafusion-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::spans::Spans", "path": "Spans"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [177, 1], "end": [181, 2], "filename": "src/spans.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/spans.rs:178`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f5acf6df8cf65c5d6ca4b0d"></a>
## eq

`function` · `datafusion_common::spans::Spans::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, _other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::spans::Spans", "path": "Spans"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [190, 2], "filename": "src/spans.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/spans.rs:187`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e0dcfa9ee2e90b6b97794ca5"></a>
## first

`function` · `datafusion_common::spans::Spans::first` · datafusion-common 55.1.0

```rust
fn first(&self) -> Option<Span>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::spans::Spans", "path": "Spans"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [175, 2], "filename": "src/spans.rs"}, "trait": null, "trait_path": null}`

Source: `src/spans.rs:157`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns the first [`Span`](../operations/datafusion_common.spans.Span.md#op-f6759e6258e5342ee1352ef2), if any. This is useful when you know that
there's gonna be only one [`Span`](../operations/datafusion_common.spans.Span.md#op-f6759e6258e5342ee1352ef2) at most.

<a id="op-b85ff256469f70343174b3d3"></a>
## fmt

`function` · `datafusion_common::spans::Spans::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::spans::Spans", "path": "Spans"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 10], "end": [144, 15], "filename": "src/spans.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/spans.rs:144`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-093bcf28b31a2b548166d073"></a>
## get_spans

`function` · `datafusion_common::spans::Spans::get_spans` · datafusion-common 55.1.0

```rust
fn get_spans(&self) -> &[Span]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::spans::Spans", "path": "Spans"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [175, 2], "filename": "src/spans.rs"}, "trait": null, "trait_path": null}`

Source: `src/spans.rs:162`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns a slice of the [`Span`](../operations/datafusion_common.spans.Span.md#op-f6759e6258e5342ee1352ef2)s.

<a id="op-bb2cd5319cb14384e8fe07fd"></a>
## hash

`function` · `datafusion_common::spans::Spans::hash` · datafusion-common 55.1.0

```rust
fn hash<H: Hasher>(&self, _state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::spans::Spans", "path": "Spans"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [218, 1], "end": [220, 2], "filename": "src/spans.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/spans.rs:219`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1054452b2e30c6f9f61b3fed"></a>
## iter

`function` · `datafusion_common::spans::Spans::iter` · datafusion-common 55.1.0

```rust
fn iter(&self) -> impl Iterator<Item = &Span>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::spans::Spans", "path": "Spans"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [175, 2], "filename": "src/spans.rs"}, "trait": null, "trait_path": null}`

Source: `src/spans.rs:172`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Iterates over the [`Span`](../operations/datafusion_common.spans.Span.md#op-f6759e6258e5342ee1352ef2)s.

<a id="op-f99203aa8288dd756cad79ca"></a>
## new

`function` · `datafusion_common::spans::Spans::new` · datafusion-common 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::spans::Spans", "path": "Spans"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [175, 2], "filename": "src/spans.rs"}, "trait": null, "trait_path": null}`

Source: `src/spans.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Creates a new empty [`Spans`](../operations/datafusion_common.spans.Spans.md#op-fbbf3241bfdb08ad34f51334) with no [`Span`](../operations/datafusion_common.spans.Span.md#op-f6759e6258e5342ee1352ef2).

<a id="op-5146ffde8464f351b42433e4"></a>
## partial_cmp

`function` · `datafusion_common::spans::Spans::partial_cmp` · datafusion-common 55.1.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::spans::Spans", "path": "Spans"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 1], "end": [204, 2], "filename": "src/spans.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/spans.rs:201`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
