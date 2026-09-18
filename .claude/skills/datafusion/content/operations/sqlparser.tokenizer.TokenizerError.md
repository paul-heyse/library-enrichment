# `sqlparser::tokenizer::TokenizerError`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.tokenizer.TokenizerError.json).

<a id="op-48b7775009ea5e759a5d8f9c"></a>
## TokenizerError

`struct` · `sqlparser::tokenizer::TokenizerError` · sqlparser 0.62.0

```rust
struct TokenizerError
```

Source: `src/tokenizer.rs:790`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An error reported by the tokenizer, with a human-readable `message` and a `location`.

<a id="op-10f2f274bac54ec0f085407d"></a>
## eq

`function` · `sqlparser::tokenizer::TokenizerError::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TokenizerError) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::TokenizerError", "path": "TokenizerError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [789, 17], "end": [789, 26], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/tokenizer.rs:789`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09569a261b9af106aa95428a"></a>
## fmt

`function` · `sqlparser::tokenizer::TokenizerError::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::TokenizerError", "path": "TokenizerError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [789, 10], "end": [789, 15], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/tokenizer.rs:789`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-344585625a3d2b9f647694fe"></a>
## fmt

`function` · `sqlparser::tokenizer::TokenizerError::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::TokenizerError", "path": "TokenizerError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [797, 1], "end": [801, 2], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/tokenizer.rs:798`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-56c4552abc1a5dfd83669d9d"></a>
## location

`struct_field` · `sqlparser::tokenizer::TokenizerError::location` · sqlparser 0.62.0

```rust
location: Location
```

Source: `src/tokenizer.rs:794`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `Location` where the error was detected.

<a id="op-5df4b49155c18433c32b4697"></a>
## message

`struct_field` · `sqlparser::tokenizer::TokenizerError::message` · sqlparser 0.62.0

```rust
message: String
```

Source: `src/tokenizer.rs:792`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A descriptive error message.
