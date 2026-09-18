# `datafusion_common::spans::Span`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.spans.Span.json).

<a id="op-f6759e6258e5342ee1352ef2"></a>
## Span

`struct` · `datafusion_common::spans::Span` · datafusion-common 55.1.0

```rust
struct Span
```

Source: `src/spans.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Represents an interval of characters in the original SQL query.

<a id="op-1b11572ad3010476e87eda23"></a>
## clone

`function` · `datafusion_common::spans::Span::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::spans::Span", "path": "Span"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 31], "end": [53, 36], "filename": "src/spans.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/spans.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea8e3ef4a0af2d1a11a24a3b"></a>
## cmp

`function` · `datafusion_common::spans::Span::cmp` · datafusion-common 55.1.0

```rust
fn cmp(&self, other: &Span) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::spans::Span", "path": "Span"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 50], "end": [53, 53], "filename": "src/spans.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/spans.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc272ebd9024b68e96762953"></a>
## end

`struct_field` · `datafusion_common::spans::Span::end` · datafusion-common 55.1.0

```rust
end: Location
```

Source: `src/spans.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-07f2bbeaedaccf80a0cc610b"></a>
## eq

`function` · `datafusion_common::spans::Span::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &Span) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::spans::Span", "path": "Span"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 14], "end": [53, 23], "filename": "src/spans.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/spans.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d66da9a8f55e6e3d01f076f4"></a>
## fmt

`function` · `datafusion_common::spans::Span::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::spans::Span", "path": "Span"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [63, 2], "filename": "src/spans.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/spans.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-731dba4fc3f62f4c5b91bb23"></a>
## hash

`function` · `datafusion_common::spans::Span::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::spans::Span", "path": "Span"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 25], "end": [53, 29], "filename": "src/spans.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/spans.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3dad1763b4577b50f59593ed"></a>
## new

`function` · `datafusion_common::spans::Span::new` · datafusion-common 55.1.0

```rust
fn new(start: Location, end: Location) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::spans::Span", "path": "Span"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [137, 2], "filename": "src/spans.rs"}, "trait": null, "trait_path": null}`

Source: `src/spans.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Creates a new [`Span`](../operations/datafusion_common.spans.Span.md#op-f6759e6258e5342ee1352ef2) from a start and an end [`Location`](../operations/datafusion_common.spans.Location.md#op-14296911d166f7886092e520).

<a id="op-d4dc455d5e46c3acdc390f45"></a>
## partial_cmp

`function` · `datafusion_common::spans::Span::partial_cmp` · datafusion-common 55.1.0

```rust
fn partial_cmp(&self, other: &Span) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::spans::Span", "path": "Span"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 38], "end": [53, 48], "filename": "src/spans.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/spans.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86ba3ea4bcdaa7a15a29f914"></a>
## start

`struct_field` · `datafusion_common::spans::Span::start` · datafusion-common 55.1.0

```rust
start: Location
```

Source: `src/spans.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e59a5678719b9ed95c2eb08d"></a>
## try_from_sqlparser_span

`function` · `datafusion_common::spans::Span::try_from_sqlparser_span` · datafusion-common 55.1.0

```rust
fn try_from_sqlparser_span(span: sqlparser::tokenizer::Span) -> Option<Span>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::spans::Span", "path": "Span"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [137, 2], "filename": "src/spans.rs"}, "trait": null, "trait_path": null}`

Source: `src/spans.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Convert a [`Span`](sqlparser::tokenizer::Span) from the parser, into a
DataFusion [`Span`](../operations/datafusion_common.spans.Span.md#op-f6759e6258e5342ee1352ef2). If the input span is empty (line 0 column 0, to
line 0 column 0), then [`None`] is returned.

Unresolved upstream links (retained, not inferred): ``None``.

<a id="op-7c864b37ec60f073bb56e5fd"></a>
## union

`function` · `datafusion_common::spans::Span::union` · datafusion-common 55.1.0

```rust
fn union(&self, other: &Span) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::spans::Span", "path": "Span"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [137, 2], "filename": "src/spans.rs"}, "trait": null, "trait_path": null}`

Source: `src/spans.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns the smallest Span that contains both `self` and `other`

# Examples
```
# use sqlparser::tokenizer::{Span, Location};
// line 1, column1 -> line 2, column 5
let span1 = Span::new(Location::new(1, 1), Location::new(2, 5));
// line 2, column 3 -> line 3, column 7
let span2 = Span::new(Location::new(2, 3), Location::new(3, 7));
// Union of the two is the min/max of the two spans
// line 1, column 1 -> line 3, column 7
let union = span1.union(&span2);
assert_eq!(union, Span::new(Location::new(1, 1), Location::new(3, 7)));
```

<a id="op-0537ccb1117c4d539eac8320"></a>
## union_iter

`function` · `datafusion_common::spans::Span::union_iter` · datafusion-common 55.1.0

```rust
fn union_iter<I: IntoIterator<Item = Span>>(iter: I) -> Option<Span>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::spans::Span", "path": "Span"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [137, 2], "filename": "src/spans.rs"}, "trait": null, "trait_path": null}`

Source: `src/spans.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Return the [Span::union](../operations/datafusion_common.spans.Span.md#op-7c864b37ec60f073bb56e5fd) of all spans in the iterator.

If the iterator is empty, [`None`] is returned.

# Example
```
# use sqlparser::tokenizer::{Span, Location};
let spans = vec![
    Span::new(Location::new(1, 1), Location::new(2, 5)),
    Span::new(Location::new(2, 3), Location::new(3, 7)),
    Span::new(Location::new(3, 1), Location::new(4, 2)),
];
// line 1, column 1 -> line 4, column 2
assert_eq!(
  Span::union_iter(spans),
  Span::new(Location::new(1, 1), Location::new(4, 2))
);

Unresolved upstream links (retained, not inferred): ``None``.

<a id="op-ae351af4c0f3dc9681b26c76"></a>
## union_opt

`function` · `datafusion_common::spans::Span::union_opt` · datafusion-common 55.1.0

```rust
fn union_opt(&self, other: &Option<Span>) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::spans::Span", "path": "Span"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [137, 2], "filename": "src/spans.rs"}, "trait": null, "trait_path": null}`

Source: `src/spans.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Same as [Span::union](../operations/datafusion_common.spans.Span.md#op-7c864b37ec60f073bb56e5fd) for `Option<Span>`.

If `other` is `None`, `self` is returned.
