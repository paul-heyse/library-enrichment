# `sqlparser::tokenizer::Span`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.tokenizer.Span.json).

<a id="op-c744756cf28c8276d407a53a"></a>
## Span

`struct` · `sqlparser::tokenizer::Span` · sqlparser 0.62.0

```rust
struct Span
```

Source: `src/tokenizer.rs:615`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A span represents a linear portion of the input string (start, end)

See [Spanned](crate::ast::Spanned) for more information.

<a id="op-d90918a4fe40d6eb68737f7f"></a>
## clone

`function` · `sqlparser::tokenizer::Span::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Span", "path": "Span"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [612, 31], "end": [612, 36], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/tokenizer.rs:612`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-617c0d1641b3f1e8927ef0fe"></a>
## cmp

`function` · `sqlparser::tokenizer::Span::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Span) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Span", "path": "Span"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [612, 50], "end": [612, 53], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/tokenizer.rs:612`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-514bf1b9d61f1665d396aa86"></a>
## deserialize

`function` · `sqlparser::tokenizer::Span::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Span", "path": "Span"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [613, 49], "end": [613, 60], "filename": "src/tokenizer.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/tokenizer.rs:613`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-901a002887b77badb7cad462"></a>
## empty

`function` · `sqlparser::tokenizer::Span::empty` · sqlparser 0.62.0

```rust
const fn empty() -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Span", "path": "Span"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [628, 1], "end": [709, 2], "filename": "src/tokenizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/tokenizer.rs:642`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns an empty span `(0, 0) -> (0, 0)`

Empty spans represent no knowledge of source location
See [Spanned](crate::ast::Spanned) for more information.

<a id="op-272889fbce15992531893501"></a>
## end

`struct_field` · `sqlparser::tokenizer::Span::end` · sqlparser 0.62.0

```rust
end: Location
```

Source: `src/tokenizer.rs:619`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

End `Location` (inclusive).

<a id="op-1fc39cc05899a65c263f643f"></a>
## eq

`function` · `sqlparser::tokenizer::Span::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Span) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Span", "path": "Span"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [612, 14], "end": [612, 23], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/tokenizer.rs:612`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4cb9b6a5be5d7208617c0018"></a>
## fmt

`function` · `sqlparser::tokenizer::Span::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Span", "path": "Span"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [622, 1], "end": [626, 2], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/tokenizer.rs:623`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2140df0e0fba0e03e622dce6"></a>
## hash

`function` · `sqlparser::tokenizer::Span::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Span", "path": "Span"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [612, 25], "end": [612, 29], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/tokenizer.rs:612`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f23c7b33d5075412aef979a9"></a>
## new

`function` · `sqlparser::tokenizer::Span::new` · sqlparser 0.62.0

```rust
fn new(start: Location, end: Location) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Span", "path": "Span"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [628, 1], "end": [709, 2], "filename": "src/tokenizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/tokenizer.rs:634`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Create a new span from a start and end [`Location`](../operations/sqlparser.tokenizer.Location.md#op-e28189c4b3ca84cefc9c04c4)

<a id="op-05ed2b2a5b9d013861c9a040"></a>
## partial_cmp

`function` · `sqlparser::tokenizer::Span::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Span) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Span", "path": "Span"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [612, 38], "end": [612, 48], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/tokenizer.rs:612`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc78c30e0ef797a17f9447c3"></a>
## serialize

`function` · `sqlparser::tokenizer::Span::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Span", "path": "Span"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [613, 38], "end": [613, 47], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/tokenizer.rs:613`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4892cecd473a28d08c12e2cc"></a>
## start

`struct_field` · `sqlparser::tokenizer::Span::start` · sqlparser 0.62.0

```rust
start: Location
```

Source: `src/tokenizer.rs:617`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Start `Location` (inclusive).

<a id="op-64a4bbe828bb7954d46a141d"></a>
## union

`function` · `sqlparser::tokenizer::Span::union` · sqlparser 0.62.0

```rust
fn union(&self, other: &Span) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Span", "path": "Span"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [628, 1], "end": [709, 2], "filename": "src/tokenizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/tokenizer.rs:664`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns the smallest Span that contains both `self` and `other`
If either span is [Span::empty](../operations/sqlparser.tokenizer.Span.md#op-901a002887b77badb7cad462), the other span is returned

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

<a id="op-63bebe5a33cef77821351385"></a>
## union_iter

`function` · `sqlparser::tokenizer::Span::union_iter` · sqlparser 0.62.0

```rust
fn union_iter<I: IntoIterator<Item = Span>>(iter: I) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Span", "path": "Span"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [628, 1], "end": [709, 2], "filename": "src/tokenizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/tokenizer.rs:704`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Return the [Span::union](../operations/sqlparser.tokenizer.Span.md#op-64a4bbe828bb7954d46a141d) of all spans in the iterator

If the iterator is empty, an empty span is returned

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

<a id="op-bd3fe0cd3bc9df833543b93b"></a>
## union_opt

`function` · `sqlparser::tokenizer::Span::union_opt` · sqlparser 0.62.0

```rust
fn union_opt(&self, other: &Option<Span>) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Span", "path": "Span"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [628, 1], "end": [709, 2], "filename": "src/tokenizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/tokenizer.rs:680`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Same as [Span::union](../operations/sqlparser.tokenizer.Span.md#op-64a4bbe828bb7954d46a141d) for `Option<Span>`

If `other` is `None`, `self` is returned

<a id="op-42926c219d075205786bc0b2"></a>
## visit

`function` · `sqlparser::tokenizer::Span::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Span", "path": "Span"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [614, 47], "end": [614, 55], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/tokenizer.rs:614`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6bb1932aadade497a9ac7057"></a>
## visit

`function` · `sqlparser::tokenizer::Span::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Span", "path": "Span"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [614, 40], "end": [614, 45], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/tokenizer.rs:614`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
