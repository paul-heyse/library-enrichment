# `sqlparser::ast::comments::Iter`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.comments.Iter.json).

<a id="op-af3db7b0da4bbe4f0ade5785"></a>
## Iter

`struct` · `sqlparser::ast::comments::Iter` · sqlparser 0.62.0

```rust
struct Iter<'a>
```

Source: `src/ast/comments.rs:212`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An opaque iterator implementation over comments served by [Comments::find](../operations/sqlparser.ast.comments.Comments.md#op-e2e9e65fb072221f78752d7f).

<a id="op-0613c932480d4310cd4ff18e"></a>
## Item

`assoc_type` · `sqlparser::ast::comments::Iter::Item` · sqlparser 0.62.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::ast::comments::Iter", "path": "Iter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [214, 1], "end": [220, 2], "filename": "src/ast/comments.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/ast/comments.rs:215`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fdcaa4258d512bdca20a106d"></a>
## next

`function` · `sqlparser::ast::comments::Iter::next` · sqlparser 0.62.0

```rust
fn next(&mut self) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::ast::comments::Iter", "path": "Iter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [214, 1], "end": [220, 2], "filename": "src/ast/comments.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/ast/comments.rs:217`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
