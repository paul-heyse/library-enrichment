# `sqlparser::parser::Parser`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.parser.Parser.json).

<a id="op-b761887fc90688e669ac5e12"></a>
## Parser

`struct` · `sqlparser::parser::Parser` · sqlparser 0.62.0

```rust
struct Parser<'a>
```

Source: `src/parser/mod.rs:347`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A SQL Parser

This struct is the main entry point for parsing SQL queries.

# Functionality:
* Parsing SQL: see examples on [`Parser::new`](../operations/sqlparser.parser.Parser.md#op-82ec32737f9279974a92f7f8) and [`Parser::parse_sql`](../operations/sqlparser.parser.Parser.md#op-fc7ccb40a43499852e70701f)
* Controlling recursion: See [`Parser::with_recursion_limit`](../operations/sqlparser.parser.Parser.md#op-b818b7fc65470b89ca72babb)
* Controlling parser options: See [`Parser::with_options`](../operations/sqlparser.parser.Parser.md#op-6efafad3a1718a37999c1921)
* Providing your own tokens: See [`Parser::with_tokens`](../operations/sqlparser.parser.Parser.md#op-bdbbc36ca57b6b9439609fe6)

# Internals

The parser uses a [`Tokenizer`](../operations/sqlparser.tokenizer.Tokenizer.md#op-58f927537ea1ecff6e8c5b80) to tokenize the input SQL string into a
`Vec` of [`TokenWithSpan`](../operations/sqlparser.tokenizer.TokenWithSpan.md#op-8f18f1407940a3467d080b83)s and maintains an `index` to the current token
being processed. The token vec may contain multiple SQL statements.

* The "current" token is the token at `index - 1`
* The "next" token is the token at `index`
* The "previous" token is the token at `index - 2`

If `index` is equal to the length of the token stream, the 'next' token is
[`Token::EOF`](../operations/sqlparser.tokenizer.Token.md#op-e3b3bdb4e63cf95a06aad31e).

For example, the SQL string "SELECT * FROM foo" will be tokenized into
following tokens:
```text
 [
   "SELECT", // token index 0
   " ",      // whitespace
   "*",
   " ",
   "FROM",
   " ",
   "foo"
  ]
```



<a id="op-00b57e5c31bdb28b841eefea"></a>
## advance_token

`function` · `sqlparser::parser::Parser::advance_token` · sqlparser 0.62.0

```rust
fn advance_token(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4528`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Advances the current token to the next non-whitespace token

See [`Self::get_current_token`](../operations/sqlparser.parser.Parser.md#op-eed4593f7b246ee40e17c729) to get the current token after advancing

<a id="op-0e755536e7d3c4aac60998bd"></a>
## consume_token

`function` · `sqlparser::parser::Parser::consume_token` · sqlparser 0.62.0

```rust
fn consume_token(&mut self, expected: &Token) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4777`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Consume the next token if it matches the expected token, otherwise return false

See [Self::advance_token](../operations/sqlparser.parser.Parser.md#op-00b57e5c31bdb28b841eefea) to consume the token unconditionally

<a id="op-69d1eaaf078fb5a699eb7c08"></a>
## consume_tokens

`function` · `sqlparser::parser::Parser::consume_tokens` · sqlparser 0.62.0

```rust
fn consume_tokens(&mut self, tokens: &[Token]) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4790`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

If the current and subsequent tokens exactly match the `tokens`
sequence, consume them and returns true. Otherwise, no tokens are
consumed and returns false

<a id="op-9019a8042b56040f7720f12a"></a>
## expect_keyword

`function` · `sqlparser::parser::Parser::expect_keyword` · sqlparser 0.62.0

```rust
fn expect_keyword(&mut self, expected: Keyword) -> Result<TokenWithSpan, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4743`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

If the current token is the `expected` keyword, consume the token.
Otherwise, return an error.


<a id="op-bab3159ebfc69c2b8ec6ddee"></a>
## expect_keyword_is

`function` · `sqlparser::parser::Parser::expect_keyword_is` · sqlparser 0.62.0

```rust
fn expect_keyword_is(&mut self, expected: Keyword) -> Result<(), ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4756`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

If the current token is the `expected` keyword, consume the token.
Otherwise, return an error.

This differs from expect_keyword only in that the matched keyword
token is not returned.

<a id="op-5326bc56bbe8454533955d68"></a>
## expect_keywords

`function` · `sqlparser::parser::Parser::expect_keywords` · sqlparser 0.62.0

```rust
fn expect_keywords(&mut self, expected: &[Keyword]) -> Result<(), ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4766`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

If the current and subsequent tokens exactly match the `keywords`
sequence, consume them and returns Ok. Otherwise, return an Error.

<a id="op-36955db1c8fcc7282ddcbdb4"></a>
## expect_one_of_keywords

`function` · `sqlparser::parser::Parser::expect_one_of_keywords` · sqlparser 0.62.0

```rust
fn expect_one_of_keywords(&mut self, keywords: &[Keyword]) -> Result<Keyword, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4727`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

If the current token is one of the expected keywords, consume the token
and return the keyword that matches. Otherwise, return an error.

<a id="op-9fce075fae2b0b74c83308e0"></a>
## expect_token

`function` · `sqlparser::parser::Parser::expect_token` · sqlparser 0.62.0

```rust
fn expect_token(&mut self, expected: &Token) -> Result<TokenWithSpan, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4802`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Bail out if the current token is not an expected keyword, or consume it if it is

<a id="op-b2c8296f0d527dfe93ebf7fc"></a>
## expected

`function` · `sqlparser::parser::Parser::expected` · sqlparser 0.62.0

```rust
fn expected<T>(&self, expected: &str, found: TokenWithSpan) -> Result<T, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4584`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Report `found` was encountered instead of `expected`

<a id="op-457218fbc578faa791720fa4"></a>
## expected_at

`function` · `sqlparser::parser::Parser::expected_at` · sqlparser 0.62.0

```rust
fn expected_at<T>(&self, expected: &str, index: usize) -> Result<T, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4600`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Report that the token at `index` was found instead of `expected`.

<a id="op-a9371f0b86e6ef3fd9ed69f2"></a>
## expected_ref

`function` · `sqlparser::parser::Parser::expected_ref` · sqlparser 0.62.0

```rust
fn expected_ref<T>(&self, expected: &str, found: &TokenWithSpan) -> Result<T, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4592`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

report `found` was encountered instead of `expected`

<a id="op-b1be7bcba9983db7ce595db3"></a>
## get_current_index

`function` · `sqlparser::parser::Parser::get_current_index` · sqlparser 0.62.0

```rust
fn get_current_index(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4515`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns the index of the current token

This can be used with APIs that expect an index, such as
[`Self::token_at`](../operations/sqlparser.parser.Parser.md#op-b4b71c52793eb75b1b8900b5)

<a id="op-eed4593f7b246ee40e17c729"></a>
## get_current_token

`function` · `sqlparser::parser::Parser::get_current_token` · sqlparser 0.62.0

```rust
fn get_current_token(&self) -> &TokenWithSpan
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4544`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns a reference to the current token

Does not advance the current token.

<a id="op-ed88e159b4c47955a7d92159"></a>
## get_next_precedence

`function` · `sqlparser::parser::Parser::get_next_precedence` · sqlparser 0.62.0

```rust
fn get_next_precedence(&self) -> Result<u8, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4351`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Get the precedence of the next token

<a id="op-903ad1cb3eee2616b7e892a4"></a>
## get_next_token

`function` · `sqlparser::parser::Parser::get_next_token` · sqlparser 0.62.0

```rust
fn get_next_token(&self) -> &TokenWithSpan
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4558`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns a reference to the next token

Does not advance the current token.

<a id="op-ce53be6e32e0747f97eaea1a"></a>
## get_previous_token

`function` · `sqlparser::parser::Parser::get_previous_token` · sqlparser 0.62.0

```rust
fn get_previous_token(&self) -> &TokenWithSpan
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4551`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns a reference to the previous token

Does not advance the current token.

<a id="op-9e6daaaf0feeb65dba983b14"></a>
## index

`function` · `sqlparser::parser::Parser::index` · sqlparser 0.62.0

```rust
fn index(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:19775`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The index of the first unprocessed token.

<a id="op-5c88ebd70ec0f2bc9f85e995"></a>
## into_tokens

`function` · `sqlparser::parser::Parser::into_tokens` · sqlparser 0.62.0

```rust
fn into_tokens(self) -> Vec<TokenWithSpan>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:20311`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Consume the parser and return its underlying token buffer

<a id="op-887862023dfd17d7f34be5c8"></a>
## maybe_parse

`function` · `sqlparser::parser::Parser::maybe_parse` · sqlparser 0.62.0

```rust
fn maybe_parse<T, F>(&mut self, f: F) -> Result<Option<T>, ParserError> where F: FnMut(&mut Parser<'_>) -> Result<T, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:5057`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Run a parser method `f`, reverting back to the current position if unsuccessful.
Returns `ParserError::RecursionLimitExceeded` if `f` returns a `RecursionLimitExceeded`.
Returns `Ok(None)` if `f` returns any other error.

<a id="op-a7d32f37435769efd073dcce"></a>
## maybe_parse_connect_by

`function` · `sqlparser::parser::Parser::maybe_parse_connect_by` · sqlparser 0.62.0

```rust
fn maybe_parse_connect_by(&mut self) -> Result<Vec<ConnectByKind>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:15068`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a `CONNECT BY` clause (Oracle-style hierarchical query support).

<a id="op-05e44f52c00700bf98c6661d"></a>
## maybe_parse_options

`function` · `sqlparser::parser::Parser::maybe_parse_options` · sqlparser 0.62.0

```rust
fn maybe_parse_options(&mut self, keyword: Keyword) -> Result<Option<Vec<SqlOption>>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:9955`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optionally parse a parenthesized list of `SqlOption`s introduced by `keyword`.

<a id="op-092ad1427fb1e3bf6b58aaba"></a>
## maybe_parse_table_alias

`function` · `sqlparser::parser::Parser::maybe_parse_table_alias` · sqlparser 0.62.0

```rust
fn maybe_parse_table_alias(&mut self) -> Result<Option<TableAlias>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:12837`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optionally parses an alias for a table like in `... FROM generate_series(1, 10) AS t (col)`.
In this case, the alias is allowed to optionally name the columns in the table, in
addition to the table itself.

<a id="op-48e7ea55cee3a870fdb10138"></a>
## maybe_parse_table_version

`function` · `sqlparser::parser::Parser::maybe_parse_table_version` · sqlparser 0.62.0

```rust
fn maybe_parse_table_version(&mut self) -> Result<Option<TableVersion>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:16846`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parses a the timestamp version specifier (i.e. query historical data)

<a id="op-82ec32737f9279974a92f7f8"></a>
## new

`function` · `sqlparser::parser::Parser::new` · sqlparser 0.62.0

```rust
fn new(dialect: &'a dyn Dialect) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:380`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Create a parser for a [`Dialect`](../operations/sqlparser.dialect.Dialect.md#op-f184df0633eac2f7d134147b)

See also [`Parser::parse_sql`](../operations/sqlparser.parser.Parser.md#op-fc7ccb40a43499852e70701f)

Example:
```
# use sqlparser::{parser::{Parser, ParserError}, dialect::GenericDialect};
# fn main() -> Result<(), ParserError> {
let dialect = GenericDialect{};
let statements = Parser::new(&dialect)
  .try_with_sql("SELECT * FROM foo")?
  .parse_statements()?;
# Ok(())
# }
```

<a id="op-4f22791edb6d813770570674"></a>
## next_token

`function` · `sqlparser::parser::Parser::next_token` · sqlparser 0.62.0

```rust
fn next_token(&mut self) -> TokenWithSpan
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4506`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Advances to the next non-whitespace token and returns a copy.

Please use [`Self::advance_token`](../operations/sqlparser.parser.Parser.md#op-00b57e5c31bdb28b841eefea) and [`Self::get_current_token`](../operations/sqlparser.parser.Parser.md#op-eed4593f7b246ee40e17c729) to
avoid the copy.

<a id="op-c999c8c0d3c43115fb2c0663"></a>
## next_token_is_temporal_unit

`function` · `sqlparser::parser::Parser::next_token_is_temporal_unit` · sqlparser 0.62.0

```rust
fn next_token_is_temporal_unit(&mut self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:3345`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Peek at the next token and determine if it is a temporal unit
like `second`.

<a id="op-896b55aaf84121b5f6b97cfa"></a>
## next_token_no_skip

`function` · `sqlparser::parser::Parser::next_token_no_skip` · sqlparser 0.62.0

```rust
fn next_token_no_skip(&mut self) -> Option<&TokenWithSpan>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4520`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Return the next unprocessed token, possibly whitespace.

<a id="op-474493b1229ce0e9e44f1ba7"></a>
## parse_actions_list

`function` · `sqlparser::parser::Parser::parse_actions_list` · sqlparser 0.62.0

```rust
fn parse_actions_list(&mut self) -> Result<Vec<Action>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4840`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a list of actions for `GRANT` statements.

<a id="op-fb472728369ace38c9dbfafc"></a>
## parse_all_or_distinct

`function` · `sqlparser::parser::Parser::parse_all_or_distinct` · sqlparser 0.62.0

```rust
fn parse_all_or_distinct(&mut self) -> Result<Option<Distinct>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:5086`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse either `ALL`, `DISTINCT` or `DISTINCT ON (...)`. Returns [`None`] if `ALL` is parsed
and results in a [`ParserError`](../operations/sqlparser.parser.ParserError.md#op-69a995b52087ac70f2c20443) if both `ALL` and `DISTINCT` are found.

Unresolved upstream links (retained, not inferred): ``None``.

<a id="op-05eeddc46d0a59a799799c7b"></a>
## parse_alter

`function` · `sqlparser::parser::Parser::parse_alter` · sqlparser 0.62.0

```rust
fn parse_alter(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:10718`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an `ALTER <object>` statement and dispatch to the appropriate alter handler.

<a id="op-1756deaa8f6fd7630130fb0c"></a>
## parse_alter_collation

`function` · `sqlparser::parser::Parser::parse_alter_collation` · sqlparser 0.62.0

```rust
fn parse_alter_collation(&mut self) -> Result<AlterCollation, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:11124`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a [Statement::AlterCollation](../operations/sqlparser.ast.Statement.md#op-de7822fcc4ff7132ab1fb732).

[PostgreSQL Documentation](https://www.postgresql.org/docs/current/sql-altercollation.html)

<a id="op-db4b84589969b991e550d7df"></a>
## parse_alter_connector

`function` · `sqlparser::parser::Parser::parse_alter_connector` · sqlparser 0.62.0

```rust
fn parse_alter_connector(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "super::Parser"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 1], "end": [512, 2], "filename": "src/parser/alter.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/alter.rs:114`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an `ALTER CONNECTOR` statement
```sql
ALTER CONNECTOR connector_name SET DCPROPERTIES(property_name=property_value, ...);

ALTER CONNECTOR connector_name SET URL new_url;

ALTER CONNECTOR connector_name SET OWNER [USER|ROLE] user_or_role;
```

<a id="op-1ea1b052396dd9b8a629c2b3"></a>
## parse_alter_function

`function` · `sqlparser::parser::Parser::parse_alter_function` · sqlparser 0.62.0

```rust
fn parse_alter_function(&mut self, kind: AlterFunctionKind) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:10956`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an `ALTER FUNCTION` or `ALTER AGGREGATE` statement.

<a id="op-dc51a50f5f838ea7824a8862"></a>
## parse_alter_operator

`function` · `sqlparser::parser::Parser::parse_alter_operator` · sqlparser 0.62.0

```rust
fn parse_alter_operator(&mut self) -> Result<AlterOperator, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:11151`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a [Statement::AlterOperator](../operations/sqlparser.ast.Statement.md#op-bb5e5f50a7484323de74c575)

[PostgreSQL Documentation](https://www.postgresql.org/docs/current/sql-alteroperator.html)

<a id="op-a73c53971071ceca61c3befb"></a>
## parse_alter_operator_class

`function` · `sqlparser::parser::Parser::parse_alter_operator_class` · sqlparser 0.62.0

```rust
fn parse_alter_operator_class(&mut self) -> Result<AlterOperatorClass, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:11426`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an `ALTER OPERATOR CLASS` statement.

Handles operations like `RENAME TO`, `OWNER TO`, and `SET SCHEMA`.

<a id="op-01d14fb66cf104569541f620"></a>
## parse_alter_operator_family

`function` · `sqlparser::parser::Parser::parse_alter_operator_family` · sqlparser 0.62.0

```rust
fn parse_alter_operator_family(&mut self) -> Result<AlterOperatorFamily, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:11389`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a [Statement::AlterOperatorFamily](../operations/sqlparser.ast.Statement.md#op-b408103e53cc7018f2046cbb)
See <https://www.postgresql.org/docs/current/sql-alteropfamily.html>

<a id="op-0863bfa55e9d4362f0c3568b"></a>
## parse_alter_policy

`function` · `sqlparser::parser::Parser::parse_alter_policy` · sqlparser 0.62.0

```rust
fn parse_alter_policy(&mut self) -> Result<AlterPolicy, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "super::Parser"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 1], "end": [512, 2], "filename": "src/parser/alter.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/alter.rs:57`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse ALTER POLICY statement
```sql
ALTER POLICY policy_name ON table_name [ RENAME TO new_name ]
or
ALTER POLICY policy_name ON table_name
[ TO { role_name | PUBLIC | CURRENT_ROLE | CURRENT_USER | SESSION_USER } [, ...] ]
[ USING ( using_expression ) ]
[ WITH CHECK ( check_expression ) ]
```

[PostgreSQL](https://www.postgresql.org/docs/current/sql-alterpolicy.html)

<a id="op-941cd9cfcaa957827018efe7"></a>
## parse_alter_role

`function` · `sqlparser::parser::Parser::parse_alter_role` · sqlparser 0.62.0

```rust
fn parse_alter_role(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "super::Parser"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 1], "end": [512, 2], "filename": "src/parser/alter.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/alter.rs:34`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse `ALTER ROLE` statement

<a id="op-01956fc02f5f13498dce06a4"></a>
## parse_alter_schema

`function` · `sqlparser::parser::Parser::parse_alter_schema` · sqlparser 0.62.0

```rust
fn parse_alter_schema(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:11457`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an `ALTER SCHEMA` statement.

Supports operations such as setting options, renaming, adding/dropping replicas, and changing owner.

<a id="op-0166ab927c8ef4336b902057"></a>
## parse_alter_table

`function` · `sqlparser::parser::Parser::parse_alter_table` · sqlparser 0.62.0

```rust
fn parse_alter_table(&mut self, iceberg: bool) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:11011`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a [Statement::AlterTable](../operations/sqlparser.ast.Statement.md#op-ac3d08e4a9088c30c40aec02)

<a id="op-f13effea472846a69c35c636"></a>
## parse_alter_table_add_projection

`function` · `sqlparser::parser::Parser::parse_alter_table_add_projection` · sqlparser 0.62.0

```rust
fn parse_alter_table_add_projection(&mut self) -> Result<AlterTableOperation, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:10188`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse `ALTER TABLE ... ADD PROJECTION ...` operation.

<a id="op-1664cc74be693914dff330a5"></a>
## parse_alter_table_operation

`function` · `sqlparser::parser::Parser::parse_alter_table_operation` · sqlparser 0.62.0

```rust
fn parse_alter_table_operation(&mut self) -> Result<AlterTableOperation, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:10212`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a single `ALTER TABLE` operation and return an `AlterTableOperation`.

<a id="op-155649393ce8deed4d33067b"></a>
## parse_alter_type

`function` · `sqlparser::parser::Parser::parse_alter_type` · sqlparser 0.62.0

```rust
fn parse_alter_type(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:11073`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a [Statement::AlterType](../operations/sqlparser.ast.Statement.md#op-92d99d94952bf4d680c4e886)

<a id="op-6a658fd687d5a7cba940f0eb"></a>
## parse_alter_user

`function` · `sqlparser::parser::Parser::parse_alter_user` · sqlparser 0.62.0

```rust
fn parse_alter_user(&mut self) -> Result<AlterUser, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "super::Parser"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 1], "end": [512, 2], "filename": "src/parser/alter.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/alter.rs:151`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an `ALTER USER` statement
```sql
ALTER USER [ IF EXISTS ] [ <name> ] [ OPTIONS ]
```

<a id="op-0aec14cc60f3960c8c1ec891"></a>
## parse_alter_view

`function` · `sqlparser::parser::Parser::parse_alter_view` · sqlparser 0.62.0

```rust
fn parse_alter_view(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:11055`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an `ALTER VIEW` statement.

<a id="op-f0b48b0ad96477aa164e144b"></a>
## parse_analyze

`function` · `sqlparser::parser::Parser::parse_analyze` · sqlparser 0.62.0

```rust
fn parse_analyze(&mut self) -> Result<Analyze, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:1238`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse `ANALYZE` statement.

<a id="op-d23185e5d0da98ea5d02b524"></a>
## parse_analyze_format

`function` · `sqlparser::parser::Parser::parse_analyze_format` · sqlparser 0.62.0

```rust
fn parse_analyze_format(&mut self) -> Result<AnalyzeFormat, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:6502`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an `ANALYZE FORMAT`.

<a id="op-ed9dbb54082f014426bb0151"></a>
## parse_array_expr

`function` · `sqlparser::parser::Parser::parse_array_expr` · sqlparser 0.62.0

```rust
fn parse_array_expr(&mut self, named: bool) -> Result<Expr, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:3037`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parses an array expression `[ex1, ex2, ..]`
if `named` is `true`, came from an expression like  `ARRAY[ex1, ex2]`

<a id="op-9f98c8cb276847d84b926a1a"></a>
## parse_as_query

`function` · `sqlparser::parser::Parser::parse_as_query` · sqlparser 0.62.0

```rust
fn parse_as_query(&mut self) -> Result<(bool, Box<Query>), ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:5384`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse 'AS' before as query,such as `WITH XXX AS SELECT XXX` oer `CACHE TABLE AS SELECT XXX`

<a id="op-cbc56d4741a30f9aca7a8a86"></a>
## parse_as_table

`function` · `sqlparser::parser::Parser::parse_as_table` · sqlparser 0.62.0

```rust
fn parse_as_table(&mut self) -> Result<Table, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:15093`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse `CREATE TABLE x AS TABLE y`

<a id="op-266741fa07dbf30bc747d04f"></a>
## parse_asc_desc

`function` · `sqlparser::parser::Parser::parse_asc_desc` · sqlparser 0.62.0

```rust
fn parse_asc_desc(&mut self) -> Option<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:18771`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse ASC or DESC, returns an Option with true if ASC, false of DESC or `None` if none of
them.

<a id="op-5539db62ad2dfc72150e81ec"></a>
## parse_assert

`function` · `sqlparser::parser::Parser::parse_assert` · sqlparser 0.62.0

```rust
fn parse_assert(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:1430`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse `ASSERT` statement.

<a id="op-eae30c87d9b38f6c6029d339"></a>
## parse_assignment

`function` · `sqlparser::parser::Parser::parse_assignment` · sqlparser 0.62.0

```rust
fn parse_assignment(&mut self) -> Result<Assignment, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:18253`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a `var = expr` assignment, used in an UPDATE statement

<a id="op-9a0ff940153004d24eba28fe"></a>
## parse_assignment_target

`function` · `sqlparser::parser::Parser::parse_assignment_target` · sqlparser 0.62.0

```rust
fn parse_assignment_target(&mut self) -> Result<AssignmentTarget, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:18261`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse the left-hand side of an assignment, used in an UPDATE statement

<a id="op-336f8f032d4e722a262c67a5"></a>
## parse_attach_database

`function` · `sqlparser::parser::Parser::parse_attach_database` · sqlparser 0.62.0

```rust
fn parse_attach_database(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:1225`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse `ATTACH DATABASE` statement.

<a id="op-707584646c96473edf5ac76d"></a>
## parse_attach_duckdb_database

`function` · `sqlparser::parser::Parser::parse_attach_duckdb_database` · sqlparser 0.62.0

```rust
fn parse_attach_duckdb_database(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:1192`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse `ATTACH DUCKDB DATABASE` statement.

<a id="op-3a3b6d4d9c58737bd2a29a5c"></a>
## parse_attach_duckdb_database_options

`function` · `sqlparser::parser::Parser::parse_attach_duckdb_database_options` · sqlparser 0.62.0

```rust
fn parse_attach_duckdb_database_options(&mut self) -> Result<Vec<AttachDuckDBDatabaseOption>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:1155`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse options for `ATTACH DUCKDB DATABASE` statement.

<a id="op-426539767765cb73eee59ba5"></a>
## parse_begin

`function` · `sqlparser::parser::Parser::parse_begin` · sqlparser 0.62.0

```rust
fn parse_begin(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:19137`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a 'BEGIN' statement

<a id="op-2384d016596c96f61f923fc0"></a>
## parse_begin_exception_end

`function` · `sqlparser::parser::Parser::parse_begin_exception_end` · sqlparser 0.62.0

```rust
fn parse_begin_exception_end(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:19159`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a 'BEGIN ... EXCEPTION ... END' block

<a id="op-70ed1272fcfd420f66861ce3"></a>
## parse_between

`function` · `sqlparser::parser::Parser::parse_between` · sqlparser 0.62.0

```rust
fn parse_between(&mut self, expr: Expr, negated: bool) -> Result<Expr, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4325`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parses `BETWEEN <low> AND <high>`, assuming the `BETWEEN` keyword was already consumed.

<a id="op-df90581e48d25d613f6e6545"></a>
## parse_big_query_declare

`function` · `sqlparser::parser::Parser::parse_big_query_declare` · sqlparser 0.62.0

```rust
fn parse_big_query_declare(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:7661`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a [BigQuery] `DECLARE` statement.

Syntax:
```text
DECLARE variable_name[, ...] [{ <variable_type> | <DEFAULT expression> }];
```
[BigQuery]: https://cloud.google.com/bigquery/docs/reference/standard-sql/procedural-language#declare

<a id="op-270e432682169a3f4a96d420"></a>
## parse_binary_length

`function` · `sqlparser::parser::Parser::parse_binary_length` · sqlparser 0.62.0

```rust
fn parse_binary_length(&mut self) -> Result<BinaryLength, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:13734`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a binary length specification, returning `BinaryLength`.

<a id="op-4dcbb11109bfe9294d8ba512"></a>
## parse_cache_table

`function` · `sqlparser::parser::Parser::parse_cache_table` · sqlparser 0.62.0

```rust
fn parse_cache_table(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:5308`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a CACHE TABLE statement

<a id="op-21f677def8e06de5e2123d41"></a>
## parse_call

`function` · `sqlparser::parser::Parser::parse_call` · sqlparser 0.62.0

```rust
fn parse_call(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:11497`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a `CALL procedure_name(arg1, arg2, ...)`
or `CALL procedure_name` statement

<a id="op-849432d31cc508a78d3bfef9"></a>
## parse_case_expr

`function` · `sqlparser::parser::Parser::parse_case_expr` · sqlparser 0.62.0

```rust
fn parse_case_expr(&mut self) -> Result<Expr, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:2686`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a `CASE` expression and return an [`Expr::Case`](../operations/sqlparser.ast.Expr.md#op-87c61b34b38c2b146d245779).

<a id="op-4c5cdaab2f9d572898170254"></a>
## parse_case_stmt

`function` · `sqlparser::parser::Parser::parse_case_stmt` · sqlparser 0.62.0

```rust
fn parse_case_stmt(&mut self) -> Result<CaseStatement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:735`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a `CASE` statement.

See [Statement::Case](../operations/sqlparser.ast.Statement.md#op-fa906e2a69b8ff6269f8f9eb)

<a id="op-ec89df716eb68d60d9360977"></a>
## parse_cast_expr

`function` · `sqlparser::parser::Parser::parse_cast_expr` · sqlparser 0.62.0

```rust
fn parse_cast_expr(&mut self, kind: CastKind) -> Result<Expr, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:2803`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a SQL CAST function e.g. `CAST(expr AS FLOAT)`

<a id="op-98158689896c9664cb5a7686"></a>
## parse_ceil_floor_expr

`function` · `sqlparser::parser::Parser::parse_ceil_floor_expr` · sqlparser 0.62.0

```rust
fn parse_ceil_floor_expr(&mut self, is_ceil: bool) -> Result<Expr, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:2857`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a `CEIL` or `FLOOR` expression.

<a id="op-d82969b5a2a7d9e357d1c821"></a>
## parse_character_length

`function` · `sqlparser::parser::Parser::parse_character_length` · sqlparser 0.62.0

```rust
fn parse_character_length(&mut self) -> Result<CharacterLength, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:13718`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a character length, handling `MAX` or integer lengths with optional units.

<a id="op-fa75e5499b579a388c1d6d3e"></a>
## parse_close

`function` · `sqlparser::parser::Parser::parse_close` · sqlparser 0.62.0

```rust
fn parse_close(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:11598`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a `CLOSE` cursor statement.

<a id="op-68a3ebc7f9b697ee5b6dd99d"></a>
## parse_column_def

`function` · `sqlparser::parser::Parser::parse_column_def` · sqlparser 0.62.0

```rust
fn parse_column_def(&mut self) -> Result<ColumnDef, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:9173`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse column definition.

<a id="op-a4c110fed02d26115485f2bf"></a>
## parse_columns

`function` · `sqlparser::parser::Parser::parse_columns` · sqlparser 0.62.0

```rust
fn parse_columns(&mut self) -> Result<(Vec<ColumnDef>, Vec<TableConstraint>), ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:9105`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse columns and constraints.

<a id="op-9013427baf4386895f304933"></a>
## parse_comma_separated

`function` · `sqlparser::parser::Parser::parse_comma_separated` · sqlparser 0.62.0

```rust
fn parse_comma_separated<T, F>(&mut self, f: F) -> Result<Vec<T>, ParserError> where F: FnMut(&mut Parser<'a>) -> Result<T, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4917`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a comma-separated list of 1+ items accepted by `F`

<a id="op-5dd7d194e47fdb7b3bc155dd"></a>
## parse_comma_separated0

`function` · `sqlparser::parser::Parser::parse_comma_separated0` · sqlparser 0.62.0

```rust
fn parse_comma_separated0<T, F>(&mut self, f: F, end_token: Token) -> Result<Vec<T>, ParserError> where F: FnMut(&mut Parser<'a>) -> Result<T, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:5002`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a comma-separated list of 0+ items accepted by `F`
* `end_token` - expected end token for the closure (e.g. [Token::RParen](../operations/sqlparser.tokenizer.Token.md#op-deb5d817b2ba79b6c29931ff), [Token::RBrace](../operations/sqlparser.tokenizer.Token.md#op-a00c4648579ebb835a859713) ...)

<a id="op-405b97635c109d2dc10f1d40"></a>
## parse_comment

`function` · `sqlparser::parser::Parser::parse_comment` · sqlparser 0.62.0

```rust
fn parse_comment(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:898`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a COMMENT statement.

See [Statement::Comment](../operations/sqlparser.ast.Statement.md#op-4fd39b7f4e0f72ac2bdc2ce6)

<a id="op-377b6635bc65c7aab250891f"></a>
## parse_comment_value

`function` · `sqlparser::parser::Parser::parse_comment_value` · sqlparser 0.62.0

```rust
fn parse_comment_value(&mut self) -> Result<String, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:9068`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse comment value.

<a id="op-c570f07a7745dc8c4c82d8b9"></a>
## parse_commit

`function` · `sqlparser::parser::Parser::parse_commit` · sqlparser 0.62.0

```rust
fn parse_commit(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:19262`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a 'COMMIT' statement

<a id="op-118d366a72791ba536dc5f2d"></a>
## parse_commit_rollback_chain

`function` · `sqlparser::parser::Parser::parse_commit_rollback_chain` · sqlparser 0.62.0

```rust
fn parse_commit_rollback_chain(&mut self) -> Result<bool, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:19279`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an optional `AND [NO] CHAIN` clause for `COMMIT` and `ROLLBACK` statements

<a id="op-963f95d739fc3dcbe28ceaa9"></a>
## parse_compound_expr

`function` · `sqlparser::parser::Parser::parse_compound_expr` · sqlparser 0.62.0

```rust
fn parse_compound_expr(&mut self, root: Expr, chain: Vec<AccessExpr>) -> Result<Expr, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:1991`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Try to parse an [Expr::CompoundFieldAccess](../operations/sqlparser.ast.Expr.md#op-caa40f99aa3a3865bf4efc4e) like `a.b.c` or `a.b[1].c`.
If all the fields are `Expr::Identifier`s, return an [Expr::CompoundIdentifier](../operations/sqlparser.ast.Expr.md#op-e08e68e59c071d6335d7ce14) instead.
If only the root exists, return the root.
Parses compound expressions which may be delimited by period
or bracket notation.
For example: `a.b.c`, `a.b[1]`.

<a id="op-4669b7474af37b11475cd7fe"></a>
## parse_constraint_characteristics

`function` · `sqlparser::parser::Parser::parse_constraint_characteristics` · sqlparser 0.62.0

```rust
fn parse_constraint_characteristics(&mut self) -> Result<Option<ConstraintCharacteristics>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:9670`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse optional constraint characteristics such as `DEFERRABLE`, `INITIALLY` and `ENFORCED`.

<a id="op-65375f7b8bbeb93d08a68ca3"></a>
## parse_convert_expr

`function` · `sqlparser::parser::Parser::parse_convert_expr` · sqlparser 0.62.0

```rust
fn parse_convert_expr(&mut self, is_try: bool) -> Result<Expr, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:2766`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a SQL CONVERT function:
 - `CONVERT('héhé' USING utf8mb4)` (MySQL)
 - `CONVERT('héhé', CHAR CHARACTER SET utf8mb4)` (MySQL)
 - `CONVERT(DECIMAL(10, 5), 42)` (MSSQL) - the type comes first

<a id="op-f08c4ff9841d1a6f16bbff5f"></a>
## parse_copy

`function` · `sqlparser::parser::Parser::parse_copy` · sqlparser 0.62.0

```rust
fn parse_copy(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:11522`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a copy statement

<a id="op-c3d57ae3abea45fb84fef549"></a>
## parse_create

`function` · `sqlparser::parser::Parser::parse_create` · sqlparser 0.62.0

```rust
fn parse_create(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:5124`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a SQL CREATE statement

<a id="op-a4e429dbedb066f28c8e8b28"></a>
## parse_create_collation

`function` · `sqlparser::parser::Parser::parse_create_collation` · sqlparser 0.62.0

```rust
fn parse_create_collation(&mut self) -> Result<CreateCollation, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:8155`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a PostgreSQL-specific [Statement::CreateCollation](../operations/sqlparser.ast.Statement.md#op-e9aab561217ad70fe591afd2) statement.

<a id="op-0746ce429b4af775e3246850"></a>
## parse_create_connector

`function` · `sqlparser::parser::Parser::parse_create_connector` · sqlparser 0.62.0

```rust
fn parse_create_connector(&mut self) -> Result<CreateConnector, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:7038`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
CREATE CONNECTOR [IF NOT EXISTS] connector_name
[TYPE datasource_type]
[URL datasource_url]
[COMMENT connector_comment]
[WITH DCPROPERTIES(property_name=property_value, ...)]
```

[Hive Documentation](https://cwiki.apache.org/confluence/pages/viewpage.action?pageId=27362034#LanguageManualDDL-CreateDataConnectorCreateConnector)

<a id="op-4c00ee19d6d6a69192677a44"></a>
## parse_create_database

`function` · `sqlparser::parser::Parser::parse_create_database` · sqlparser 0.62.0

```rust
fn parse_create_database(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:5486`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a `CREATE DATABASE` statement.

<a id="op-06d1951b160811678132bec9"></a>
## parse_create_extension

`function` · `sqlparser::parser::Parser::parse_create_extension` · sqlparser 0.62.0

```rust
fn parse_create_extension(&mut self) -> Result<CreateExtension, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:8121`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a `CREATE EXTENSION` statement.

<a id="op-f4c1f8a9959b8a51c58fd598"></a>
## parse_create_external_table

`function` · `sqlparser::parser::Parser::parse_create_external_table` · sqlparser 0.62.0

```rust
fn parse_create_external_table(&mut self, or_replace: bool) -> Result<CreateTable, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:6394`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a `CREATE EXTERNAL TABLE` statement.

<a id="op-74bf430ab40215e43b8d23b6"></a>
## parse_create_function

`function` · `sqlparser::parser::Parser::parse_create_function` · sqlparser 0.62.0

```rust
fn parse_create_function(&mut self, or_alter: bool, or_replace: bool, temporary: bool) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:5584`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a `CREATE FUNCTION` statement.

<a id="op-cba09e414d0bc769f281e30d"></a>
## parse_create_index

`function` · `sqlparser::parser::Parser::parse_create_index` · sqlparser 0.62.0

```rust
fn parse_create_index(&mut self, unique: bool) -> Result<CreateIndex, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:8029`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a `CREATE INDEX` statement.

<a id="op-f0dfc9d9acfe04aacaf0455b"></a>
## parse_create_index_expr

`function` · `sqlparser::parser::Parser::parse_create_index_expr` · sqlparser 0.62.0

```rust
fn parse_create_index_expr(&mut self) -> Result<IndexColumn, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:18788`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an [IndexColumn](../operations/sqlparser.ast.ddl.IndexColumn.md#op-7b594611ac7779eb0ddff495).

<a id="op-55a551aee6176d821a71cdf7"></a>
## parse_create_macro

`function` · `sqlparser::parser::Parser::parse_create_macro` · sqlparser 0.62.0

```rust
fn parse_create_macro(&mut self, or_replace: bool, temporary: bool) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:6346`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a `CREATE MACRO` statement.

<a id="op-58a8abc5337aaff66529a5b2"></a>
## parse_create_operator

`function` · `sqlparser::parser::Parser::parse_create_operator` · sqlparser 0.62.0

```rust
fn parse_create_operator(&mut self) -> Result<CreateOperator, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:7093`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a [Statement::CreateOperator](../operations/sqlparser.ast.Statement.md#op-9b9a66342ea929c87d229082)

[PostgreSQL Documentation](https://www.postgresql.org/docs/current/sql-createoperator.html)

<a id="op-3537b2a158a64c10cd5a3379"></a>
## parse_create_operator_class

`function` · `sqlparser::parser::Parser::parse_create_operator_class` · sqlparser 0.62.0

```rust
fn parse_create_operator_class(&mut self) -> Result<CreateOperatorClass, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:7226`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a [Statement::CreateOperatorClass](../operations/sqlparser.ast.Statement.md#op-eec4fb913ffebe198a915599)

[PostgreSQL Documentation](https://www.postgresql.org/docs/current/sql-createopclass.html)

<a id="op-2f935c58f046bda05d0e6573"></a>
## parse_create_operator_family

`function` · `sqlparser::parser::Parser::parse_create_operator_family` · sqlparser 0.62.0

```rust
fn parse_create_operator_family(&mut self) -> Result<CreateOperatorFamily, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:7215`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a [Statement::CreateOperatorFamily](../operations/sqlparser.ast.Statement.md#op-2a94caaa39492547e2fcdb44)

[PostgreSQL Documentation](https://www.postgresql.org/docs/current/sql-createopfamily.html)

<a id="op-144a265eca0f48b1846b5d0d"></a>
## parse_create_policy

`function` · `sqlparser::parser::Parser::parse_create_policy` · sqlparser 0.62.0

```rust
fn parse_create_policy(&mut self) -> Result<CreatePolicy, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:6953`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
    CREATE POLICY name ON table_name [ AS { PERMISSIVE | RESTRICTIVE } ]
    [ FOR { ALL | SELECT | INSERT | UPDATE | DELETE } ]
    [ TO { role_name | PUBLIC | CURRENT_USER | CURRENT_ROLE | SESSION_USER } [, ...] ]
    [ USING ( using_expression ) ]
    [ WITH CHECK ( with_check_expression ) ]
```

[PostgreSQL Documentation](https://www.postgresql.org/docs/current/sql-createpolicy.html)

<a id="op-d3acb4bca1b422cd321a00fb"></a>
## parse_create_procedure

`function` · `sqlparser::parser::Parser::parse_create_procedure` · sqlparser 0.62.0

```rust
fn parse_create_procedure(&mut self, or_alter: bool) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:19796`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse `CREATE PROCEDURE` statement.

<a id="op-a5c765b114b0d53967d67545"></a>
## parse_create_role

`function` · `sqlparser::parser::Parser::parse_create_role` · sqlparser 0.62.0

```rust
fn parse_create_role(&mut self) -> Result<CreateRole, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:6670`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a `CREATE ROLE` statement.

<a id="op-3c7fcb2bb2b87cd334e6ae40"></a>
## parse_create_schema

`function` · `sqlparser::parser::Parser::parse_create_schema` · sqlparser 0.62.0

```rust
fn parse_create_schema(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:5429`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a `CREATE SCHEMA` statement.

<a id="op-0903108260b85f974fb9f5bf"></a>
## parse_create_secret

`function` · `sqlparser::parser::Parser::parse_create_secret` · sqlparser 0.62.0

```rust
fn parse_create_secret(&mut self, or_replace: bool, temporary: bool, persistent: bool) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:5249`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See [DuckDB Docs](https://duckdb.org/docs/sql/statements/create_secret.html) for more details.

<a id="op-ce8d80104b0850eee197f9d0"></a>
## parse_create_sequence

`function` · `sqlparser::parser::Parser::parse_create_sequence` · sqlparser 0.62.0

```rust
fn parse_create_sequence(&mut self, temporary: bool) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:19655`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
CREATE [ { TEMPORARY | TEMP } ] SEQUENCE [ IF NOT EXISTS ] <sequence_name>
```

See [Postgres docs](https://www.postgresql.org/docs/current/sql-createsequence.html) for more details.

<a id="op-3898afd5d8e83cf98e74b60d"></a>
## parse_create_snapshot_table

`function` · `sqlparser::parser::Parser::parse_create_snapshot_table` · sqlparser 0.62.0

```rust
fn parse_create_snapshot_table(&mut self) -> Result<CreateTable, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:6444`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse `CREATE SNAPSHOT TABLE` statement.

<https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#create_snapshot_table_statement>

<a id="op-ddc1be6afc4c1afc8877850a"></a>
## parse_create_table

`function` · `sqlparser::parser::Parser::parse_create_table` · sqlparser 0.62.0

```rust
fn parse_create_table(&mut self, or_replace: bool, temporary: bool, global: Option<bool>, transient: bool) -> Result<CreateTable, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:8467`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse `CREATE TABLE` statement.

<a id="op-e52cb6c03d17ef4fbfa17aa6"></a>
## parse_create_trigger

`function` · `sqlparser::parser::Parser::parse_create_trigger` · sqlparser 0.62.0

```rust
fn parse_create_trigger(&mut self, temporary: bool, or_alter: bool, or_replace: bool, is_constraint: bool) -> Result<CreateTrigger, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:6166`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a `CREATE TRIGGER` statement.

<a id="op-1ac66994f9fb833b96354dc3"></a>
## parse_create_type

`function` · `sqlparser::parser::Parser::parse_create_type` · sqlparser 0.62.0

```rust
fn parse_create_type(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:19855`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse `CREATE TYPE` statement.

<a id="op-22738dd504bf91407e49d7a2"></a>
## parse_create_type_enum

`function` · `sqlparser::parser::Parser::parse_create_type_enum` · sqlparser 0.62.0

```rust
fn parse_create_type_enum(&mut self, name: ObjectName) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:19939`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse remainder of `CREATE TYPE AS ENUM` statement (see [Statement::CreateType](../operations/sqlparser.ast.Statement.md#op-6f1984bca01130739d231819) and [Self::parse_create_type](../operations/sqlparser.parser.Parser.md#op-1ac66994f9fb833b96354dc3))

See [PostgreSQL](https://www.postgresql.org/docs/current/sql-createtype.html)

<a id="op-c58e5b47596b549d5f1933d4"></a>
## parse_create_view

`function` · `sqlparser::parser::Parser::parse_create_view` · sqlparser 0.62.0

```rust
fn parse_create_view(&mut self, or_alter: bool, or_replace: bool, temporary: bool, create_view_params: Option<CreateViewParams>) -> Result<CreateView, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:6517`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a `CREATE VIEW` statement.

<a id="op-7f720f259e91af52eb0478a8"></a>
## parse_create_virtual_table

`function` · `sqlparser::parser::Parser::parse_create_virtual_table` · sqlparser 0.62.0

```rust
fn parse_create_virtual_table(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:5409`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SQLite-specific `CREATE VIRTUAL TABLE`

<a id="op-0fe3a26833a571ad6231e5fb"></a>
## parse_cte

`function` · `sqlparser::parser::Parser::parse_cte` · sqlparser 0.62.0

```rust
fn parse_cte(&mut self) -> Result<Cte, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:14485`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a CTE (`alias [( col1, col2, ... )] [AS] (subquery)`)

<a id="op-54522f55383e1068609cc82a"></a>
## parse_data_type

`function` · `sqlparser::parser::Parser::parse_data_type` · sqlparser 0.62.0

```rust
fn parse_data_type(&mut self) -> Result<DataType, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:12286`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a SQL datatype (in the context of a CREATE TABLE statement for example)

<a id="op-d00e0c26bae39c74091289e4"></a>
## parse_date_time_field

`function` · `sqlparser::parser::Parser::parse_date_time_field` · sqlparser 0.62.0

```rust
fn parse_date_time_field(&mut self) -> Result<DateTimeField, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:3088`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a date/time field for `EXTRACT`, interval qualifiers, and ceil/floor operations.

`EXTRACT` supports a wider set of date/time fields than interval qualifiers,
so this function may need to be split in two.

See [`DateTimeField`](../operations/sqlparser.ast.value.DateTimeField.md#op-e1c76b31457ce16b158c0a10)

<a id="op-e21d11d4b0adfdc789628f74"></a>
## parse_datetime_64

`function` · `sqlparser::parser::Parser::parse_datetime_64` · sqlparser 0.62.0

```rust
fn parse_datetime_64(&mut self) -> Result<(u64, Option<String>), ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:13680`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse datetime64 [1]
Syntax
```sql
DateTime64(precision[, timezone])
```

[1]: https://clickhouse.com/docs/en/sql-reference/data-types/datetime64

<a id="op-2769d1485b082f602f6226a9"></a>
## parse_deallocate

`function` · `sqlparser::parser::Parser::parse_deallocate` · sqlparser 0.62.0

```rust
fn parse_deallocate(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:19368`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a SQL `DEALLOCATE` statement

<a id="op-35f590764a7f34ba6a058b7e"></a>
## parse_declare

`function` · `sqlparser::parser::Parser::parse_declare` · sqlparser 0.62.0

```rust
fn parse_declare(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:7588`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a `DECLARE` statement.

```sql
DECLARE name [ BINARY ] [ ASENSITIVE | INSENSITIVE ] [ [ NO ] SCROLL ]
    CURSOR [ { WITH | WITHOUT } HOLD ] FOR query
```

The syntax can vary significantly between warehouses. See the grammar
on the warehouse specific function in such cases.

<a id="op-446db2a98781bb8ac9370a17"></a>
## parse_delete

`function` · `sqlparser::parser::Parser::parse_delete` · sqlparser 0.62.0

```rust
fn parse_delete(&mut self, delete_token: TokenWithSpan) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:13853`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a `DELETE` statement and return `Statement::Delete`.

<a id="op-d25467e8bdd8ad453af06ab4"></a>
## parse_deny

`function` · `sqlparser::parser::Parser::parse_deny` · sqlparser 0.62.0

```rust
fn parse_deny(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:17765`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse [`Statement::Deny`](../operations/sqlparser.ast.Statement.md#op-f75c18e3651a8a998cb200af)

<a id="op-4c2fc571b0dbb84d1a4f892d"></a>
## parse_derived_table_factor

`function` · `sqlparser::parser::Parser::parse_derived_table_factor` · sqlparser 0.62.0

```rust
fn parse_derived_table_factor(&mut self, lateral: IsLateral) -> Result<TableFactor, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:16982`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a derived table factor (a parenthesized subquery), handling optional LATERAL.

<a id="op-5cb73c06840b40a9c2694335"></a>
## parse_detach_duckdb_database

`function` · `sqlparser::parser::Parser::parse_detach_duckdb_database` · sqlparser 0.62.0

```rust
fn parse_detach_duckdb_database(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:1213`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse `DETACH DUCKDB DATABASE` statement.

<a id="op-e846fdcaab15063fb372855d"></a>
## parse_discard

`function` · `sqlparser::parser::Parser::parse_discard` · sqlparser 0.62.0

```rust
fn parse_discard(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:8010`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a `DISCARD` statement.

<a id="op-9473ff3b0eeddb467dd9f9ec"></a>
## parse_drop

`function` · `sqlparser::parser::Parser::parse_drop` · sqlparser 0.62.0

```rust
fn parse_drop(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:7353`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a `DROP` statement.

<a id="op-223076894908790d1198f1ac"></a>
## parse_drop_extension

`function` · `sqlparser::parser::Parser::parse_drop_extension` · sqlparser 0.62.0

```rust
fn parse_drop_extension(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:8180`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a PostgreSQL-specific [Statement::DropExtension](../operations/sqlparser.ast.Statement.md#op-f5e3dc6b49db3cd6bbf1ff47) statement.

<a id="op-177111d769d1c18b58ca1122"></a>
## parse_drop_operator

`function` · `sqlparser::parser::Parser::parse_drop_operator` · sqlparser 0.62.0

```rust
fn parse_drop_operator(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:8200`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a[Statement::DropOperator](../operations/sqlparser.ast.Statement.md#op-751fbc98a8130398200fafec) statement.


<a id="op-56f9a75606cffdc6428946ac"></a>
## parse_drop_operator_class

`function` · `sqlparser::parser::Parser::parse_drop_operator_class` · sqlparser 0.62.0

```rust
fn parse_drop_operator_class(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:8258`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a [Statement::DropOperatorClass](../operations/sqlparser.ast.Statement.md#op-7ea95a0b0fdfa0420f395d4d)

[PostgreSQL Documentation](https://www.postgresql.org/docs/current/sql-dropopclass.html)

<a id="op-66e26e430bc9a380b8fc9e27"></a>
## parse_drop_operator_family

`function` · `sqlparser::parser::Parser::parse_drop_operator_family` · sqlparser 0.62.0

```rust
fn parse_drop_operator_family(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:8241`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a [Statement::DropOperatorFamily](../operations/sqlparser.ast.Statement.md#op-41986f97f33bde7fb9288349)

[PostgreSQL Documentation](https://www.postgresql.org/docs/current/sql-dropopfamily.html)

<a id="op-79ad612d7c89ceca6d9d5bcd"></a>
## parse_drop_trigger

`function` · `sqlparser::parser::Parser::parse_drop_trigger` · sqlparser 0.62.0

```rust
fn parse_drop_trigger(&mut self) -> Result<DropTrigger, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:6136`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse statements of the DropTrigger type such as:

```sql
DROP TRIGGER [ IF EXISTS ] name ON table_name [ CASCADE | RESTRICT ]
```

<a id="op-adf5f42275d4b355a0eb95f1"></a>
## parse_end

`function` · `sqlparser::parser::Parser::parse_end` · sqlparser 0.62.0

```rust
fn parse_end(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:19205`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an 'END' statement

<a id="op-4b118042fb4e6d834f920977"></a>
## parse_enum_values

`function` · `sqlparser::parser::Parser::parse_enum_values` · sqlparser 0.62.0

```rust
fn parse_enum_values(&mut self) -> Result<Vec<EnumMember>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:12268`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse parenthesized enum members, used with `ENUM(...)` type definitions.

<a id="op-1a78490ea197d021e840c4de"></a>
## parse_escape_char

`function` · `sqlparser::parser::Parser::parse_escape_char` · sqlparser 0.62.0

```rust
fn parse_escape_char(&mut self) -> Result<Option<ValueWithSpan>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4120`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse the `ESCAPE CHAR` portion of `LIKE`, `ILIKE`, and `SIMILAR TO`

<a id="op-eeb0f3d3bbfe3489038fecd6"></a>
## parse_exact_number_optional_precision_scale

`function` · `sqlparser::parser::Parser::parse_exact_number_optional_precision_scale` · sqlparser 0.62.0

```rust
fn parse_exact_number_optional_precision_scale(&mut self) -> Result<ExactNumberInfo, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:13761`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse exact-number precision/scale info like `(precision[, scale])` for decimal types.

<a id="op-c3de0c2ce33518c88cce2ee5"></a>
## parse_execute

`function` · `sqlparser::parser::Parser::parse_execute` · sqlparser 0.62.0

```rust
fn parse_execute(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:19375`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a SQL `EXECUTE` statement

<a id="op-2bcb365effdbddf4c76f9356"></a>
## parse_exists_expr

`function` · `sqlparser::parser::Parser::parse_exists_expr` · sqlparser 0.62.0

```rust
fn parse_exists_expr(&mut self, negated: bool) -> Result<Expr, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:2821`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a SQL EXISTS expression e.g. `WHERE EXISTS(SELECT ...)`.

<a id="op-c711ab1fbf9b923e118a3fc4"></a>
## parse_explain

`function` · `sqlparser::parser::Parser::parse_explain` · sqlparser 0.62.0

```rust
fn parse_explain(&mut self, describe_alias: DescribeAlias) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:13945`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an `EXPLAIN` statement, handling dialect-specific options and modifiers.

<a id="op-b80687a72fd6e2cc6bc7ee8b"></a>
## parse_expr

`function` · `sqlparser::parser::Parser::parse_expr` · sqlparser 0.62.0

```rust
fn parse_expr(&mut self) -> Result<Expr, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:1366`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a new expression.

<a id="op-eba46da2a548e3b275c7c952"></a>
## parse_expr_with_alias

`function` · `sqlparser::parser::Parser::parse_expr_with_alias` · sqlparser 0.62.0

```rust
fn parse_expr_with_alias(&mut self) -> Result<ExprWithAlias, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:17028`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parses an expression with an optional alias

Examples:

```sql
SUM(price) AS total_price
```
```sql
SUM(price)
```

Example
```
# use sqlparser::parser::{Parser, ParserError};
# use sqlparser::dialect::GenericDialect;
# fn main() ->Result<(), ParserError> {
let sql = r#"SUM("a") as "b""#;
let mut parser = Parser::new(&GenericDialect).try_with_sql(sql)?;
let expr_with_alias = parser.parse_expr_with_alias()?;
assert_eq!(Some("b".to_string()), expr_with_alias.alias.map(|x|x.value));
# Ok(())
# }

<a id="op-46927de5789147e08743a1d5"></a>
## parse_expr_with_alias_and_order_by

`function` · `sqlparser::parser::Parser::parse_expr_with_alias_and_order_by` · sqlparser 0.62.0

```rust
fn parse_expr_with_alias_and_order_by(&mut self) -> Result<ExprWithAliasAndOrderBy, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:1371`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse expression with optional alias and order by.

<a id="op-68a42ff67a0be6d3da532dc3"></a>
## parse_extract_expr

`function` · `sqlparser::parser::Parser::parse_extract_expr` · sqlparser 0.62.0

```rust
fn parse_extract_expr(&mut self) -> Result<Expr, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:2832`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a SQL `EXTRACT` expression e.g. `EXTRACT(YEAR FROM date)`.

<a id="op-345da7c847442e354bcb5f00"></a>
## parse_fetch

`function` · `sqlparser::parser::Parser::parse_fetch` · sqlparser 0.62.0

```rust
fn parse_fetch(&mut self) -> Result<Fetch, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:18958`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a FETCH clause

<a id="op-768f5b6c9e2a083f7bf46e31"></a>
## parse_fetch_statement

`function` · `sqlparser::parser::Parser::parse_fetch_statement` · sqlparser 0.62.0

```rust
fn parse_fetch_statement(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:7940`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse `FETCH [direction] { FROM | IN } cursor INTO target;` statement.

<a id="op-b012ba4ee70775eb0f756569"></a>
## parse_file_format

`function` · `sqlparser::parser::Parser::parse_file_format` · sqlparser 0.62.0

```rust
fn parse_file_format(&mut self) -> Result<FileFormat, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:6476`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a file format for external tables.

<a id="op-5d30964c53f3a082abd68757"></a>
## parse_flush

`function` · `sqlparser::parser::Parser::parse_flush` · sqlparser 0.62.0

```rust
fn parse_flush(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:975`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse `FLUSH` statement.

<a id="op-946dc0077a592a2385992084"></a>
## parse_for_clause

`function` · `sqlparser::parser::Parser::parse_for_clause` · sqlparser 0.62.0

```rust
fn parse_for_clause(&mut self) -> Result<Option<ForClause>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:14383`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a mssql `FOR [XML | JSON | BROWSE]` clause

<a id="op-da64b1a45709cef5c7e41573"></a>
## parse_for_json

`function` · `sqlparser::parser::Parser::parse_for_json` · sqlparser 0.62.0

```rust
fn parse_for_json(&mut self) -> Result<ForClause, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:14451`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a mssql `FOR JSON` clause

<a id="op-4eb2d3f165e3001ed605ed49"></a>
## parse_for_xml

`function` · `sqlparser::parser::Parser::parse_for_xml` · sqlparser 0.62.0

```rust
fn parse_for_xml(&mut self) -> Result<ForClause, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:14396`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a mssql `FOR XML` clause

<a id="op-0c7904c80dace412bb255d75"></a>
## parse_function

`function` · `sqlparser::parser::Parser::parse_function` · sqlparser 0.62.0

```rust
fn parse_function(&mut self, name: ObjectName) -> Result<Expr, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:2437`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a function call expression named by `name` and return it as an `Expr`.

<a id="op-645b1d309cc50ef5d8faec7a"></a>
## parse_function_args

`function` · `sqlparser::parser::Parser::parse_function_args` · sqlparser 0.62.0

```rust
fn parse_function_args(&mut self) -> Result<FunctionArg, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:18273`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a single function argument, handling named and unnamed variants.

<a id="op-825a5110f38ca4f65c6f0267"></a>
## parse_grant

`function` · `sqlparser::parser::Parser::parse_grant` · sqlparser 0.62.0

```rust
fn parse_grant(&mut self) -> Result<Grant, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:17173`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a GRANT statement.

<a id="op-212563d637640a36be59ad22"></a>
## parse_grant_deny_revoke_privileges_objects

`function` · `sqlparser::parser::Parser::parse_grant_deny_revoke_privileges_objects` · sqlparser 0.62.0

```rust
fn parse_grant_deny_revoke_privileges_objects(&mut self) -> Result<(Privileges, Option<GrantObjects>), ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:17283`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse privileges and optional target objects for GRANT/DENY/REVOKE statements.

<a id="op-5f66df5d0c79a03ca4418ea4"></a>
## parse_grant_permission

`function` · `sqlparser::parser::Parser::parse_grant_permission` · sqlparser 0.62.0

```rust
fn parse_grant_permission(&mut self) -> Result<Action, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:17497`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a single grantable permission/action (used within GRANT statements).

<a id="op-47f52f0bd4dbbc82adf46e6e"></a>
## parse_grantee_name

`function` · `sqlparser::parser::Parser::parse_grantee_name` · sqlparser 0.62.0

```rust
fn parse_grantee_name(&mut self) -> Result<GranteeName, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:17749`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a grantee name, possibly with a host qualifier (user@host).

<a id="op-01287433cd886f6740deb8d3"></a>
## parse_hive_distribution

`function` · `sqlparser::parser::Parser::parse_hive_distribution` · sqlparser 0.62.0

```rust
fn parse_hive_distribution(&mut self) -> Result<HiveDistributionStyle, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:8275`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse Hive distribution style.

TODO: Support parsing for `SKEWED` distribution style.

<a id="op-928da9a3695459041cd29e2c"></a>
## parse_hive_formats

`function` · `sqlparser::parser::Parser::parse_hive_formats` · sqlparser 0.62.0

```rust
fn parse_hive_formats(&mut self) -> Result<Option<HiveFormat>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:8305`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse Hive formats.

<a id="op-eea9db263e1be46ea152c0fd"></a>
## parse_identifier

`function` · `sqlparser::parser::Parser::parse_identifier` · sqlparser 0.62.0

```rust
fn parse_identifier(&mut self) -> Result<Ident, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:13347`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a simple one-word identifier (possibly quoted, possibly a keyword)

<a id="op-de52b722eb8bf63a96911843"></a>
## parse_identifier_with_alias

`function` · `sqlparser::parser::Parser::parse_identifier_with_alias` · sqlparser 0.62.0

```rust
fn parse_identifier_with_alias(&mut self) -> Result<IdentWithAlias, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:12777`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Strictly parse `identifier AS identifier`

<a id="op-83f087346353b5a34573c478"></a>
## parse_identifiers

`function` · `sqlparser::parser::Parser::parse_identifiers` · sqlparser 0.62.0

```rust
fn parse_identifiers(&mut self) -> Result<Vec<Ident>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:13238`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse identifiers

<a id="op-ef84b6596e95ca48f44f2763"></a>
## parse_if_stmt

`function` · `sqlparser::parser::Parser::parse_if_stmt` · sqlparser 0.62.0

```rust
fn parse_if_stmt(&mut self) -> Result<IfStatement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:772`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an `IF` statement.

See [Statement::If](../operations/sqlparser.ast.Statement.md#op-91544b25fd3e9a33e5de011c)

<a id="op-38803a504cf7af46261540fb"></a>
## parse_in

`function` · `sqlparser::parser::Parser::parse_in` · sqlparser 0.62.0

```rust
fn parse_in(&mut self, expr: Expr, negated: bool) -> Result<Expr, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4290`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parses the parens following the `[ NOT ] IN` operator.

<a id="op-e544ed2065f7bc99a24f73fb"></a>
## parse_index_options

`function` · `sqlparser::parser::Parser::parse_index_options` · sqlparser 0.62.0

```rust
fn parse_index_options(&mut self) -> Result<Vec<IndexOption>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:10063`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse zero or more index options and return them as a vector.

<a id="op-c1548986778a9cf0a2109ec1"></a>
## parse_index_type

`function` · `sqlparser::parser::Parser::parse_index_type` · sqlparser 0.62.0

```rust
fn parse_index_type(&mut self) -> Result<IndexType, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:9995`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an index type token (e.g. `BTREE`, `HASH`, or a custom identifier).

<a id="op-3b56462fb1b1eb28f08d65fc"></a>
## parse_index_type_display

`function` · `sqlparser::parser::Parser::parse_index_type_display` · sqlparser 0.62.0

```rust
fn parse_index_type_display(&mut self) -> KeyOrIndexDisplay
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:10040`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse optional `KEY` or `INDEX` display tokens used in index/constraint declarations.

<a id="op-8e71129d809ab9ad7ed4ece8"></a>
## parse_infix

`function` · `sqlparser::parser::Parser::parse_infix` · sqlparser 0.62.0

```rust
fn parse_infix(&mut self, expr: Expr, precedence: u8) -> Result<Expr, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:3733`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an operator following an expression

<a id="op-b651f2b28d470b70320b852e"></a>
## parse_input_format_clause

`function` · `sqlparser::parser::Parser::parse_input_format_clause` · sqlparser 0.62.0

```rust
fn parse_input_format_clause(&mut self) -> Result<InputFormatClause, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:18081`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parses input format clause used for ClickHouse.

<https://clickhouse.com/docs/en/interfaces/formats>

<a id="op-8fd098b3d8ed34eb9c0ede2d"></a>
## parse_insert

`function` · `sqlparser::parser::Parser::parse_insert` · sqlparser 0.62.0

```rust
fn parse_insert(&mut self, insert_token: TokenWithSpan) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:17852`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an INSERT statement

<a id="op-3bdd53ad2e31d5da3cd2ef3d"></a>
## parse_insert_partition

`function` · `sqlparser::parser::Parser::parse_insert_partition` · sqlparser 0.62.0

```rust
fn parse_insert_partition(&mut self) -> Result<Option<Vec<Expr>>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:18152`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an optional `PARTITION (...)` clause for INSERT statements.

<a id="op-114b5f08d6b2dc4238617747"></a>
## parse_install

`function` · `sqlparser::parser::Parser::parse_install` · sqlparser 0.62.0

```rust
fn parse_install(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:19544`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`INSTALL [extension_name]`

<a id="op-668f225dbcc8514c85d8849a"></a>
## parse_interpolation

`function` · `sqlparser::parser::Parser::parse_interpolation` · sqlparser 0.62.0

```rust
fn parse_interpolation(&mut self) -> Result<InterpolateExpr, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:18898`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a INTERPOLATE expression (ClickHouse dialect)

<a id="op-b25c654b3d8349fca7e4f59d"></a>
## parse_interpolations

`function` · `sqlparser::parser::Parser::parse_interpolations` · sqlparser 0.62.0

```rust
fn parse_interpolations(&mut self) -> Result<Option<Interpolate>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:18878`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a set of comma separated INTERPOLATE expressions (ClickHouse dialect)
that follow the INTERPOLATE keyword in an ORDER BY clause with the WITH FILL modifier

<a id="op-2a4caa6dd6a3b4e0795971ba"></a>
## parse_interval

`function` · `sqlparser::parser::Parser::parse_interval` · sqlparser 0.62.0

```rust
fn parse_interval(&mut self) -> Result<Expr, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:3275`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an `INTERVAL` expression.

Some syntactically valid intervals:

```sql
  1. INTERVAL '1' DAY
  2. INTERVAL '1-1' YEAR TO MONTH
  3. INTERVAL '1' SECOND
  4. INTERVAL '1:1:1.1' HOUR (5) TO SECOND (5)
  5. INTERVAL '1.1' SECOND (2, 2)
  6. INTERVAL '1:1' HOUR (5) TO MINUTE (5)
  7. (MySql & BigQuery only): INTERVAL 1 DAY
```

Note that we do not currently attempt to parse the quoted value.

<a id="op-f23da49e495bee26f366b888"></a>
## parse_join_constraint

`function` · `sqlparser::parser::Parser::parse_join_constraint` · sqlparser 0.62.0

```rust
fn parse_join_constraint(&mut self, natural: bool) -> Result<JoinConstraint, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:17157`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a JOIN constraint (`NATURAL`, `ON <expr>`, `USING (...)`, or no constraint).

<a id="op-1228bf156a71666ddb7a492e"></a>
## parse_json_table_column_def

`function` · `sqlparser::parser::Parser::parse_json_table_column_def` · sqlparser 0.62.0

```rust
fn parse_json_table_column_def(&mut self) -> Result<JsonTableColumn, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:16895`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parses MySQL's JSON_TABLE column definition.
For example: `id INT EXISTS PATH '$' DEFAULT '0' ON EMPTY ERROR ON ERROR`

<a id="op-d211019f401d93ba341f7373"></a>
## parse_keyword

`function` · `sqlparser::parser::Parser::parse_keyword` · sqlparser 0.62.0

```rust
fn parse_keyword(&mut self, expected: Keyword) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4611`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

If the current token is the `expected` keyword, consume it and returns
true. Otherwise, no tokens are consumed and returns false.

<a id="op-10345553d6cca7af8e46f564"></a>
## parse_keyword_separated

`function` · `sqlparser::parser::Parser::parse_keyword_separated` · sqlparser 0.62.0

```rust
fn parse_keyword_separated<T, F>(&mut self, keyword: Keyword, f: F) -> Result<Vec<T>, ParserError> where F: FnMut(&mut Parser<'a>) -> Result<T, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4971`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a keyword-separated list of 1+ items accepted by `F`

<a id="op-def7531569813efc595d9375"></a>
## parse_keyword_with_tokens

`function` · `sqlparser::parser::Parser::parse_keyword_with_tokens` · sqlparser 0.62.0

```rust
fn parse_keyword_with_tokens(&mut self, expected: Keyword, tokens: &[Token]) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4635`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

If the current token is the `expected` keyword followed by
specified tokens, consume them and returns true.
Otherwise, no tokens are consumed and returns false.

Note that if the length of `tokens` is too long, this function will
not be efficient as it does a loop on the tokens with `peek_nth_token`
each time.

<a id="op-ec4b6a4918ba8d5799e56dd0"></a>
## parse_keywords

`function` · `sqlparser::parser::Parser::parse_keywords` · sqlparser 0.62.0

```rust
fn parse_keywords(&mut self, keywords: &[Keyword]) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4672`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

If the current and subsequent tokens exactly match the `keywords`
sequence, consume them and returns true. Otherwise, no tokens are
consumed and returns false

<a id="op-f3218ab527027413d8661f1b"></a>
## parse_kill

`function` · `sqlparser::parser::Parser::parse_kill` · sqlparser 0.62.0

```rust
fn parse_kill(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:13919`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a `KILL` statement, optionally specifying `CONNECTION`, `QUERY`, or `MUTATION`.
KILL [CONNECTION | QUERY | MUTATION] processlist_id

<a id="op-7a8cc447e4c5211940a3fa6f"></a>
## parse_limit

`function` · `sqlparser::parser::Parser::parse_limit` · sqlparser 0.62.0

```rust
fn parse_limit(&mut self) -> Result<Option<Expr>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:18936`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a LIMIT clause

<a id="op-c386c8589d1df4d02c96438f"></a>
## parse_listagg_on_overflow

`function` · `sqlparser::parser::Parser::parse_listagg_on_overflow` · sqlparser 0.62.0

```rust
fn parse_listagg_on_overflow(&mut self) -> Result<Option<ListAggOnOverflow>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:3046`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse the `ON OVERFLOW` clause for `LISTAGG`.

See [`ListAggOnOverflow`](../operations/sqlparser.ast.ListAggOnOverflow.md#op-4985ee1d185da88b7520ca0e)

<a id="op-abc57c8ebaa28b34577eec04"></a>
## parse_listen

`function` · `sqlparser::parser::Parser::parse_listen` · sqlparser 0.62.0

```rust
fn parse_listen(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:1456`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse `LISTEN` statement.

<a id="op-b2fd8c0a77fec602583efca4"></a>
## parse_literal_string

`function` · `sqlparser::parser::Parser::parse_literal_string` · sqlparser 0.62.0

```rust
fn parse_literal_string(&mut self) -> Result<String, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:12213`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a literal string

<a id="op-31d44f3148918491cb265988"></a>
## parse_literal_uint

`function` · `sqlparser::parser::Parser::parse_literal_uint` · sqlparser 0.62.0

```rust
fn parse_literal_uint(&mut self) -> Result<u64, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:12176`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an unsigned literal integer/long

<a id="op-717fd735f0a4634644b2205f"></a>
## parse_load

`function` · `sqlparser::parser::Parser::parse_load` · sqlparser 0.62.0

```rust
fn parse_load(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:19551`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a SQL LOAD statement

<a id="op-60878fe4b046cc43ae523232"></a>
## parse_load_data_table_format

`function` · `sqlparser::parser::Parser::parse_load_data_table_format` · sqlparser 0.62.0

```rust
fn parse_load_data_table_format(&mut self) -> Result<Option<HiveLoadDataFormat>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:18164`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse optional Hive `INPUTFORMAT ... SERDE ...` clause used by LOAD DATA.

<a id="op-0368155da9fbcc30080bf42f"></a>
## parse_lock

`function` · `sqlparser::parser::Parser::parse_lock` · sqlparser 0.62.0

```rust
fn parse_lock(&mut self) -> Result<LockClause, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:18987`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a FOR UPDATE/FOR SHARE clause

<a id="op-0565d872cd7a059b0a1f5aa7"></a>
## parse_lock_statement

`function` · `sqlparser::parser::Parser::parse_lock_statement` · sqlparser 0.62.0

```rust
fn parse_lock_statement(&mut self) -> Result<Lock, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:19015`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a PostgreSQL `LOCK` statement.

<a id="op-3fa7c00f0000e5dd68d15d86"></a>
## parse_match_against

`function` · `sqlparser::parser::Parser::parse_match_against` · sqlparser 0.62.0

```rust
fn parse_match_against(&mut self) -> Result<Expr, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:3216`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parses fulltext expressions [`sqlparser::ast::Expr::MatchAgainst`](../operations/sqlparser.ast.Expr.md#op-e3bc252fa8223d32bebe3515)

# Errors
This method will raise an error if the column list is empty or with invalid identifiers,
the match expression is not a literal string, or if the search modifier is not valid.

<a id="op-2fe94f7d93f0209917ae8067"></a>
## parse_match_kind

`function` · `sqlparser::parser::Parser::parse_match_kind` · sqlparser 0.62.0

```rust
fn parse_match_kind(&mut self) -> Result<ConstraintReferenceMatchKind, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:9642`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a `MATCH` kind for constraint references: `FULL`, `PARTIAL`, or `SIMPLE`.

<a id="op-ed8c0e6d96ecb907c0b17df1"></a>
## parse_merge

`function` · `sqlparser::parser::Parser::parse_merge` · sqlparser 0.62.0

```rust
fn parse_merge(&mut self, merge_token: TokenWithSpan) -> Result<Merge, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "super::Parser"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 1], "end": [261, 2], "filename": "src/parser/merge.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/merge.rs:45`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a `MERGE` statement

<a id="op-0b0d667acd739b930d15f306"></a>
## parse_msck

`function` · `sqlparser::parser::Parser::parse_msck` · sqlparser 0.62.0

```rust
fn parse_msck(&mut self) -> Result<Msck, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:1066`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse `MSCK` statement.

<a id="op-5b06960217598d0c7f33bd4a"></a>
## parse_mssql_declare

`function` · `sqlparser::parser::Parser::parse_mssql_declare` · sqlparser 0.62.0

```rust
fn parse_mssql_declare(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:7824`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a [MsSql] `DECLARE` statement.

Syntax:
```text
DECLARE
```
[MsSql]: https://learn.microsoft.com/en-us/sql/t-sql/language-elements/declare-local-variable-transact-sql?view=sql-server-ver16

<a id="op-d9947577a0d54076a9d9f666"></a>
## parse_mssql_declare_stmt

`function` · `sqlparser::parser::Parser::parse_mssql_declare_stmt` · sqlparser 0.62.0

```rust
fn parse_mssql_declare_stmt(&mut self) -> Result<Declare, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:7840`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse the body of a [MsSql] `DECLARE`statement.

Syntax:
```text
```
[MsSql]: https://learn.microsoft.com/en-us/sql/t-sql/language-elements/declare-local-variable-transact-sql?view=sql-server-ver16

<a id="op-98fed0312f837fa5a8fdaa20"></a>
## parse_mssql_variable_declaration_expression

`function` · `sqlparser::parser::Parser::parse_mssql_variable_declaration_expression` · sqlparser 0.62.0

```rust
fn parse_mssql_variable_declaration_expression(&mut self) -> Result<Option<DeclareAssignment>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:7925`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parses the assigned expression in a variable declaration.

Syntax:
```text
[ = <expression>]
```

<a id="op-25e138f8206f152e6bd98065"></a>
## parse_multi_dim_subscript

`function` · `sqlparser::parser::Parser::parse_multi_dim_subscript` · sqlparser 0.62.0

```rust
fn parse_multi_dim_subscript(&mut self, chain: &mut Vec<AccessExpr>) -> Result<(), ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4204`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a multi-dimension array accessing like `[1:3][1][1]`

<a id="op-5cd0516038d3c54d933ceb8c"></a>
## parse_multipart_identifier

`function` · `sqlparser::parser::Parser::parse_multipart_identifier` · sqlparser 0.62.0

```rust
fn parse_multipart_identifier(&mut self) -> Result<Vec<Ident>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:13295`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse identifiers of form ident1[.identN]*

Similar in functionality to [parse_identifiers], with difference
being this function is much more strict about parsing a valid multipart identifier, not
allowing extraneous tokens to be parsed, otherwise it fails.

For example:

```rust
use sqlparser::ast::Ident;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

let dialect = GenericDialect {};
let expected = vec![Ident::new("one"), Ident::new("two")];

// expected usage
let sql = "one.two";
let mut parser = Parser::new(&dialect).try_with_sql(sql).unwrap();
let actual = parser.parse_multipart_identifier().unwrap();
assert_eq!(&actual, &expected);

// parse_identifiers is more loose on what it allows, parsing successfully
let sql = "one + two";
let mut parser = Parser::new(&dialect).try_with_sql(sql).unwrap();
let actual = parser.parse_identifiers().unwrap();
assert_eq!(&actual, &expected);

// expected to strictly fail due to + separator
let sql = "one + two";
let mut parser = Parser::new(&dialect).try_with_sql(sql).unwrap();
let actual = parser.parse_multipart_identifier().unwrap_err();
assert_eq!(
    actual.to_string(),
    "sql parser error: Unexpected token in identifier: +"
);
```

[parse_identifiers]: Parser::parse_identifiers

<a id="op-cf9ea0dd5db5d9f676c5338d"></a>
## parse_named_window

`function` · `sqlparser::parser::Parser::parse_named_window` · sqlparser 0.62.0

```rust
fn parse_named_window(&mut self) -> Result<NamedWindowDefinition, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:19780`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a named window definition.

<a id="op-fff08cc9fc01db0370f14052"></a>
## parse_not

`function` · `sqlparser::parser::Parser::parse_not` · sqlparser 0.62.0

```rust
fn parse_not(&mut self) -> Result<Expr, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:3164`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a `NOT` expression.

Represented in the AST as `Expr::UnaryOp` with `UnaryOperator::Not`.

<a id="op-0277b2dead9d7bf38340439a"></a>
## parse_notify

`function` · `sqlparser::parser::Parser::parse_notify` · sqlparser 0.62.0

```rust
fn parse_notify(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:1478`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse `NOTIFY` statement.

<a id="op-b6c69cdded8c691b0a3183db"></a>
## parse_number

`function` · `sqlparser::parser::Parser::parse_number` · sqlparser 0.62.0

```rust
fn parse_number(&mut self) -> Result<Expr, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:12134`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a numeric literal as an expression. Returns a [`Expr::UnaryOp`](../operations/sqlparser.ast.Expr.md#op-9fbf0fee698d7f438622ad27) if the number is signed,
otherwise returns a [`Expr::Value`](../operations/sqlparser.ast.Expr.md#op-b308556454b939d0289fc387)

<a id="op-eb8dd8d6b8f941c48b998ad4"></a>
## parse_number_value

`function` · `sqlparser::parser::Parser::parse_number_value` · sqlparser 0.62.0

```rust
fn parse_number_value(&mut self) -> Result<ValueWithSpan, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:12120`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an unsigned numeric literal

<a id="op-4d89b74c036dbc979254b462"></a>
## parse_object_name

`function` · `sqlparser::parser::Parser::parse_object_name` · sqlparser 0.62.0

```rust
fn parse_object_name(&mut self, in_table_clause: bool) -> Result<ObjectName, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:13136`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a possibly qualified, possibly quoted identifier, e.g.
`foo` or `myschema."table"

The `in_table_clause` parameter indicates whether the object name is a table in a FROM, JOIN,
or similar table clause. Currently, this is used only to support unquoted hyphenated identifiers
in this context on BigQuery.

<a id="op-777d6eb08ffab761e26c9a82"></a>
## parse_offset

`function` · `sqlparser::parser::Parser::parse_offset` · sqlparser 0.62.0

```rust
fn parse_offset(&mut self) -> Result<Offset, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:18945`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an OFFSET clause

<a id="op-788c717f9fd180ed98e98a7d"></a>
## parse_one_of_keywords

`function` · `sqlparser::parser::Parser::parse_one_of_keywords` · sqlparser 0.62.0

```rust
fn parse_one_of_keywords(&mut self, keywords: &[Keyword]) -> Option<Keyword>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4710`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

If the current token is one of the given `keywords`, consume the token
and return the keyword that matches. Otherwise, no tokens are consumed
and returns [`None`].

Unresolved upstream links (retained, not inferred): ``None``.

<a id="op-50e74db0296782196194d368"></a>
## parse_openjson_table_column_def

`function` · `sqlparser::parser::Parser::parse_openjson_table_column_def` · sqlparser 0.62.0

```rust
fn parse_openjson_table_column_def(&mut self) -> Result<OpenJsonTableColumn, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:16944`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parses MSSQL's `OPENJSON WITH` column definition.

```sql
colName type [ column_path ] [ AS JSON ]
```

Reference: <https://learn.microsoft.com/en-us/sql/t-sql/functions/openjson-transact-sql?view=sql-server-ver16#syntax>

<a id="op-088c8428a7ceec199e38809c"></a>
## parse_optimize_table

`function` · `sqlparser::parser::Parser::parse_optimize_table` · sqlparser 0.62.0

```rust
fn parse_optimize_table(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:19592`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ClickHouse:
```sql
OPTIMIZE TABLE [db.]name [ON CLUSTER cluster] [PARTITION partition | PARTITION ID 'partition_id'] [FINAL] [DEDUPLICATE [BY expression]]
```
[ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/optimize)

Databricks:
```sql
OPTIMIZE table_name [WHERE predicate] [ZORDER BY (col_name1 [, ...])]
```
[Databricks](https://docs.databricks.com/en/sql/language-manual/delta-optimize.html)

<a id="op-2309f4a906b8d90fb003eef5"></a>
## parse_option_clustered

`function` · `sqlparser::parser::Parser::parse_option_clustered` · sqlparser 0.62.0

```rust
fn parse_option_clustered(&mut self) -> Result<SqlOption, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:10099`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a `CLUSTERED` table option (MSSQL-specific syntaxes supported).

<a id="op-a688a7173c1057dc12a3dde7"></a>
## parse_option_partition

`function` · `sqlparser::parser::Parser::parse_option_partition` · sqlparser 0.62.0

```rust
fn parse_option_partition(&mut self) -> Result<SqlOption, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:10136`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a `PARTITION(...) FOR VALUES(...)` table option.

<a id="op-7202008cf2446cedb4e1376f"></a>
## parse_optional_alias

`function` · `sqlparser::parser::Parser::parse_optional_alias` · sqlparser 0.62.0

```rust
fn parse_optional_alias(&mut self, reserved_kwds: &[Keyword]) -> Result<Option<Ident>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:12923`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Wrapper for parse_optional_alias_inner, left for backwards-compatibility
but new flows should use the context-specific methods such as `maybe_parse_select_item_alias`
and `maybe_parse_table_alias`.

<a id="op-8ec48b5a22bd86b4f1cee6eb"></a>
## parse_optional_args

`function` · `sqlparser::parser::Parser::parse_optional_args` · sqlparser 0.62.0

```rust
fn parse_optional_args(&mut self) -> Result<Vec<FunctionArg>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:18352`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an optional, comma-separated list of function arguments (consumes closing paren).

<a id="op-ca53d7ea0380e5c50548bb30"></a>
## parse_optional_binary_length

`function` · `sqlparser::parser::Parser::parse_optional_binary_length` · sqlparser 0.62.0

```rust
fn parse_optional_binary_length(&mut self) -> Result<Option<BinaryLength>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:13707`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an optional binary length specification like `(n)`.

<a id="op-6008142991fb6fb64ae6e369"></a>
## parse_optional_cast_format

`function` · `sqlparser::parser::Parser::parse_optional_cast_format` · sqlparser 0.62.0

```rust
fn parse_optional_cast_format(&mut self) -> Result<Option<CastFormat>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:2719`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an optional `FORMAT` clause for `CAST` expressions.

<a id="op-2628cb2277225ae7824aba9d"></a>
## parse_optional_character_length

`function` · `sqlparser::parser::Parser::parse_optional_character_length` · sqlparser 0.62.0

```rust
fn parse_optional_character_length(&mut self) -> Result<Option<CharacterLength>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:13694`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an optional character length specification `(n | MAX [CHARACTERS|OCTETS])`.

<a id="op-0e4ca7c6ac574291cc4a9094"></a>
## parse_optional_clustered_by

`function` · `sqlparser::parser::Parser::parse_optional_clustered_by` · sqlparser 0.62.0

```rust
fn parse_optional_clustered_by(&mut self) -> Result<Option<ClusteredBy>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:9590`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse optional `CLUSTERED BY` clause for Hive/Generic dialects.

<a id="op-b4c5839da244cb4d54f50821"></a>
## parse_optional_column_option

`function` · `sqlparser::parser::Parser::parse_optional_column_option` · sqlparser 0.62.0

```rust
fn parse_optional_column_option(&mut self) -> Result<Option<ColumnOption>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:9239`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse optional column option.

<a id="op-2452453d2fb52187bc6a4f3e"></a>
## parse_optional_create_function_using

`function` · `sqlparser::parser::Parser::parse_optional_create_function_using` · sqlparser 0.62.0

```rust
fn parse_optional_create_function_using(&mut self) -> Result<Option<CreateFunctionUsing>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:5561`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an optional `USING` clause for `CREATE FUNCTION`.

<a id="op-f2179baa0b4cec7e0f5dfa31"></a>
## parse_optional_group_by

`function` · `sqlparser::parser::Parser::parse_optional_group_by` · sqlparser 0.62.0

```rust
fn parse_optional_group_by(&mut self) -> Result<Option<GroupByExpr>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:12979`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an optional `GROUP BY` clause, returning `Some(GroupByExpr)` when present.

<a id="op-232ea8747a1e8d0fd914f40d"></a>
## parse_optional_ident

`function` · `sqlparser::parser::Parser::parse_optional_ident` · sqlparser 0.62.0

```rust
fn parse_optional_ident(&mut self) -> Result<Option<Ident>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:10034`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse `[ident]`, mostly `ident` is name, like:
`window_name`, `index_name`, ...
Parse an optional identifier, returning `Some(Ident)` if present.

<a id="op-117b7e67c47e8f1dc96bd1fc"></a>
## parse_optional_index_option

`function` · `sqlparser::parser::Parser::parse_optional_index_option` · sqlparser 0.62.0

```rust
fn parse_optional_index_option(&mut self) -> Result<Option<IndexOption>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:10051`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an optional index option such as `USING <type>` or `COMMENT <string>`.

<a id="op-d7cf18a615ac3521e6008feb"></a>
## parse_optional_inline_comment

`function` · `sqlparser::parser::Parser::parse_optional_inline_comment` · sqlparser 0.62.0

```rust
fn parse_optional_inline_comment(&mut self) -> Result<Option<CommentDef>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:9052`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse optional inline comment.

<a id="op-219bcd72c419603a4c192a5a"></a>
## parse_optional_order_by

`function` · `sqlparser::parser::Parser::parse_optional_order_by` · sqlparser 0.62.0

```rust
fn parse_optional_order_by(&mut self) -> Result<Option<OrderBy>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:13036`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an optional `ORDER BY` clause, returning `Some(OrderBy)` when present.

<a id="op-5cfb31213304f74b5d0f30da"></a>
## parse_optional_precision

`function` · `sqlparser::parser::Parser::parse_optional_precision` · sqlparser 0.62.0

```rust
fn parse_optional_precision(&mut self) -> Result<Option<u64>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:13584`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an optional precision `(n)` and return it as `Some(n)` when present.

<a id="op-afb09b3b9981e9f59b5ab1dd"></a>
## parse_optional_precision_scale

`function` · `sqlparser::parser::Parser::parse_optional_precision_scale` · sqlparser 0.62.0

```rust
fn parse_optional_precision_scale(&mut self) -> Result<(Option<u64>, Option<u64>), ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:13743`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an optional `(precision[, scale])` and return `(Option<precision>, Option<scale>)`.

<a id="op-aca7070668ae07621b0430ad"></a>
## parse_optional_procedure_parameters

`function` · `sqlparser::parser::Parser::parse_optional_procedure_parameters` · sqlparser 0.62.0

```rust
fn parse_optional_procedure_parameters(&mut self) -> Result<Option<Vec<ProcedureParam>>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:9079`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse optional procedure parameters.

<a id="op-f8fc5c16f9cdb7189fdaa6a7"></a>
## parse_optional_select_item_except

`function` · `sqlparser::parser::Parser::parse_optional_select_item_except` · sqlparser 0.62.0

```rust
fn parse_optional_select_item_except(&mut self) -> Result<Option<ExceptSelectItem>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:18682`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an [`Except`](ExceptSelectItem) information for wildcard select items.

If it is not possible to parse it, will return an option.

<a id="op-b368d2430897018793b9908c"></a>
## parse_optional_select_item_exclude

`function` · `sqlparser::parser::Parser::parse_optional_select_item_exclude` · sqlparser 0.62.0

```rust
fn parse_optional_select_item_exclude(&mut self) -> Result<Option<ExcludeSelectItem>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:18659`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an [`Exclude`](ExcludeSelectItem) information for wildcard select items.

If it is not possible to parse it, will return an option.

<a id="op-b2bbb410c0930160eabe915b"></a>
## parse_optional_select_item_ilike

`function` · `sqlparser::parser::Parser::parse_optional_select_item_ilike` · sqlparser 0.62.0

```rust
fn parse_optional_select_item_ilike(&mut self) -> Result<Option<IlikeSelectItem>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:18640`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an [`Ilike`](IlikeSelectItem) information for wildcard select items.

If it is not possible to parse it, will return an option.

<a id="op-22a379a885f143538515d14a"></a>
## parse_optional_select_item_rename

`function` · `sqlparser::parser::Parser::parse_optional_select_item_rename` · sqlparser 0.62.0

```rust
fn parse_optional_select_item_rename(&mut self) -> Result<Option<RenameSelectItem>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:18716`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a [`Rename`](RenameSelectItem) information for wildcard select items.

<a id="op-c9e7d570ea1e54d0590ef552"></a>
## parse_optional_select_item_replace

`function` · `sqlparser::parser::Parser::parse_optional_select_item_replace` · sqlparser 0.62.0

```rust
fn parse_optional_select_item_replace(&mut self) -> Result<Option<ReplaceSelectItem>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:18737`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a [`Replace`](ReplaceSelectItem) information for wildcard select items.

<a id="op-52799820b5f704a60376df80"></a>
## parse_optional_table_constraint

`function` · `sqlparser::parser::Parser::parse_optional_table_constraint` · sqlparser 0.62.0

```rust
fn parse_optional_table_constraint(&mut self) -> Result<Option<TableConstraint>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:9708`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an optional table constraint (e.g. `PRIMARY KEY`, `UNIQUE`, `FOREIGN KEY`, `CHECK`).

<a id="op-bb6626e3ded7faf53d0b1407"></a>
## parse_optional_time_zone

`function` · `sqlparser::parser::Parser::parse_optional_time_zone` · sqlparser 0.62.0

```rust
fn parse_optional_time_zone(&mut self) -> Result<Option<ValueWithSpan>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:2732`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an optional `AT TIME ZONE` clause.

<a id="op-9c43797dade71d2a25f7a1d4"></a>
## parse_optional_type_modifiers

`function` · `sqlparser::parser::Parser::parse_optional_type_modifiers` · sqlparser 0.62.0

```rust
fn parse_optional_type_modifiers(&mut self) -> Result<Option<Vec<String>>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:13805`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse optional type modifiers appearing in parentheses e.g. `(UNSIGNED, ZEROFILL)`.

<a id="op-613678bb7a5b83a9f9c3a1ea"></a>
## parse_optional_using_then_index_type

`function` · `sqlparser::parser::Parser::parse_optional_using_then_index_type` · sqlparser 0.62.0

```rust
fn parse_optional_using_then_index_type(&mut self) -> Result<Option<IndexType>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:10021`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optionally parse the `USING` keyword, followed by an [IndexType](../operations/sqlparser.ast.ddl.IndexType.md#op-8a08950c8c97b3d345c24530)
Example:
```sql
```
Optionally parse `USING <index_type>` and return the parsed `IndexType` if present.

<a id="op-348dbaede26e44cea3ee9226"></a>
## parse_options

`function` · `sqlparser::parser::Parser::parse_options` · sqlparser 0.62.0

```rust
fn parse_options(&mut self, keyword: Keyword) -> Result<Vec<SqlOption>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:9968`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a parenthesized list of `SqlOption`s following `keyword`, or return an empty vec.

<a id="op-ace0e9765eac22ade4e2c01e"></a>
## parse_options_with_keywords

`function` · `sqlparser::parser::Parser::parse_options_with_keywords` · sqlparser 0.62.0

```rust
fn parse_options_with_keywords(&mut self, keywords: &[Keyword]) -> Result<Vec<SqlOption>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:9980`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse options introduced by one of `keywords` followed by a parenthesized list.

<a id="op-42e951a0f8549ff6a209bb34"></a>
## parse_order_by_expr

`function` · `sqlparser::parser::Parser::parse_order_by_expr` · sqlparser 0.62.0

```rust
fn parse_order_by_expr(&mut self) -> Result<OrderByExpr, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:18782`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an [OrderByExpr](../operations/sqlparser.ast.query.OrderByExpr.md#op-9202f7394a1d5e3a01f7064a) expression.

<a id="op-75bec6da54e79086319fb208"></a>
## parse_overlay_expr

`function` · `sqlparser::parser::Parser::parse_overlay_expr` · sqlparser 0.62.0

```rust
fn parse_overlay_expr(&mut self) -> Result<Expr, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:2952`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an OVERLAY expression.

See [Expr::Overlay](../operations/sqlparser.ast.Expr.md#op-adbdd68b92fc323e928b96b6)

<a id="op-e4c9aeadcad8b8e048928761"></a>
## parse_owner

`function` · `sqlparser::parser::Parser::parse_owner` · sqlparser 0.62.0

```rust
fn parse_owner(&mut self) -> Result<Owner, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:6895`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an `OWNER` clause.

<a id="op-89b8e38161764a04e929b27f"></a>
## parse_parenthesized

`function` · `sqlparser::parser::Parser::parse_parenthesized` · sqlparser 0.62.0

```rust
fn parse_parenthesized<T, F>(&mut self, f: F) -> Result<T, ParserError> where F: FnMut(&mut Parser<'a>) -> Result<T, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4990`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an expression enclosed in parentheses.

<a id="op-92690f2822894938928485e3"></a>
## parse_parenthesized_column_list

`function` · `sqlparser::parser::Parser::parse_parenthesized_column_list` · sqlparser 0.62.0

```rust
fn parse_parenthesized_column_list(&mut self, optional: IsOptional, allow_empty: bool) -> Result<Vec<Ident>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:13492`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parses a parenthesized comma-separated list of unqualified, possibly quoted identifiers.
For example: `(col1, "col 2", ...)`

<a id="op-f75d006641b9df19d0528254"></a>
## parse_parenthesized_compound_identifier_list

`function` · `sqlparser::parser::Parser::parse_parenthesized_compound_identifier_list` · sqlparser 0.62.0

```rust
fn parse_parenthesized_compound_identifier_list(&mut self, optional: IsOptional, allow_empty: bool) -> Result<Vec<Expr>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:13501`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a parenthesized list of compound identifiers as expressions.

<a id="op-f9fcde99b6e2c492b768b0c8"></a>
## parse_parenthesized_qualified_column_list

`function` · `sqlparser::parser::Parser::parse_parenthesized_qualified_column_list` · sqlparser 0.62.0

```rust
fn parse_parenthesized_qualified_column_list(&mut self, optional: IsOptional, allow_empty: bool) -> Result<Vec<ObjectName>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:13523`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parses a parenthesized comma-separated list of qualified, possibly quoted identifiers.
For example: `(db1.sc1.tbl1.col1, db1.sc1.tbl1."col 2", ...)`

<a id="op-e20df9c78e6e628a56906b28"></a>
## parse_partition

`function` · `sqlparser::parser::Parser::parse_partition` · sqlparser 0.62.0

```rust
fn parse_partition(&mut self) -> Result<Partition, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:10166`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a parenthesized list of partition expressions and return a `Partition` value.

<a id="op-dff7e72cee2f823612f492f2"></a>
## parse_pg_cast

`function` · `sqlparser::parser::Parser::parse_pg_cast` · sqlparser 0.62.0

```rust
fn parse_pg_cast(&mut self, expr: Expr) -> Result<Expr, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4340`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a PostgreSQL casting style which is in the form of `expr::datatype`.

<a id="op-07487039ae06ae2367de2265"></a>
## parse_pg_create_server

`function` · `sqlparser::parser::Parser::parse_pg_create_server` · sqlparser 0.62.0

```rust
fn parse_pg_create_server(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:19734`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

 Parse a `CREATE SERVER` statement.

See [Statement::CreateServer](../operations/sqlparser.ast.Statement.md#op-2d1a84ea83272b81724b3f5a)

<a id="op-5088c614639243c13dfb57ef"></a>
## parse_pivot_table_factor

`function` · `sqlparser::parser::Parser::parse_pivot_table_factor` · sqlparser 0.62.0

```rust
fn parse_pivot_table_factor(&mut self, table: TableFactor) -> Result<TableFactor, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:17066`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a PIVOT table factor (ClickHouse/Oracle style pivot), returning a TableFactor.

<a id="op-ab1b52ef1188f4a4fd7a10bf"></a>
## parse_plain_options

`function` · `sqlparser::parser::Parser::parse_plain_options` · sqlparser 0.62.0

```rust
fn parse_plain_options(&mut self) -> Result<Vec<SqlOption>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:9038`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse plain options.

<a id="op-669ce74aacb8986a8c0c0499"></a>
## parse_position_expr

`function` · `sqlparser::parser::Parser::parse_position_expr` · sqlparser 0.62.0

```rust
fn parse_position_expr(&mut self, ident: Ident) -> Result<Expr, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:2892`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a `POSITION` expression.

<a id="op-aa4b123ae7cf99ab53e4c351"></a>
## parse_pragma

`function` · `sqlparser::parser::Parser::parse_pragma` · sqlparser 0.62.0

```rust
fn parse_pragma(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:19518`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

PRAGMA [schema-name '.'] pragma-name [('=' pragma-value) | '(' pragma-value ')']

<a id="op-99b7a9a6c0d264ac8b970031"></a>
## parse_precision

`function` · `sqlparser::parser::Parser::parse_precision` · sqlparser 0.62.0

```rust
fn parse_precision(&mut self) -> Result<u64, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:13576`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an unsigned precision value enclosed in parentheses, e.g. `(10)`.

<a id="op-840329fdb888b87b1e0e95ca"></a>
## parse_prefix

`function` · `sqlparser::parser::Parser::parse_prefix` · sqlparser 0.62.0

```rust
fn parse_prefix(&mut self) -> Result<Expr, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:1713`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an expression prefix.

<a id="op-116958e889bb9d8e08185614"></a>
## parse_prepare

`function` · `sqlparser::parser::Parser::parse_prepare` · sqlparser 0.62.0

```rust
fn parse_prepare(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:19435`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a SQL `PREPARE` statement

<a id="op-5434a5bed59c630d8877a1b0"></a>
## parse_procedure_param

`function` · `sqlparser::parser::Parser::parse_procedure_param` · sqlparser 0.62.0

```rust
fn parse_procedure_param(&mut self) -> Result<ProcedureParam, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:9146`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse procedure parameter.

<a id="op-d0c80f445a4ccae2bed11e03"></a>
## parse_projection

`function` · `sqlparser::parser::Parser::parse_projection` · sqlparser 0.62.0

```rust
fn parse_projection(&mut self) -> Result<Vec<SelectItem>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4823`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a comma-separated list of 1+ SelectItem

<a id="op-9f7f809a09d286a9a4205e69"></a>
## parse_projection_select

`function` · `sqlparser::parser::Parser::parse_projection_select` · sqlparser 0.62.0

```rust
fn parse_projection_select(&mut self) -> Result<ProjectionSelect, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:10174`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a parenthesized `SELECT` projection used for projection-based operations.

<a id="op-5f443c9e6d366b3df0fffa81"></a>
## parse_query

`function` · `sqlparser::parser::Parser::parse_query` · sqlparser 0.62.0

```rust
fn parse_query(&mut self) -> Result<Box<Query>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:14019`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a query expression, i.e. a `SELECT` statement optionally
preceded with some `WITH` CTE declarations and optionally followed
by `ORDER BY`. Unlike some other parse_... methods, this one doesn't
expect the initial keyword to be already consumed

<a id="op-41408b873f6a775b5bd9419e"></a>
## parse_query_body

`function` · `sqlparser::parser::Parser::parse_query_body` · sqlparser 0.62.0

```rust
fn parse_query_body(&mut self, precedence: u8) -> Result<Box<SetExpr>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:14569`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a "query body", which is an expression with roughly the
following grammar:
```sql
  query_body ::= restricted_select | '(' subquery ')' | set_operation
  restricted_select ::= 'SELECT' [expr_list] [ from ] [ where ] [ groupby_having ]
  subquery ::= query_body [ order_by_limit ]
  set_operation ::= query_body { 'UNION' | 'EXCEPT' | 'INTERSECT' } [ 'ALL' ] query_body
```

<a id="op-7e584a8579c401015827276f"></a>
## parse_raise_stmt

`function` · `sqlparser::parser::Parser::parse_raise_stmt` · sqlparser 0.62.0

```rust
fn parse_raise_stmt(&mut self) -> Result<RaiseStatement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:883`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a `RAISE` statement.

See [Statement::Raise](../operations/sqlparser.ast.Statement.md#op-bfa924658e1fc1460d587b9f)

<a id="op-80e6ae87cd6f139fa5e1874c"></a>
## parse_raiserror

`function` · `sqlparser::parser::Parser::parse_raiserror` · sqlparser 0.62.0

```rust
fn parse_raiserror(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:19303`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a 'RAISERROR' statement

<a id="op-ee8604816a6abf041509b2f9"></a>
## parse_raiserror_option

`function` · `sqlparser::parser::Parser::parse_raiserror_option` · sqlparser 0.62.0

```rust
fn parse_raiserror_option(&mut self) -> Result<RaisErrorOption, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:19331`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a single `RAISERROR` option

<a id="op-a0de594fb7053eac70291880"></a>
## parse_referential_action

`function` · `sqlparser::parser::Parser::parse_referential_action` · sqlparser 0.62.0

```rust
fn parse_referential_action(&mut self) -> Result<ReferentialAction, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:9622`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a referential action used in foreign key clauses.

Recognized forms: `RESTRICT`, `CASCADE`, `SET NULL`, `NO ACTION`, `SET DEFAULT`.

<a id="op-bcd931a45e15e6a4116305cd"></a>
## parse_release

`function` · `sqlparser::parser::Parser::parse_release` · sqlparser 0.62.0

```rust
fn parse_release(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:1448`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse `RELEASE` statement.

<a id="op-bf8efa887b3e10947b59e771"></a>
## parse_rename

`function` · `sqlparser::parser::Parser::parse_rename` · sqlparser 0.62.0

```rust
fn parse_rename(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:1489`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parses a `RENAME TABLE` statement. See [Statement::RenameTable](../operations/sqlparser.ast.Statement.md#op-6f4f036d679d804ccedfb6ab)

<a id="op-b21adaf369803edc6d449a2b"></a>
## parse_replace

`function` · `sqlparser::parser::Parser::parse_replace` · sqlparser 0.62.0

```rust
fn parse_replace(&mut self, replace_token: TokenWithSpan) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:17822`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an REPLACE statement

<a id="op-40295b11391c3f6ea4776a30"></a>
## parse_replace_elements

`function` · `sqlparser::parser::Parser::parse_replace_elements` · sqlparser 0.62.0

```rust
fn parse_replace_elements(&mut self) -> Result<ReplaceSelectElement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:18758`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a single element of a `REPLACE (...)` select-item clause.

<a id="op-94898a1a1933b0ed721ddaae"></a>
## parse_revoke

`function` · `sqlparser::parser::Parser::parse_revoke` · sqlparser 0.62.0

```rust
fn parse_revoke(&mut self) -> Result<Revoke, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:17798`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a REVOKE statement

<a id="op-d52fe9cc0dfc197a31d71c99"></a>
## parse_rollback

`function` · `sqlparser::parser::Parser::parse_rollback` · sqlparser 0.62.0

```rust
fn parse_rollback(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:19271`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a 'ROLLBACK' statement

<a id="op-4a6ef3988ee5cd3001d167a7"></a>
## parse_rollback_savepoint

`function` · `sqlparser::parser::Parser::parse_rollback_savepoint` · sqlparser 0.62.0

```rust
fn parse_rollback_savepoint(&mut self) -> Result<Option<Ident>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:19291`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an optional 'TO SAVEPOINT savepoint_name' clause for ROLLBACK statements

<a id="op-15b4ed0fc20173af00b3bf11"></a>
## parse_row_format

`function` · `sqlparser::parser::Parser::parse_row_format` · sqlparser 0.62.0

```rust
fn parse_row_format(&mut self) -> Result<HiveRowFormat, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:8372`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse Hive row format.

<a id="op-ca5eea9eb646e679218f8802"></a>
## parse_savepoint

`function` · `sqlparser::parser::Parser::parse_savepoint` · sqlparser 0.62.0

```rust
fn parse_savepoint(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:1442`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse `SAVEPOINT` statement.

<a id="op-7735e10c3c98a4b9f24e798e"></a>
## parse_select

`function` · `sqlparser::parser::Parser::parse_select` · sqlparser 0.62.0

```rust
fn parse_select(&mut self) -> Result<Select, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:14677`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a restricted `SELECT` statement (no CTEs / `UNION` / `ORDER BY`)

<a id="op-2619036a337c0bef91367e22"></a>
## parse_select_item

`function` · `sqlparser::parser::Parser::parse_select_item` · sqlparser 0.62.0

```rust
fn parse_select_item(&mut self) -> Result<SelectItem, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:18512`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a comma-delimited list of projections after SELECT

<a id="op-a1397effb4ce9207d424dbea"></a>
## parse_set_operator

`function` · `sqlparser::parser::Parser::parse_set_operator` · sqlparser 0.62.0

```rust
fn parse_set_operator(&mut self, token: &Token) -> Option<SetOperator>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:14637`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a set operator token into its `SetOperator` variant.

<a id="op-4d95b5c4387cc433603b76a4"></a>
## parse_set_quantifier

`function` · `sqlparser::parser::Parser::parse_set_quantifier` · sqlparser 0.62.0

```rust
fn parse_set_quantifier(&mut self, op: &Option<SetOperator>) -> SetQuantifier
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:14648`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a set quantifier (e.g., `ALL`, `DISTINCT BY NAME`) for the given set operator.

<a id="op-0b47a204d52a0d0e79570427"></a>
## parse_set_session_params

`function` · `sqlparser::parser::Parser::parse_set_session_params` · sqlparser 0.62.0

```rust
fn parse_set_session_params(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:15388`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse session parameter assignments after `SET` when no `=` or `TO` is present.

<a id="op-53ac6b2466fe595282b4d913"></a>
## parse_show

`function` · `sqlparser::parser::Parser::parse_show` · sqlparser 0.62.0

```rust
fn parse_show(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:15463`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a `SHOW` statement and dispatch to specific SHOW handlers.

<a id="op-383847dfe7c645fc1403748d"></a>
## parse_show_collation

`function` · `sqlparser::parser::Parser::parse_show_collation` · sqlparser 0.62.0

```rust
fn parse_show_collation(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:15643`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse `SHOW COLLATION` and optional filter.

<a id="op-fb57428dae7224ea6fceca06"></a>
## parse_show_columns

`function` · `sqlparser::parser::Parser::parse_show_columns` · sqlparser 0.62.0

```rust
fn parse_show_columns(&mut self, extended: bool, full: bool) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:15591`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse `SHOW COLUMNS`/`SHOW FIELDS` and return a `ShowColumns` statement.

<a id="op-ea4544dcce22720df798e04b"></a>
## parse_show_create

`function` · `sqlparser::parser::Parser::parse_show_create` · sqlparser 0.62.0

```rust
fn parse_show_create(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:15565`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse `SHOW CREATE <object>` returning the corresponding `ShowCreate` statement.

<a id="op-6bec505ce4183b5646de64a1"></a>
## parse_show_functions

`function` · `sqlparser::parser::Parser::parse_show_functions` · sqlparser 0.62.0

```rust
fn parse_show_functions(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:15637`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse `SHOW FUNCTIONS` and optional filter.

<a id="op-2e2c76f2cc5eb18297036089"></a>
## parse_show_statement_filter

`function` · `sqlparser::parser::Parser::parse_show_statement_filter` · sqlparser 0.62.0

```rust
fn parse_show_statement_filter(&mut self) -> Result<Option<ShowStatementFilter>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:15649`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an optional filter used by `SHOW` statements (LIKE, ILIKE, WHERE, or literal).

<a id="op-44f2d1640a4a5f2b697a8d24"></a>
## parse_snowflake_declare

`function` · `sqlparser::parser::Parser::parse_snowflake_declare` · sqlparser 0.62.0

```rust
fn parse_snowflake_declare(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:7721`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a [Snowflake] `DECLARE` statement.

Syntax:
```text
DECLARE
  [{ <variable_declaration>
     | <cursor_declaration>
     | <resultset_declaration>
     | <exception_declaration> }; ... ]

<variable_declaration>
<variable_name> [<type>] [ { DEFAULT | := } <expression>]

<cursor_declaration>
<cursor_name> CURSOR FOR <query>

<resultset_declaration>
<resultset_name> RESULTSET [ { DEFAULT | := } ( <query> ) ] ;

<exception_declaration>
<exception_name> EXCEPTION [ ( <exception_number> , '<exception_message>' ) ] ;
```

[Snowflake]: https://docs.snowflake.com/en/sql-reference/snowflake-scripting/declare

<a id="op-bbfd3daabaee5de58fbbec96"></a>
## parse_snowflake_variable_declaration_expression

`function` · `sqlparser::parser::Parser::parse_snowflake_variable_declaration_expression` · sqlparser 0.62.0

```rust
fn parse_snowflake_variable_declaration_expression(&mut self) -> Result<Option<DeclareAssignment>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:7901`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parses the assigned expression in a variable declaration.

Syntax:
```text
[ { DEFAULT | := } <expression>]
```
<https://docs.snowflake.com/en/sql-reference/snowflake-scripting/declare#variable-declaration-syntax>

<a id="op-fc7ccb40a43499852e70701f"></a>
## parse_sql

`function` · `sqlparser::parser::Parser::parse_sql` · sqlparser 0.62.0

```rust
fn parse_sql(dialect: &dyn Dialect, sql: &str) -> Result<Vec<Statement>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:545`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Convenience method to parse a string with one or more SQL
statements into produce an Abstract Syntax Tree (AST).

Example
```
# use sqlparser::{parser::{Parser, ParserError}, dialect::GenericDialect};
# fn main() -> Result<(), ParserError> {
let dialect = GenericDialect{};
let statements = Parser::parse_sql(
  &dialect, "SELECT * FROM foo"
)?;
assert_eq!(statements.len(), 1);
# Ok(())
# }
```

<a id="op-18d2e350ff0daa223a5048a6"></a>
## parse_sql_option

`function` · `sqlparser::parser::Parser::parse_sql_option` · sqlparser 0.62.0

```rust
fn parse_sql_option(&mut self) -> Result<SqlOption, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:10075`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a single `SqlOption` used by various dialect-specific DDL statements.

<a id="op-93f87130b0a73afb4df436c3"></a>
## parse_sql_with_comments

`function` · `sqlparser::parser::Parser::parse_sql_with_comments` · sqlparser 0.62.0

```rust
fn parse_sql_with_comments(dialect: &'a dyn Dialect, sql: &str) -> Result<(Vec<Statement>, comments::Comments), ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:553`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parses the given `sql` into an Abstract Syntax Tree (AST), returning
also encountered source code comments.

See [Parser::parse_sql](../operations/sqlparser.parser.Parser.md#op-fc7ccb40a43499852e70701f).

<a id="op-704244dab990bd35a3db63e9"></a>
## parse_start_transaction

`function` · `sqlparser::parser::Parser::parse_start_transaction` · sqlparser 0.62.0

```rust
fn parse_start_transaction(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:19104`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a 'START TRANSACTION' statement

<a id="op-4f1ded133b370b9870e52662"></a>
## parse_statement

`function` · `sqlparser::parser::Parser::parse_statement` · sqlparser 0.62.0

```rust
fn parse_statement(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:589`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a single top-level statement (such as SELECT, INSERT, CREATE, etc.),
stopping before the statement separator, if any.

<a id="op-505112fede589f7089aaa334"></a>
## parse_statements

`function` · `sqlparser::parser::Parser::parse_statements` · sqlparser 0.62.0

```rust
fn parse_statements(&mut self) -> Result<Vec<Statement>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:494`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse potentially multiple statements

Example
```
# use sqlparser::{parser::{Parser, ParserError}, dialect::GenericDialect};
# fn main() -> Result<(), ParserError> {
let dialect = GenericDialect{};
let statements = Parser::new(&dialect)
  // Parse a SQL string with 2 separate statements
  .try_with_sql("SELECT * FROM foo; SELECT * FROM bar;")?
  .parse_statements()?;
assert_eq!(statements.len(), 2);
# Ok(())
# }
```

<a id="op-418d3eb07fd722d88c2579e7"></a>
## parse_string_values

`function` · `sqlparser::parser::Parser::parse_string_values` · sqlparser 0.62.0

```rust
fn parse_string_values(&mut self) -> Result<Vec<String>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:12757`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a parenthesized, comma-separated list of single-quoted strings.

<a id="op-c4399b6b80f444193cfaf821"></a>
## parse_subexpr

`function` · `sqlparser::parser::Parser::parse_subexpr` · sqlparser 0.62.0

```rust
fn parse_subexpr(&mut self, precedence: u8) -> Result<Expr, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:1391`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse tokens until the precedence changes.

<a id="op-4483dc32e40aae37ba07304f"></a>
## parse_substring

`function` · `sqlparser::parser::Parser::parse_substring` · sqlparser 0.62.0

```rust
fn parse_substring(&mut self) -> Result<Expr, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:2917`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse `SUBSTRING`/`SUBSTR` expressions: `SUBSTRING(expr FROM start FOR length)` or `SUBSTR(expr, start, length)`.

<a id="op-9d1d5aaa766b0c5a4349f857"></a>
## parse_tab_value

`function` · `sqlparser::parser::Parser::parse_tab_value` · sqlparser 0.62.0

```rust
fn parse_tab_value(&mut self) -> Vec<Option<String>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:11933`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a single tab-separated value row used by `COPY` payload parsing.

<a id="op-9163faaf371c5320f077ad00"></a>
## parse_table_and_joins

`function` · `sqlparser::parser::Parser::parse_table_and_joins` · sqlparser 0.62.0

```rust
fn parse_table_and_joins(&mut self) -> Result<TableWithJoins, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:15725`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a table factor followed by any join clauses, returning `TableWithJoins`.

<a id="op-6d4684a98d6f3c32934da385"></a>
## parse_table_factor

`function` · `sqlparser::parser::Parser::parse_table_factor` · sqlparser 0.62.0

```rust
fn parse_table_factor(&mut self) -> Result<TableFactor, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:15945`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A table name or a parenthesized subquery, followed by optional `[AS] alias`

<a id="op-aca32d4e38df14268b54f247"></a>
## parse_table_object

`function` · `sqlparser::parser::Parser::parse_table_object` · sqlparser 0.62.0

```rust
fn parse_table_object(&mut self) -> Result<TableObject, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:13117`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a table object for insertion
e.g. `some_database.some_table` or `FUNCTION some_table_func(...)`

<a id="op-d515691764dddc2fc2238327"></a>
## parse_throw

`function` · `sqlparser::parser::Parser::parse_throw` · sqlparser 0.62.0

```rust
fn parse_throw(&mut self) -> Result<ThrowStatement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:19346`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a MSSQL `THROW` statement.

See [Statement::Throw](../operations/sqlparser.ast.Statement.md#op-649bdc33fd7832279e6a10a1)

<a id="op-5a44129e5b05d05cc0d54e63"></a>
## parse_time_functions

`function` · `sqlparser::parser::Parser::parse_time_functions` · sqlparser 0.62.0

```rust
fn parse_time_functions(&mut self, name: ObjectName) -> Result<Expr, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:2546`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse time-related function `name` possibly followed by `(...)` arguments.

<a id="op-43edfe3b1c680cce3e7d0f7a"></a>
## parse_top

`function` · `sqlparser::parser::Parser::parse_top` · sqlparser 0.62.0

```rust
fn parse_top(&mut self) -> Result<Top, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:18910`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a TOP clause, MSSQL equivalent of LIMIT,
that follows after `SELECT [DISTINCT]`.

<a id="op-0533326033ff8e77affd3312"></a>
## parse_transaction_modes

`function` · `sqlparser::parser::Parser::parse_transaction_modes` · sqlparser 0.62.0

```rust
fn parse_transaction_modes(&mut self) -> Result<Vec<TransactionMode>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:19223`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a list of transaction modes

<a id="op-390779885bd615bb36d66924"></a>
## parse_trigger_event

`function` · `sqlparser::parser::Parser::parse_trigger_event` · sqlparser 0.62.0

```rust
fn parse_trigger_event(&mut self) -> Result<TriggerEvent, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:6280`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse the event part of a trigger (`INSERT`, `UPDATE`, etc.).

<a id="op-5912499ad2e7c5bb67168faa"></a>
## parse_trigger_exec_body

`function` · `sqlparser::parser::Parser::parse_trigger_exec_body` · sqlparser 0.62.0

```rust
fn parse_trigger_exec_body(&mut self) -> Result<TriggerExecBody, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:6330`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse the execution body of a trigger (`FUNCTION` or `PROCEDURE`).

<a id="op-f85ce33d2bfa98bea17440ba"></a>
## parse_trigger_period

`function` · `sqlparser::parser::Parser::parse_trigger_period` · sqlparser 0.62.0

```rust
fn parse_trigger_period(&mut self) -> Result<TriggerPeriod, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:6258`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse the period part of a trigger (`BEFORE`, `AFTER`, etc.).

<a id="op-4db2d110fd6d3ae98afab360"></a>
## parse_trigger_referencing

`function` · `sqlparser::parser::Parser::parse_trigger_referencing` · sqlparser 0.62.0

```rust
fn parse_trigger_referencing(&mut self) -> Result<Option<TriggerReferencing>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:6307`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse the `REFERENCING` clause of a trigger.

<a id="op-b639dfd332f9b5f0d6c46746"></a>
## parse_trim_expr

`function` · `sqlparser::parser::Parser::parse_trim_expr` · sqlparser 0.62.0

```rust
fn parse_trim_expr(&mut self) -> Result<Expr, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:2979`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
TRIM ([WHERE] ['text' FROM] 'text')
TRIM ('text')
TRIM(<expr>, [, characters]) -- PostgreSQL, DuckDB, Snowflake, BigQuery, Generic
```

<a id="op-03a2dd22d61519434c626fa5"></a>
## parse_trim_where

`function` · `sqlparser::parser::Parser::parse_trim_where` · sqlparser 0.62.0

```rust
fn parse_trim_where(&mut self) -> Result<TrimWhereField, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:3022`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse the `WHERE` field for a `TRIM` expression.

See [TrimWhereField](../operations/sqlparser.ast.value.TrimWhereField.md#op-e711960a81e5df7bf864046b)

<a id="op-f30d6090de6f43dfbc53a048"></a>
## parse_truncate

`function` · `sqlparser::parser::Parser::parse_truncate` · sqlparser 0.62.0

```rust
fn parse_truncate(&mut self) -> Result<Truncate, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:1094`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse `TRUNCATE` statement.

<a id="op-d024d221559c30ca2082fa59"></a>
## parse_tsv

`function` · `sqlparser::parser::Parser::parse_tsv` · sqlparser 0.62.0

```rust
fn parse_tsv(&mut self) -> Vec<Option<String>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:11928`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a tab separated values in
COPY payload

<a id="op-efb3676702430c8c5f2bf565"></a>
## parse_uncache_table

`function` · `sqlparser::parser::Parser::parse_uncache_table` · sqlparser 0.62.0

```rust
fn parse_uncache_table(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:5398`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a UNCACHE TABLE statement

<a id="op-34d535275eb2eaba6deb70db"></a>
## parse_unicode_is_normalized

`function` · `sqlparser::parser::Parser::parse_unicode_is_normalized` · sqlparser 0.62.0

```rust
fn parse_unicode_is_normalized(&mut self, expr: Expr) -> Result<Expr, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:12241`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a literal unicode normalization clause

<a id="op-5a20d96a30e8dbb841bf4eaf"></a>
## parse_unlisten

`function` · `sqlparser::parser::Parser::parse_unlisten` · sqlparser 0.62.0

```rust
fn parse_unlisten(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:1462`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse `UNLISTEN` statement.

<a id="op-151fa7ead232d4a0cfc9f719"></a>
## parse_unload

`function` · `sqlparser::parser::Parser::parse_unload` · sqlparser 0.62.0

```rust
fn parse_unload(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:19454`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a SQL `UNLOAD` statement

<a id="op-dcc004dd82dc0a8c2a03ad9c"></a>
## parse_unpivot_table_factor

`function` · `sqlparser::parser::Parser::parse_unpivot_table_factor` · sqlparser 0.62.0

```rust
fn parse_unpivot_table_factor(&mut self, table: TableFactor) -> Result<TableFactor, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:17123`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an UNPIVOT table factor, returning a TableFactor.

<a id="op-dccfbaaeab59f090fc9987bf"></a>
## parse_update

`function` · `sqlparser::parser::Parser::parse_update` · sqlparser 0.62.0

```rust
fn parse_update(&mut self, update_token: TokenWithSpan) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:18191`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an `UPDATE` statement and return `Statement::Update`.

<a id="op-e19d998cdea0f7733873ebbd"></a>
## parse_use

`function` · `sqlparser::parser::Parser::parse_use` · sqlparser 0.62.0

```rust
fn parse_use(&mut self) -> Result<Statement, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:15673`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a `USE` statement (database/catalog/schema/warehouse/role selection).

<a id="op-7f2739c13b2b250b716d73cc"></a>
## parse_utility_options

`function` · `sqlparser::parser::Parser::parse_utility_options` · sqlparser 0.62.0

```rust
fn parse_utility_options(&mut self) -> Result<Vec<UtilityOption>, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:2271`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse utility options in the form of `(option1, option2 arg2, option3 arg3, ...)`

<a id="op-6b2ddef402cd0b3707ba29a1"></a>
## parse_value

`function` · `sqlparser::parser::Parser::parse_value` · sqlparser 0.62.0

```rust
fn parse_value(&mut self) -> Result<ValueWithSpan, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:11963`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a literal value (numbers, strings, date/time, booleans)

<a id="op-1578241db1fd4197fd0b0a69"></a>
## parse_values

`function` · `sqlparser::parser::Parser::parse_values` · sqlparser 0.62.0

```rust
fn parse_values(&mut self, allow_empty: bool, value_keyword: bool) -> Result<Values, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:19075`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a VALUES clause

<a id="op-734d5fb618bbddbadc2ac31f"></a>
## parse_wildcard_additional_options

`function` · `sqlparser::parser::Parser::parse_wildcard_additional_options` · sqlparser 0.62.0

```rust
fn parse_wildcard_additional_options(&mut self, wildcard_token: TokenWithSpan) -> Result<WildcardAdditionalOptions, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:18589`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse an [`WildcardAdditionalOptions`](../operations/sqlparser.ast.query.WildcardAdditionalOptions.md#op-35c8794d2c31ce63bada0f67) information for wildcard select items.

If it is not possible to parse it, will return an option.

<a id="op-e16d644ddba95d81857c54d4"></a>
## parse_wildcard_expr

`function` · `sqlparser::parser::Parser::parse_wildcard_expr` · sqlparser 0.62.0

```rust
fn parse_wildcard_expr(&mut self) -> Result<Expr, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:1303`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a new expression including wildcard & qualified wildcard.

<a id="op-decb44704b1fcb126de8f5d0"></a>
## parse_window_frame

`function` · `sqlparser::parser::Parser::parse_window_frame` · sqlparser 0.62.0

```rust
fn parse_window_frame(&mut self) -> Result<WindowFrame, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:2579`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a `WINDOW` frame definition (units and bounds).

<a id="op-1f209f255fd4b4ff02f671f3"></a>
## parse_window_frame_bound

`function` · `sqlparser::parser::Parser::parse_window_frame_bound` · sqlparser 0.62.0

```rust
fn parse_window_frame_bound(&mut self) -> Result<WindowFrameBound, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:2597`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a window frame bound: `CURRENT ROW` or `<n> PRECEDING|FOLLOWING`.

<a id="op-8c720dececf051494d5f0878"></a>
## parse_window_frame_units

`function` · `sqlparser::parser::Parser::parse_window_frame_units` · sqlparser 0.62.0

```rust
fn parse_window_frame_units(&mut self) -> Result<WindowFrameUnits, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:2565`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse window frame `UNITS` clause: `ROWS`, `RANGE`, or `GROUPS`.

<a id="op-e15dd40a8a5ebaa5b2424b52"></a>
## parse_window_spec

`function` · `sqlparser::parser::Parser::parse_window_spec` · sqlparser 0.62.0

```rust
fn parse_window_spec(&mut self) -> Result<WindowSpec, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:19820`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a window specification.

<a id="op-ecb10cd9c4681dbad1762b36"></a>
## parse_with_fill

`function` · `sqlparser::parser::Parser::parse_with_fill` · sqlparser 0.62.0

```rust
fn parse_with_fill(&mut self) -> Result<WithFill, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:18854`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse a `WITH FILL` clause used in ORDER BY (ClickHouse dialect).

<a id="op-72b8dadbfaf23d2fff245e18"></a>
## peek_keyword

`function` · `sqlparser::parser::Parser::peek_keyword` · sqlparser 0.62.0

```rust
fn peek_keyword(&self, expected: Keyword) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4624`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Check if the current token is the expected keyword without consuming it.

Returns true if the current token matches the expected keyword.

<a id="op-7ebd8d29b63b15947ddc777f"></a>
## peek_nth_token

`function` · `sqlparser::parser::Parser::peek_nth_token` · sqlparser 0.62.0

```rust
fn peek_nth_token(&self, n: usize) -> TokenWithSpan
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4446`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Return nth non-whitespace token that has not yet been processed

<a id="op-e9161d243b2b9ac783f84ba9"></a>
## peek_nth_token_no_skip

`function` · `sqlparser::parser::Parser::peek_nth_token_no_skip` · sqlparser 0.62.0

```rust
fn peek_nth_token_no_skip(&self, n: usize) -> TokenWithSpan
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4477`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Return nth token, possibly whitespace, that has not yet been processed.

<a id="op-b9e9d6d71da728bd1e117c4f"></a>
## peek_nth_token_ref

`function` · `sqlparser::parser::Parser::peek_nth_token_ref` · sqlparser 0.62.0

```rust
fn peek_nth_token_ref(&self, n: usize) -> &TokenWithSpan
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4451`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Return nth non-whitespace token that has not yet been processed

<a id="op-64ce63be1ca3226d31243f1f"></a>
## peek_one_of_keywords

`function` · `sqlparser::parser::Parser::peek_one_of_keywords` · sqlparser 0.62.0

```rust
fn peek_one_of_keywords(&self, keywords: &[Keyword]) -> Option<Keyword>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4697`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

If the current token is one of the given `keywords`, returns the keyword
that matches, without consuming the token. Otherwise, returns [`None`].

Unresolved upstream links (retained, not inferred): ``None``.

<a id="op-e8673576d150fb88789b0a7c"></a>
## peek_token

`function` · `sqlparser::parser::Parser::peek_token` · sqlparser 0.62.0

```rust
fn peek_token(&self) -> TokenWithSpan
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4365`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Return the first non-whitespace token that has not yet been processed
or Token::EOF

See [`Self::peek_token_ref`](../operations/sqlparser.parser.Parser.md#op-d4463aa91a7aefff3ce728a7) to avoid the copy.

<a id="op-37c322af7c0e633341ff936d"></a>
## peek_token_no_skip

`function` · `sqlparser::parser::Parser::peek_token_no_skip` · sqlparser 0.62.0

```rust
fn peek_token_no_skip(&self) -> TokenWithSpan
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4472`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Return the first token, possibly whitespace, that has not yet been processed
(or None if reached end-of-file).

<a id="op-d4463aa91a7aefff3ce728a7"></a>
## peek_token_ref

`function` · `sqlparser::parser::Parser::peek_token_ref` · sqlparser 0.62.0

```rust
fn peek_token_ref(&self) -> &TokenWithSpan
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4371`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Return a reference to the first non-whitespace token that has not yet
been processed or Token::EOF

<a id="op-3b752a5b90ade2a85358424d"></a>
## peek_tokens

`function` · `sqlparser::parser::Parser::peek_tokens` · sqlparser 0.62.0

```rust
fn peek_tokens<const N: usize>(&self) -> [Token; N]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4397`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns the `N` next non-whitespace tokens that have not yet been
processed.

Example:
```rust
# use sqlparser::dialect::GenericDialect;
# use sqlparser::parser::Parser;
# use sqlparser::keywords::Keyword;
# use sqlparser::tokenizer::{Token, Word};
let dialect = GenericDialect {};
let mut parser = Parser::new(&dialect).try_with_sql("ORDER BY foo, bar").unwrap();

// Note that Rust infers the number of tokens to peek based on the
// length of the slice pattern!
assert!(matches!(
    parser.peek_tokens(),
    [
        Token::Word(Word { keyword: Keyword::ORDER, .. }),
        Token::Word(Word { keyword: Keyword::BY, .. }),
    ]
));
```

<a id="op-fd217223a7c47337b4d9feae"></a>
## peek_tokens_ref

`function` · `sqlparser::parser::Parser::peek_tokens_ref` · sqlparser 0.62.0

```rust
fn peek_tokens_ref<const N: usize>(&self) -> [&TokenWithSpan; N]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4429`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns references to the `N` next non-whitespace tokens
that have not yet been processed.

See [`Self::peek_tokens`](../operations/sqlparser.parser.Parser.md#op-3b752a5b90ade2a85358424d) for an example.

<a id="op-ebe6ec98c8e56ce757a75cbe"></a>
## peek_tokens_with_location

`function` · `sqlparser::parser::Parser::peek_tokens_with_location` · sqlparser 0.62.0

```rust
fn peek_tokens_with_location<const N: usize>(&self) -> [TokenWithSpan; N]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4406`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns the `N` next non-whitespace tokens with locations that have not
yet been processed.

See [`Self::peek_token`](../operations/sqlparser.parser.Parser.md#op-e8673576d150fb88789b0a7c) for an example.

<a id="op-223eaac56f6d29ae07c08469"></a>
## prev_token

`function` · `sqlparser::parser::Parser::prev_token` · sqlparser 0.62.0

```rust
fn prev_token(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4568`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Seek back the last one non-whitespace token.

Must be called after `next_token()`, otherwise might panic. OK to call
after `next_token()` indicates an EOF.


<a id="op-b4b71c52793eb75b1b8900b5"></a>
## token_at

`function` · `sqlparser::parser::Parser::token_at` · sqlparser 0.62.0

```rust
fn token_at(&self, index: usize) -> &TokenWithSpan
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:4357`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Return the token at the given location, or EOF if the index is beyond
the length of the current set of tokens.

<a id="op-a697049f37aacc49e5770219"></a>
## try_parse

`function` · `sqlparser::parser::Parser::try_parse` · sqlparser 0.62.0

```rust
fn try_parse<T, F>(&mut self, f: F) -> Result<T, ParserError> where F: FnMut(&mut Parser<'_>) -> Result<T, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:5069`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Run a parser method `f`, reverting back to the current position if unsuccessful.

<a id="op-79c4230a9c4caf6e2845f247"></a>
## try_with_sql

`function` · `sqlparser::parser::Parser::try_with_sql` · sqlparser 0.62.0

```rust
fn try_with_sql(self, sql: &str) -> Result<Self, ParserError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:471`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Tokenize the sql string and sets this [`Parser`](../operations/sqlparser.parser.Parser.md#op-b761887fc90688e669ac5e12)'s state to
parse the resulting tokens

Returns an error if there was an error tokenizing the SQL string.

See example on [`Parser::new()`](../operations/sqlparser.parser.Parser.md#op-82ec32737f9279974a92f7f8) for an example

<a id="op-6efafad3a1718a37999c1921"></a>
## with_options

`function` · `sqlparser::parser::Parser::with_options` · sqlparser 0.62.0

```rust
fn with_options(self, options: ParserOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:440`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Specify additional parser options

[`Parser`](../operations/sqlparser.parser.Parser.md#op-b761887fc90688e669ac5e12) supports additional options ([`ParserOptions`](../operations/sqlparser.parser.ParserOptions.md#op-02e47302b4279575345d622b))
that allow you to mix & match behavior otherwise constrained
to certain dialects (e.g. trailing commas).

Example:
```
# use sqlparser::{parser::{Parser, ParserError, ParserOptions}, dialect::GenericDialect};
# fn main() -> Result<(), ParserError> {
let dialect = GenericDialect{};
let options = ParserOptions::new()
   .with_trailing_commas(true)
   .with_unescape(false);
let result = Parser::new(&dialect)
  .with_options(options)
  .try_with_sql("SELECT a, b, COUNT(*), FROM foo GROUP BY a, b,")?
  .parse_statements();
  assert!(matches!(result, Ok(_)));
# Ok(())
# }
```

<a id="op-b818b7fc65470b89ca72babb"></a>
## with_recursion_limit

`function` · `sqlparser::parser::Parser::with_recursion_limit` · sqlparser 0.62.0

```rust
fn with_recursion_limit(self, recursion_limit: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:413`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Specify the maximum recursion limit while parsing.

[`Parser`](../operations/sqlparser.parser.Parser.md#op-b761887fc90688e669ac5e12) prevents stack overflows by returning
[`ParserError::RecursionLimitExceeded`](../operations/sqlparser.parser.ParserError.md#op-30c7068096427b0c5d403ae4) if the parser exceeds
this depth while processing the query.

Example:
```
# use sqlparser::{parser::{Parser, ParserError}, dialect::GenericDialect};
# fn main() -> Result<(), ParserError> {
let dialect = GenericDialect{};
let result = Parser::new(&dialect)
  .with_recursion_limit(1)
  .try_with_sql("SELECT * FROM foo WHERE (a OR (b OR (c OR d)))")?
  .parse_statements();
  assert_eq!(result, Err(ParserError::RecursionLimitExceeded));
# Ok(())
# }
```

Note: when "recursive-protection" feature is enabled, this crate uses additional stack overflow protection

<a id="op-bdbbc36ca57b6b9439609fe6"></a>
## with_tokens

`function` · `sqlparser::parser::Parser::with_tokens` · sqlparser 0.62.0

```rust
fn with_tokens(self, tokens: Vec<Token>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:453`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Reset this parser state to parse the specified tokens

<a id="op-d70f46325219da289e936be1"></a>
## with_tokens_with_locations

`function` · `sqlparser::parser::Parser::with_tokens_with_locations` · sqlparser 0.62.0

```rust
fn with_tokens_with_locations(self, tokens: Vec<TokenWithSpan>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::parser::Parser", "path": "Parser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [20562, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:446`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Reset this parser to parse the specified token stream
