# `sqlparser::ast::value::QuoteDelimitedString`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.value.QuoteDelimitedString.json).

<a id="op-8cf05b5dadcde8fdeef370c9"></a>
## QuoteDelimitedString

`struct` · `sqlparser::ast::value::QuoteDelimitedString` · sqlparser 0.62.0

```rust
struct QuoteDelimitedString
```

Source: `src/ast/value.rs:326`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A quote delimited string literal, e.g. `Q'_abc_'`.

See [Value::QuoteDelimitedStringLiteral](../operations/sqlparser.ast.value.Value.md#op-fe645ce168ae3916ceccf75a) and/or
[Value::NationalQuoteDelimitedStringLiteral](../operations/sqlparser.ast.value.Value.md#op-182c8d7e642cac3ae52ce0f4).

<a id="op-f0735d07be3c393b0ad2e26a"></a>
## clone

`function` · `sqlparser::ast::value::QuoteDelimitedString::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> QuoteDelimitedString
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::QuoteDelimitedString", "path": "QuoteDelimitedString"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [323, 17], "end": [323, 22], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/value.rs:323`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6153cf1041982570123613f3"></a>
## cmp

`function` · `sqlparser::ast::value::QuoteDelimitedString::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &QuoteDelimitedString) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::QuoteDelimitedString", "path": "QuoteDelimitedString"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [323, 51], "end": [323, 54], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/value.rs:323`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-342d84106109602965e10a52"></a>
## deserialize

`function` · `sqlparser::ast::value::QuoteDelimitedString::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::QuoteDelimitedString", "path": "QuoteDelimitedString"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [324, 49], "end": [324, 60], "filename": "src/ast/value.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/value.rs:324`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52f846b33c74d0c129ced5f2"></a>
## end_quote

`struct_field` · `sqlparser::ast::value::QuoteDelimitedString::end_quote` · sqlparser 0.62.0

```rust
end_quote: char
```

Source: `src/ast/value.rs:332`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

the quote end character; i.e. the character _before_ the closing `'`

<a id="op-f60c7ab81b6dc5d93a26c476"></a>
## eq

`function` · `sqlparser::ast::value::QuoteDelimitedString::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &QuoteDelimitedString) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::QuoteDelimitedString", "path": "QuoteDelimitedString"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [323, 24], "end": [323, 33], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/value.rs:323`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-000234069acf96c548f23c56"></a>
## fmt

`function` · `sqlparser::ast::value::QuoteDelimitedString::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::QuoteDelimitedString", "path": "QuoteDelimitedString"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [323, 10], "end": [323, 15], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/value.rs:323`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8fce90db06747ac7130e7367"></a>
## fmt

`function` · `sqlparser::ast::value::QuoteDelimitedString::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::QuoteDelimitedString", "path": "QuoteDelimitedString"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [335, 1], "end": [339, 2], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/value.rs:336`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-051787db5fb78b4929603f87"></a>
## hash

`function` · `sqlparser::ast::value::QuoteDelimitedString::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::QuoteDelimitedString", "path": "QuoteDelimitedString"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [323, 56], "end": [323, 60], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/value.rs:323`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6333e412059d1cdd39687665"></a>
## partial_cmp

`function` · `sqlparser::ast::value::QuoteDelimitedString::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &QuoteDelimitedString) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::QuoteDelimitedString", "path": "QuoteDelimitedString"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [323, 35], "end": [323, 45], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/value.rs:323`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-622b0a5a0a2d9bed84443ecf"></a>
## serialize

`function` · `sqlparser::ast::value::QuoteDelimitedString::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::QuoteDelimitedString", "path": "QuoteDelimitedString"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [324, 38], "end": [324, 47], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/value.rs:324`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-846abb739ce9137e7b74ea03"></a>
## start_quote

`struct_field` · `sqlparser::ast::value::QuoteDelimitedString::start_quote` · sqlparser 0.62.0

```rust
start_quote: char
```

Source: `src/ast/value.rs:328`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

the quote start character; i.e. the character _after_ the opening `Q'`

<a id="op-2e9d69a40c172434754d337a"></a>
## value

`struct_field` · `sqlparser::ast::value::QuoteDelimitedString::value` · sqlparser 0.62.0

```rust
value: String
```

Source: `src/ast/value.rs:330`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

the string literal value itself

<a id="op-d0410f5083205f5f5933dd35"></a>
## visit

`function` · `sqlparser::ast::value::QuoteDelimitedString::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::QuoteDelimitedString", "path": "QuoteDelimitedString"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [325, 40], "end": [325, 45], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/value.rs:325`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dea8bd22195c9bfc50c82bda"></a>
## visit

`function` · `sqlparser::ast::value::QuoteDelimitedString::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::QuoteDelimitedString", "path": "QuoteDelimitedString"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [325, 47], "end": [325, 55], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/value.rs:325`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
