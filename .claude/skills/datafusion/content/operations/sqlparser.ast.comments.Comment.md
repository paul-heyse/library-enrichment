# `sqlparser::ast::comments::Comment`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.comments.Comment.json).

<a id="op-2d2ccec944361ec28f796510"></a>
## Comment

`enum` · `sqlparser::ast::comments::Comment` · sqlparser 0.62.0

```rust
enum Comment
```

Source: `src/ast/comments.rs:172`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A unified type of the different source code comment formats.

<a id="op-9cfd25884c6f7c9f773d0e6f"></a>
## MultiLine

`variant` · `sqlparser::ast::comments::Comment::MultiLine` · sqlparser 0.62.0

```rust
MultiLine
```

Source: `src/ast/comments.rs:190`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A multi-line comment, typically enclosed in `/* .. */` markers. The
string represents the content excluding the markers.

<a id="op-7031aa5c30be8288a019b74f"></a>
## SingleLine

`variant` · `sqlparser::ast::comments::Comment::SingleLine` · sqlparser 0.62.0

```rust
SingleLine
```

Source: `src/ast/comments.rs:181`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A single line comment, typically introduced with a prefix and spanning
until end-of-line or end-of-file in the source code.

Note: `content` will include the terminating new-line character, if any.
A single-line comment, typically introduced with a prefix and spanning
until end-of-line or end-of-file in the source code.

Note: `content` will include the terminating new-line character, if any.

<a id="op-6e611d76760da9672959dd19"></a>
## Target

`assoc_type` · `sqlparser::ast::comments::Comment::Target` · sqlparser 0.62.0

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::comments::Comment", "path": "Comment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [203, 1], "end": [209, 2], "filename": "src/ast/comments.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/ast/comments.rs:204`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18c9fb2fc021714c7e772b13"></a>
## as_str

`function` · `sqlparser::ast::comments::Comment::as_str` · sqlparser 0.62.0

```rust
fn as_str(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::comments::Comment", "path": "Comment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [193, 1], "end": [201, 2], "filename": "src/ast/comments.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/comments.rs:195`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Retrieves the content of the comment as string slice.

<a id="op-c27e8244c4f654f4a2958117"></a>
## clone

`function` · `sqlparser::ast::comments::Comment::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Comment
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::comments::Comment", "path": "Comment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 17], "end": [171, 22], "filename": "src/ast/comments.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/comments.rs:171`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0496b6b3aa4c3b45f91d15bd"></a>
## deref

`function` · `sqlparser::ast::comments::Comment::deref` · sqlparser 0.62.0

```rust
fn deref(&self) -> &Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::comments::Comment", "path": "Comment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [203, 1], "end": [209, 2], "filename": "src/ast/comments.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/ast/comments.rs:206`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20fefec6a6b8bc70b3b1e4f3"></a>
## eq

`function` · `sqlparser::ast::comments::Comment::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Comment) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::comments::Comment", "path": "Comment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 24], "end": [171, 33], "filename": "src/ast/comments.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/comments.rs:171`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d481a9f78ff37628f9337bb7"></a>
## fmt

`function` · `sqlparser::ast::comments::Comment::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::comments::Comment", "path": "Comment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 10], "end": [171, 15], "filename": "src/ast/comments.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/comments.rs:171`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa6df88543998687b39b21b8"></a>
## hash

`function` · `sqlparser::ast::comments::Comment::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::comments::Comment", "path": "Comment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 39], "end": [171, 43], "filename": "src/ast/comments.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/comments.rs:171`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
