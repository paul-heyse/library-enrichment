# `datafusion_sql::parser::ParserInput`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.parser.ParserInput.json).

<a id="op-a4aeaf4c9e915c540a4ca1dd"></a>
## ParserInput

`enum` · `datafusion_sql::parser::ParserInput` · datafusion-sql 55.1.0

```rust
enum ParserInput<'a>
```

Source: `src/parser.rs:430`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Describes a possible input for parser

<a id="op-cc86e60ac38ff9f0671cc7fd"></a>
## Sql

`variant` · `datafusion_sql::parser::ParserInput::Sql` · datafusion-sql 55.1.0

```rust
Sql
```

Source: `src/parser.rs:433`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Raw SQL. Tokenization will be performed automatically as a
part of [`DFParserBuilder::build`](../operations/datafusion_sql.parser.DFParserBuilder.md#op-e6572b0d816b01d6b2debf75)

<a id="op-c74fc14ba9bcd55665797de0"></a>
## Tokens

`variant` · `datafusion_sql::parser::ParserInput::Tokens` · datafusion-sql 55.1.0

```rust
Tokens
```

Source: `src/parser.rs:435`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Tokens

<a id="op-3c6f4b172dc5dc6772a8b58a"></a>
## from

`function` · `datafusion_sql::parser::ParserInput::from` · datafusion-sql 55.1.0

```rust
fn from(tokens: Vec<TokenWithSpan>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'static"}], "constraints": []}}, "id": "datafusion_sql::parser::ParserInput", "path": "ParserInput"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [444, 1], "end": [448, 2], "filename": "src/parser.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::TokenWithSpan", "path": "TokenWithSpan"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/parser.rs:445`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d50258bea9cf02be8c9bbd3"></a>
## from

`function` · `datafusion_sql::parser::ParserInput::from` · datafusion-sql 55.1.0

```rust
fn from(sql: &'a str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_sql::parser::ParserInput", "path": "ParserInput"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [438, 1], "end": [442, 2], "filename": "src/parser.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/parser.rs:439`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
