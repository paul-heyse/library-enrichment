# `sqlparser::parser::ParserOptions`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.parser.ParserOptions.json).

<a id="op-02e47302b4279575345d622b"></a>
## ParserOptions

`struct` · `sqlparser::parser::ParserOptions` · sqlparser 0.62.0

```rust
struct ParserOptions
```

Source: `src/parser/mod.rs:242`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Options that control how the [`Parser`](../operations/sqlparser.parser.Parser.md#op-b761887fc90688e669ac5e12) parses SQL text

<a id="op-9b9db2121376a5059cabba64"></a>
## clone

`function` · `sqlparser::parser::ParserOptions::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ParserOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::parser::ParserOptions", "path": "ParserOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [241, 17], "end": [241, 22], "filename": "src/parser/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/parser/mod.rs:241`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-adbc6323b432ce8bafa14697"></a>
## default

`function` · `sqlparser::parser::ParserOptions::default` · sqlparser 0.62.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::parser::ParserOptions", "path": "ParserOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [253, 1], "end": [261, 2], "filename": "src/parser/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/parser/mod.rs:254`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ae506d2c19744c61f306f08"></a>
## eq

`function` · `sqlparser::parser::ParserOptions::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ParserOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::parser::ParserOptions", "path": "ParserOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [241, 24], "end": [241, 33], "filename": "src/parser/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/parser/mod.rs:241`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-90a54aa262d81e9b05242218"></a>
## fmt

`function` · `sqlparser::parser::ParserOptions::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::parser::ParserOptions", "path": "ParserOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [241, 10], "end": [241, 15], "filename": "src/parser/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/parser/mod.rs:241`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb196418fd8731276c24cda8"></a>
## new

`function` · `sqlparser::parser::ParserOptions::new` · sqlparser 0.62.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::parser::ParserOptions", "path": "ParserOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [263, 1], "end": [291, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:265`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Create a new [`ParserOptions`](../operations/sqlparser.parser.ParserOptions.md#op-02e47302b4279575345d622b)

<a id="op-3ab7b51f2ebabcbdd973a145"></a>
## require_semicolon_stmt_delimiter

`struct_field` · `sqlparser::parser::ParserOptions::require_semicolon_stmt_delimiter` · sqlparser 0.62.0

```rust
require_semicolon_stmt_delimiter: bool
```

Source: `src/parser/mod.rs:250`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Controls if the parser expects a semi-colon token
between statements. Default is `true`.

<a id="op-21417d06a8bc3ebcfe3314c9"></a>
## trailing_commas

`struct_field` · `sqlparser::parser::ParserOptions::trailing_commas` · sqlparser 0.62.0

```rust
trailing_commas: bool
```

Source: `src/parser/mod.rs:244`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Allow trailing commas in lists (e.g. `a, b,`).

<a id="op-852c1f276c86f436d859ba2c"></a>
## unescape

`struct_field` · `sqlparser::parser::ParserOptions::unescape` · sqlparser 0.62.0

```rust
unescape: bool
```

Source: `src/parser/mod.rs:247`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Controls how literal values are unescaped. See
[`Tokenizer::with_unescape`](../operations/sqlparser.tokenizer.Tokenizer.md#op-78c268a0e0624fbf2a8204bd) for more details.

<a id="op-c290444ab83c457e8f162f7e"></a>
## with_trailing_commas

`function` · `sqlparser::parser::ParserOptions::with_trailing_commas` · sqlparser 0.62.0

```rust
fn with_trailing_commas(self, trailing_commas: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::parser::ParserOptions", "path": "ParserOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [263, 1], "end": [291, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:280`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set if trailing commas are allowed.

If this option is `false` (the default), the following SQL will
not parse. If the option is `true`, the SQL will parse.

```sql
 SELECT
  foo,
  bar,
 FROM baz
```

<a id="op-3a23bd4f68e8d7be08be210e"></a>
## with_unescape

`function` · `sqlparser::parser::ParserOptions::with_unescape` · sqlparser 0.62.0

```rust
fn with_unescape(self, unescape: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::parser::ParserOptions", "path": "ParserOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [263, 1], "end": [291, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:287`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set if literal values are unescaped. Defaults to true. See
[`Tokenizer::with_unescape`](../operations/sqlparser.tokenizer.Tokenizer.md#op-78c268a0e0624fbf2a8204bd) for more details.
