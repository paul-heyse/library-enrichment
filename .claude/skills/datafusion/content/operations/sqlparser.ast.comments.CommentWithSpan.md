# `sqlparser::ast::comments::CommentWithSpan`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.comments.CommentWithSpan.json).

<a id="op-e77a0b56f54623ba1294d568"></a>
## CommentWithSpan

`struct` · `sqlparser::ast::comments::CommentWithSpan` · sqlparser 0.62.0

```rust
struct CommentWithSpan
```

Source: `src/ast/comments.rs:155`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A source code comment with information of its entire span.

<a id="op-3da424141bdd2ddac34ac211"></a>
## Target

`assoc_type` · `sqlparser::ast::comments::CommentWithSpan::Target` · sqlparser 0.62.0

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::comments::CommentWithSpan", "path": "CommentWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [162, 1], "end": [168, 2], "filename": "src/ast/comments.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/ast/comments.rs:163`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20d204171fcb9e2b9e5921f2"></a>
## clone

`function` · `sqlparser::ast::comments::CommentWithSpan::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CommentWithSpan
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::comments::CommentWithSpan", "path": "CommentWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 17], "end": [154, 22], "filename": "src/ast/comments.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/comments.rs:154`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a804b8768bc145118501e5c1"></a>
## comment

`struct_field` · `sqlparser::ast::comments::CommentWithSpan::comment` · sqlparser 0.62.0

```rust
comment: Comment
```

Source: `src/ast/comments.rs:157`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The source code comment iself

<a id="op-6d5600db97bfb6470c07dc00"></a>
## deref

`function` · `sqlparser::ast::comments::CommentWithSpan::deref` · sqlparser 0.62.0

```rust
fn deref(&self) -> &Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::comments::CommentWithSpan", "path": "CommentWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [162, 1], "end": [168, 2], "filename": "src/ast/comments.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/ast/comments.rs:165`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-74fb296edada7baabcb7785d"></a>
## eq

`function` · `sqlparser::ast::comments::CommentWithSpan::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CommentWithSpan) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::comments::CommentWithSpan", "path": "CommentWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 24], "end": [154, 33], "filename": "src/ast/comments.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/comments.rs:154`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba4682f4733313f1f0414ac2"></a>
## fmt

`function` · `sqlparser::ast::comments::CommentWithSpan::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::comments::CommentWithSpan", "path": "CommentWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 10], "end": [154, 15], "filename": "src/ast/comments.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/comments.rs:154`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf4bf02c0227989b44531f4e"></a>
## hash

`function` · `sqlparser::ast::comments::CommentWithSpan::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::comments::CommentWithSpan", "path": "CommentWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 39], "end": [154, 43], "filename": "src/ast/comments.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/comments.rs:154`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-65b06cc0f382f8ea21ee7132"></a>
## span

`struct_field` · `sqlparser::ast::comments::CommentWithSpan::span` · sqlparser 0.62.0

```rust
span: tokenizer::Span
```

Source: `src/ast/comments.rs:159`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The span of the comment including its markers

<a id="op-8d4ebfd6789b8d6cd31ae873"></a>
## span

`function` · `sqlparser::ast::comments::CommentWithSpan::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::comments::CommentWithSpan", "path": "comments::CommentWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2581, 1], "end": [2585, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2582`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
