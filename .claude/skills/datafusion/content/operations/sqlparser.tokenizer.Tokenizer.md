# `sqlparser::tokenizer::Tokenizer`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.tokenizer.Tokenizer.json).

<a id="op-58f927537ea1ecff6e8c5b80"></a>
## Tokenizer

`struct` · `sqlparser::tokenizer::Tokenizer` · sqlparser 0.62.0

```rust
struct Tokenizer<'a>
```

Source: `src/tokenizer.rs:869`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SQL Tokenizer

<a id="op-3428290372b13b40782496ef"></a>
## new

`function` · `sqlparser::tokenizer::Tokenizer::new` · sqlparser 0.62.0

```rust
fn new(dialect: &'a dyn Dialect, query: &'a str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::tokenizer::Tokenizer", "path": "Tokenizer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [877, 1], "end": [2403, 2], "filename": "src/tokenizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/tokenizer.rs:894`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Create a new SQL tokenizer for the specified SQL statement

```
# use sqlparser::tokenizer::{Token, Whitespace, Tokenizer};
# use sqlparser::dialect::GenericDialect;
# let dialect = GenericDialect{};
let query = r#"SELECT 'foo'"#;

// Parsing the query
let tokens = Tokenizer::new(&dialect, &query).tokenize().unwrap();

assert_eq!(tokens, vec![
  Token::make_word("SELECT", None),
  Token::Whitespace(Whitespace::Space),
  Token::SingleQuotedString("foo".to_string()),
]);

<a id="op-49e0e7d60582c3335dbbe42a"></a>
## tokenize

`function` · `sqlparser::tokenizer::Tokenizer::tokenize` · sqlparser 0.62.0

```rust
fn tokenize(&mut self) -> Result<Vec<Token>, TokenizerError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::tokenizer::Tokenizer", "path": "Tokenizer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [877, 1], "end": [2403, 2], "filename": "src/tokenizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/tokenizer.rs:938`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Tokenize the statement and produce a vector of tokens

<a id="op-3091a5287240f7b292d6826b"></a>
## tokenize_with_location

`function` · `sqlparser::tokenizer::Tokenizer::tokenize_with_location` · sqlparser 0.62.0

```rust
fn tokenize_with_location(&mut self) -> Result<Vec<TokenWithSpan>, TokenizerError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::tokenizer::Tokenizer", "path": "Tokenizer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [877, 1], "end": [2403, 2], "filename": "src/tokenizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/tokenizer.rs:944`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Tokenize the statement and produce a vector of tokens with location information

<a id="op-362bd3aa1d74760960314695"></a>
## tokenize_with_location_into_buf

`function` · `sqlparser::tokenizer::Tokenizer::tokenize_with_location_into_buf` · sqlparser 0.62.0

```rust
fn tokenize_with_location_into_buf(&mut self, buf: &mut Vec<TokenWithSpan>) -> Result<(), TokenizerError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::tokenizer::Tokenizer", "path": "Tokenizer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [877, 1], "end": [2403, 2], "filename": "src/tokenizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/tokenizer.rs:952`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Tokenize the statement and append tokens with location information into the provided buffer.
If an error is thrown, the buffer will contain all tokens that were successfully parsed before the error.

<a id="op-2807bed215f0a68b1355a3d3"></a>
## tokenize_with_location_into_buf_with_mapper

`function` · `sqlparser::tokenizer::Tokenizer::tokenize_with_location_into_buf_with_mapper` · sqlparser 0.62.0

```rust
fn tokenize_with_location_into_buf_with_mapper(&mut self, buf: &mut Vec<TokenWithSpan>, mapper: impl FnMut(TokenWithSpan) -> TokenWithSpan) -> Result<(), TokenizerError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::tokenizer::Tokenizer", "path": "Tokenizer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [877, 1], "end": [2403, 2], "filename": "src/tokenizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/tokenizer.rs:961`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Tokenize the statement and produce a vector of tokens, mapping each token
with provided `mapper`

<a id="op-78c268a0e0624fbf2a8204bd"></a>
## with_unescape

`function` · `sqlparser::tokenizer::Tokenizer::with_unescape` · sqlparser 0.62.0

```rust
fn with_unescape(self, unescape: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "sqlparser::tokenizer::Tokenizer", "path": "Tokenizer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [877, 1], "end": [2403, 2], "filename": "src/tokenizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/tokenizer.rs:932`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set unescape mode

When true (default) the tokenizer unescapes literal values
(for example, `""` in SQL is unescaped to the literal `"`).

When false, the tokenizer provides the raw strings as provided
in the query.  This can be helpful for programs that wish to
recover the *exact* original query text without normalizing
the escaping

# Example

```
# use sqlparser::tokenizer::{Token, Tokenizer};
# use sqlparser::dialect::GenericDialect;
# let dialect = GenericDialect{};
let query = r#""Foo "" Bar""#;
let unescaped = Token::make_word(r#"Foo " Bar"#, Some('"'));
let original  = Token::make_word(r#"Foo "" Bar"#, Some('"'));

// Parsing with unescaping (default)
let tokens = Tokenizer::new(&dialect, &query).tokenize().unwrap();
assert_eq!(tokens, vec![unescaped]);

// Parsing with unescape = false
let tokens = Tokenizer::new(&dialect, &query)
   .with_unescape(false)
   .tokenize().unwrap();
assert_eq!(tokens, vec![original]);
```
