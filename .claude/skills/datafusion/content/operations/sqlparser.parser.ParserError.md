# `sqlparser::parser::ParserError`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.parser.ParserError.json).

<a id="op-69a995b52087ac70f2c20443"></a>
## ParserError

`enum` · `sqlparser::parser::ParserError` · sqlparser 0.62.0

```rust
enum ParserError
```

Source: `src/parser/mod.rs:52`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Errors produced by the SQL parser.

<a id="op-2d9fd37147f8d1046f7a6cec"></a>
## ParserError

`variant` · `sqlparser::parser::ParserError::ParserError` · sqlparser 0.62.0

```rust
ParserError
```

Source: `src/parser/mod.rs:56`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Generic parser error with a message.

<a id="op-30c7068096427b0c5d403ae4"></a>
## RecursionLimitExceeded

`variant` · `sqlparser::parser::ParserError::RecursionLimitExceeded` · sqlparser 0.62.0

```rust
RecursionLimitExceeded
```

Source: `src/parser/mod.rs:58`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Raised when a recursion depth limit is exceeded.

<a id="op-89d9c653521f1f28833ad954"></a>
## TokenizerError

`variant` · `sqlparser::parser::ParserError::TokenizerError` · sqlparser 0.62.0

```rust
TokenizerError
```

Source: `src/parser/mod.rs:54`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Error originating from the tokenizer with a message.

<a id="op-8f277de1913465cc2acefad2"></a>
## clone

`function` · `sqlparser::parser::ParserError::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ParserError
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::parser::ParserError", "path": "ParserError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 17], "end": [51, 22], "filename": "src/parser/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/parser/mod.rs:51`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0913a591b4cd254341514454"></a>
## eq

`function` · `sqlparser::parser::ParserError::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ParserError) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::parser::ParserError", "path": "ParserError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 24], "end": [51, 33], "filename": "src/parser/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/parser/mod.rs:51`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83f0301a78ee494819a21c99"></a>
## fmt

`function` · `sqlparser::parser::ParserError::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::parser::ParserError", "path": "ParserError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 10], "end": [51, 15], "filename": "src/parser/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/parser/mod.rs:51`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-946d6e37f785a90ac3eb11b7"></a>
## fmt

`function` · `sqlparser::parser::ParserError::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::parser::ParserError", "path": "ParserError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [192, 1], "end": [204, 2], "filename": "src/parser/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/parser/mod.rs:193`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a786fa4b73917a316737b533"></a>
## from

`function` · `sqlparser::parser::ParserError::from` · sqlparser 0.62.0

```rust
fn from(e: TokenizerError) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::parser::ParserError", "path": "ParserError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [190, 2], "filename": "src/parser/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::TokenizerError", "path": "TokenizerError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/parser/mod.rs:187`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
