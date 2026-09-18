# `sqlparser::ast::OneOrManyWithParensIntoIter`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.OneOrManyWithParensIntoIter.json).

<a id="op-5548c99f73fe661092debe7c"></a>
## OneOrManyWithParensIntoIter

`struct` · `sqlparser::ast::OneOrManyWithParensIntoIter` · sqlparser 0.62.0

```rust
struct OneOrManyWithParensIntoIter<T>
```

Source: `src/ast/mod.rs:1603`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Owned iterator implementation of `OneOrManyWithParens`

<a id="op-70be2a970ad030af9cd72f18"></a>
## Item

`assoc_type` · `sqlparser::ast::OneOrManyWithParensIntoIter::Item` · sqlparser 0.62.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::OneOrManyWithParensIntoIter", "path": "OneOrManyWithParensIntoIter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1627, 1], "end": [1664, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/ast/mod.rs:1628`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1aec10eacac307ac3c9ef24"></a>
## clone

`function` · `sqlparser::ast::OneOrManyWithParensIntoIter::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> OneOrManyWithParensIntoIter<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::OneOrManyWithParensIntoIter", "path": "OneOrManyWithParensIntoIter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1602, 17], "end": [1602, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:1602`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5928930d879cfa2981142183"></a>
## count

`function` · `sqlparser::ast::OneOrManyWithParensIntoIter::count` · sqlparser 0.62.0

```rust
fn count(self) -> usize where Self: Sized
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::OneOrManyWithParensIntoIter", "path": "OneOrManyWithParensIntoIter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1627, 1], "end": [1664, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/ast/mod.rs:1644`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0904724b25c1643d21dcc6a8"></a>
## fmt

`function` · `sqlparser::ast::OneOrManyWithParensIntoIter::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::OneOrManyWithParensIntoIter", "path": "OneOrManyWithParensIntoIter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1602, 10], "end": [1602, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:1602`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca8b593a7a1e9e6365d1d057"></a>
## fold

`function` · `sqlparser::ast::OneOrManyWithParensIntoIter::fold` · sqlparser 0.62.0

```rust
fn fold<B, F>(self, init: B, f: F) -> B where Self: Sized, F: FnMut(B, Self::Item) -> B
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::OneOrManyWithParensIntoIter", "path": "OneOrManyWithParensIntoIter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1627, 1], "end": [1664, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/ast/mod.rs:1654`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9e639a14ecdc1f7235145ce"></a>
## next

`function` · `sqlparser::ast::OneOrManyWithParensIntoIter::next` · sqlparser 0.62.0

```rust
fn next(&mut self) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::OneOrManyWithParensIntoIter", "path": "OneOrManyWithParensIntoIter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1627, 1], "end": [1664, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/ast/mod.rs:1630`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dbfcbdfb35dbbcb973d1b4e7"></a>
## next_back

`function` · `sqlparser::ast::OneOrManyWithParensIntoIter::next_back` · sqlparser 0.62.0

```rust
fn next_back(&mut self) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::OneOrManyWithParensIntoIter", "path": "OneOrManyWithParensIntoIter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1666, 1], "end": [1673, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::double_ended::DoubleEndedIterator", "path": "DoubleEndedIterator"}, "trait_path": "core::iter::traits::double_ended::DoubleEndedIterator"}`

Source: `src/ast/mod.rs:1667`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7a4765cce420e50a1ae4a15"></a>
## size_hint

`function` · `sqlparser::ast::OneOrManyWithParensIntoIter::size_hint` · sqlparser 0.62.0

```rust
fn size_hint(&self) -> (usize, Option<usize>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::OneOrManyWithParensIntoIter", "path": "OneOrManyWithParensIntoIter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1627, 1], "end": [1664, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/ast/mod.rs:1637`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
